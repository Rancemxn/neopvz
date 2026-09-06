//! Particle runtime mirroring the source `TodParticleSystem` behavior.
//!
//! The simulation crate owns gameplay effect anchors; this module owns the
//! presentation-side emitters those anchors start. Update order, field
//! semantics, spawn accounting, and render parameters follow the local
//! `TodParticle.cpp` and `Definition.cpp` evidence for 1.0.0.1051.
//!
//! Cross-fading emitters are skipped at initial spawn and created only when a
//! live system requests the named transition, matching the source lifecycle.

use std::{collections::HashMap, f32::consts::PI};

use neopvz_core::{LOGICAL_HEIGHT, LOGICAL_WIDTH};
use neopvz_data::{EmitterDefinition, ParameterTrack, ParticleDefinition, ParticleField};

use crate::{AffineSpriteCommand, AffineSpriteSource, BlendMode};

/// Source `ParticleFlags` bit positions, per `gParticleFlagSymbols`.
mod flags {
    pub const RANDOM_LAUNCH_SPIN: i32 = 0;
    pub const ALIGN_LAUNCH_SPIN: i32 = 1;
    pub const ALIGN_TO_PIXELS: i32 = 2;
    pub const SYSTEM_LOOPS: i32 = 3;
    pub const PARTICLE_LOOPS: i32 = 4;
    pub const PARTICLES_DONT_FOLLOW: i32 = 5;
    pub const RANDOM_START_TIME: i32 = 6;
    pub const DIE_IF_OVERLOADED: i32 = 7;
    pub const ADDITIVE: i32 = 8;
    pub const FULLSCREEN: i32 = 9;
}

/// Source `EmitterType` values.
const EMITTER_CIRCLE: i32 = 0;
const EMITTER_BOX: i32 = 1;
const EMITTER_BOX_PATH: i32 = 2;
const EMITTER_CIRCLE_PATH: i32 = 3;
const EMITTER_CIRCLE_EVEN_SPACING: i32 = 4;

/// Source `ParticleFieldType` values.
const FIELD_FRICTION: i32 = 1;
const FIELD_ACCELERATION: i32 = 2;
const FIELD_ATTRACTOR: i32 = 3;
const FIELD_MAX_VELOCITY: i32 = 4;
const FIELD_VELOCITY: i32 = 5;
const FIELD_POSITION: i32 = 6;
const FIELD_SYSTEM_POSITION: i32 = 7;
const FIELD_GROUND_CONSTRAINT: i32 = 8;
const FIELD_SHAKE: i32 = 9;
const FIELD_CIRCLE: i32 = 10;
const FIELD_AWAY: i32 = 11;

/// Source holder arrays are each allocated with 1024 entries.
const HOLDER_CAPACITY: usize = 1024;
/// Maximum number of simultaneously allocated particles.
pub const MAX_PARTICLES: usize = HOLDER_CAPACITY;
/// Source `MAX_PARTICLES_SIZE`: the overload threshold for systems, emitters,
/// and particles. Allocation remains possible up to `HOLDER_CAPACITY`.
const OVERLOAD_THRESHOLD: usize = 900;
/// Source `MAX_PARTICLE_FIELDS`: the per-emitter field cap.
const MAX_PARTICLE_FIELDS: usize = 4;

/// Source `ParticleSystemTracks` slots.
const TRACK_EMITTER_PATH: usize = 0;
const TRACK_SPAWN_RATE: usize = 1;
const TRACK_SPAWN_MIN_ACTIVE: usize = 2;
const TRACK_SPAWN_MAX_ACTIVE: usize = 3;
const TRACK_SPAWN_MAX_LAUNCHED: usize = 4;
const TRACK_SYSTEM_RED: usize = 5;
const TRACK_SYSTEM_GREEN: usize = 6;
const TRACK_SYSTEM_BLUE: usize = 7;
const TRACK_SYSTEM_ALPHA: usize = 8;
const TRACK_SYSTEM_BRIGHTNESS: usize = 9;
const SYSTEM_TRACK_COUNT: usize = 10;

/// Source `ParticleTracks` slots.
const TRACK_PARTICLE_RED: usize = 0;
const TRACK_PARTICLE_GREEN: usize = 1;
const TRACK_PARTICLE_BLUE: usize = 2;
const TRACK_PARTICLE_ALPHA: usize = 3;
const TRACK_PARTICLE_BRIGHTNESS: usize = 4;
const TRACK_PARTICLE_SPIN_ANGLE: usize = 5;
const TRACK_PARTICLE_SPIN_SPEED: usize = 6;
const TRACK_PARTICLE_SCALE: usize = 7;
const TRACK_PARTICLE_STRETCH: usize = 8;
const TRACK_PARTICLE_COLLISION_REFLECT: usize = 9;
const TRACK_PARTICLE_COLLISION_SPIN: usize = 10;
const TRACK_PARTICLE_CLIP_TOP: usize = 11;
const TRACK_PARTICLE_CLIP_BOTTOM: usize = 12;
const TRACK_PARTICLE_CLIP_LEFT: usize = 13;
const TRACK_PARTICLE_CLIP_RIGHT: usize = 14;
const TRACK_PARTICLE_ANIMATION_RATE: usize = 15;
const PARTICLE_TRACK_COUNT: usize = 16;

fn test_bit(value: i32, bit: i32) -> bool {
    value & (1 << bit) != 0
}

/// Source `TodCurveEvaluate`. Curve 8 (`CURVE_WEAK_FAST_IN_OUT`) is retired in
/// the source enum and asserts there, so it falls back to linear here.
fn curve_evaluate(time: f32, start: f32, end: f32, curve: i32) -> f32 {
    let warped = match curve {
        0 => 0.0,                                      // CURVE_CONSTANT
        1 => time,                                     // CURVE_LINEAR
        2 => curve_quad(time),                         // CURVE_EASE_IN
        3 => curve_inv_quad(time),                     // CURVE_EASE_OUT
        4 => curve_s(curve_s(time)),                   // CURVE_EASE_IN_OUT
        5 => curve_s(time),                            // CURVE_EASE_IN_OUT_WEAK
        6 => curve_inv_quad_s(curve_inv_quad_s(time)), // CURVE_FAST_IN_OUT
        7 => curve_inv_quad_s(time),                   // CURVE_FAST_IN_OUT_WEAK
        9 => curve_bounce(time),                       // CURVE_BOUNCE
        10 => curve_quad(curve_bounce(time)),          // CURVE_BOUNCE_FAST_MIDDLE
        11 => curve_inv_quad(curve_bounce(time)),      // CURVE_BOUNCE_SLOW_MIDDLE
        12 => (2.0 * PI * time).sin(),                 // CURVE_SIN_WAVE
        13 => (2.0 * PI * curve_s(time)).sin(),        // CURVE_EASE_SIN_WAVE
        _ => time,
    };
    (end - start) * warped + start
}

fn curve_quad(time: f32) -> f32 {
    time * time
}

fn curve_inv_quad(time: f32) -> f32 {
    2.0 * time - time * time
}

fn curve_s(time: f32) -> f32 {
    3.0 * time * time - 2.0 * time * time * time
}

fn curve_inv_quad_s(time: f32) -> f32 {
    if time <= 0.5 {
        curve_inv_quad(time * 2.0) * 0.5
    } else {
        curve_quad((time - 0.5) * 2.0) * 0.5 + 0.5
    }
}

fn curve_bounce(time: f32) -> f32 {
    1.0 - (2.0 * time - 1.0).abs()
}

/// Source `FloatTrackEvaluate`.
fn track_evaluate(track: &ParameterTrack, time_value: f32, interp: f32) -> f32 {
    let nodes = &track.nodes;
    let Some(first) = nodes.first() else {
        return 0.0;
    };
    if time_value < first.time {
        return curve_evaluate(interp, first.low, first.high, first.distribution);
    }
    for index in 1..nodes.len() {
        let next = &nodes[index];
        if time_value <= next.time {
            let current = &nodes[index - 1];
            let span = next.time - current.time;
            let fraction = if span == 0.0 {
                0.0
            } else {
                (time_value - current.time) / span
            };
            let left = curve_evaluate(interp, current.low, current.high, current.distribution);
            let right = curve_evaluate(interp, next.low, next.high, next.distribution);
            return curve_evaluate(fraction, left, right, current.curve);
        }
    }
    let last = &nodes[nodes.len() - 1];
    curve_evaluate(interp, last.low, last.high, last.distribution)
}

/// Source `FloatTrackEvaluateFromLastTime`.
fn track_evaluate_from_last_time(track: &ParameterTrack, time_value: f32, interp: f32) -> f32 {
    if time_value < 0.0 {
        0.0
    } else {
        track_evaluate(track, time_value, interp)
    }
}

/// Source `FloatTrackIsSet`: set only when the first node is not a constant.
fn track_is_set(track: &ParameterTrack) -> bool {
    track.nodes.first().is_some_and(|node| node.curve != 0)
}

/// Source `FloatTrackIsConstantZero`.
fn track_is_constant_zero(track: &ParameterTrack) -> bool {
    match track.nodes.as_slice() {
        [] => true,
        [node] => node.low == 0.0 && node.high == 0.0,
        _ => false,
    }
}

/// Source `FloatTrackSetDefault`: a compiled definition stores only the tracks
/// the XML named, so unnamed tracks take their post-load default. Reproduced as
/// a read-time fallback so the decoded definition is left untouched.
fn track_or_default(track: &ParameterTrack, default: f32, time_value: f32, interp: f32) -> f32 {
    if track.nodes.is_empty() {
        default
    } else {
        track_evaluate(track, time_value, interp)
    }
}

/// Presentation RNG for particle spawn variance.
///
/// The simulation's own generator stays untouched so particle work cannot
/// perturb deterministic gameplay replays.
#[derive(Clone, Debug)]
pub struct ParticleRng {
    state: u64,
}

impl ParticleRng {
    pub fn new(seed: u64) -> Self {
        // SplitMix64: a small reproducible stream for presentation variance.
        Self {
            state: seed ^ 0x9E37_79B9_7F4A_7C15,
        }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        ((z ^ (z >> 31)) >> 32) as u32
    }

    /// Source `Sexy::Rand(float)`: a uniform value in `[0, range)`.
    fn next_f32(&mut self, range: f32) -> f32 {
        range * (self.next_u32() as f32 / 4_294_967_296.0)
    }

    /// Source `Sexy::Rand(int)`: a uniform value in `[0, range)`.
    fn next_range(&mut self, range: i32) -> i32 {
        match u32::try_from(range) {
            Ok(0) | Err(_) => 0,
            Ok(range) => (self.next_u32() % range) as i32,
        }
    }
}

/// One live particle, mirroring the source `TodParticle` fields neopvz uses.
#[derive(Clone, Debug)]
struct Particle {
    position: [f32; 2],
    velocity: [f32; 2],
    spin_position: f32,
    spin_velocity: f32,
    duration: i32,
    age: i32,
    time_value: f32,
    last_time_value: f32,
    animation_time_value: f32,
    image_frame: i32,
    interp: [f32; PARTICLE_TRACK_COUNT],
    field_interp: [[f32; 2]; MAX_PARTICLE_FIELDS],
    cross_fade: Option<ParticleCrossFade>,
    /// Stand-in for the source shake `srand(age * (int)pointer)` seed, which
    /// depends on allocation addresses; a per-particle id keeps the same
    /// per-frame independent offset without that dependency.
    shake_seed: u32,
}

#[derive(Clone, Debug)]
struct ParticleCrossFade {
    duration: i32,
    target: Box<Particle>,
}

/// One live emitter belonging to a system.
#[derive(Clone, Debug)]
struct Emitter {
    definition_index: usize,
    center: [f32; 2],
    system_age: i32,
    system_duration: i32,
    system_time_value: f32,
    system_last_time_value: f32,
    spawn_accumulator: f32,
    particles_launched: i32,
    cross_fade_emitter: Option<usize>,
    cross_fade_countdown: i32,
    dead: bool,
    particles: Vec<Particle>,
    track_interp: [f32; SYSTEM_TRACK_COUNT],
    field_interp: [[f32; 2]; MAX_PARTICLE_FIELDS],
}

/// A live particle system instance.
#[derive(Clone, Debug)]
pub struct ParticleSystem {
    id: usize,
    definition: String,
    emitters: Vec<Emitter>,
    z: i32,
    image_override: Option<String>,
    frame_override: Option<i32>,
}

impl ParticleSystem {
    pub fn definition(&self) -> &str {
        &self.definition
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn particle_count(&self) -> usize {
        self.emitters
            .iter()
            .flat_map(|emitter| &emitter.particles)
            .map(|particle| 1 + usize::from(particle.cross_fade.is_some()))
            .sum()
    }

    /// A system is finished once every emitter died and drained.
    fn is_finished(&self) -> bool {
        self.emitters
            .iter()
            .all(|emitter| emitter.dead && emitter.particles.is_empty())
    }
}

/// Rendered state of one particle. Color channels are the source 0-255 scale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParticleRenderParams {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub stretch: f32,
    pub spin: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    /// Column within the emitter's image strip, already offset by `ImageCol`.
    pub frame: i32,
    pub image_row: i32,
    pub image_frames: i32,
    pub clip_top: f32,
    pub clip_bottom: f32,
    pub clip_left: f32,
    pub clip_right: f32,
    pub additive: bool,
    pub fullscreen: bool,
}

/// One particle ready to draw: its emitter image symbol plus render params.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderedParticle<'a> {
    pub image: &'a str,
    pub z: i32,
    pub params: ParticleRenderParams,
}

/// The loaded particle definitions, keyed by resource stem (for example
/// `"Doom"` for `compiled/particles/Doom.xml.compiled`).
#[derive(Clone, Debug, Default)]
pub struct ParticleCatalog {
    definitions: HashMap<String, ParticleDefinition>,
}

impl ParticleCatalog {
    pub fn insert(&mut self, name: impl Into<String>, definition: ParticleDefinition) {
        self.definitions.insert(name.into(), definition);
    }

    pub fn get(&self, name: &str) -> Option<&ParticleDefinition> {
        self.definitions.get(name)
    }

    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    fn emitter(&self, name: &str, index: usize) -> Option<&EmitterDefinition> {
        self.definitions.get(name)?.emitters.get(index)
    }
}

/// The source `TodParticleHolder`: owns live systems and the shared budget.
#[derive(Clone, Debug)]
pub struct ParticleHolder {
    systems: Vec<ParticleSystem>,
    next_system_id: usize,
    rng: ParticleRng,
    next_shake_seed: u32,
}

impl ParticleHolder {
    pub fn new(seed: u64) -> Self {
        Self {
            systems: Vec::new(),
            next_system_id: 0,
            rng: ParticleRng::new(seed),
            next_shake_seed: 1,
        }
    }

    pub fn systems(&self) -> &[ParticleSystem] {
        &self.systems
    }

    pub fn system_count(&self) -> usize {
        self.systems.len()
    }

    pub fn particle_count(&self) -> usize {
        self.systems
            .iter()
            .map(ParticleSystem::particle_count)
            .sum()
    }

    fn emitter_count(&self) -> usize {
        self.systems
            .iter()
            .map(|system| system.emitters.len())
            .sum()
    }

    pub fn clear(&mut self) {
        self.systems.clear();
    }

    /// Source `TodParticleSystem::ParticleSystemDie`: stop and remove a live system.
    pub fn die_system(&mut self, id: usize) {
        self.systems.retain(|system| system.id != id);
    }

    pub fn override_system_image(&mut self, id: usize, image: impl Into<String>) {
        if let Some(system) = self.systems.iter_mut().find(|system| system.id == id) {
            system.image_override = Some(image.into());
        }
    }

    pub fn override_system_frame(&mut self, id: usize, frame: i32) {
        if let Some(system) = self.systems.iter_mut().find(|system| system.id == id) {
            system.frame_override = Some(frame);
        }
    }

    /// Source `TodParticleHolder::IsOverLoaded`.
    fn is_overloaded(&self) -> bool {
        self.system_count() > OVERLOAD_THRESHOLD
            || self.emitter_count() > OVERLOAD_THRESHOLD
            || self.particle_count() > OVERLOAD_THRESHOLD
    }

    /// Source `TodParticleSystem::SystemMove`: recenters live emitters and
    /// carries already-launched particles unless they opt out.
    pub fn move_system(&mut self, catalog: &ParticleCatalog, id: usize, x: f32, y: f32) {
        let Some(system) = self.systems.iter_mut().find(|system| system.id == id) else {
            return;
        };
        for emitter in &mut system.emitters {
            let offset = [x - emitter.center[0], y - emitter.center[1]];
            emitter.center = [x, y];
            let dont_follow = catalog
                .emitter(&system.definition, emitter.definition_index)
                .is_some_and(|definition| {
                    test_bit(definition.particle_flags, flags::PARTICLES_DONT_FOLLOW)
                });
            if dont_follow {
                continue;
            }
            for particle in &mut emitter.particles {
                particle.position[0] += offset[0];
                particle.position[1] += offset[1];
            }
        }
    }

    /// Source `TodParticleSystem::CrossFade`: pairs every live particle with a
    /// particle from the named cross-fade emitter.
    pub fn cross_fade_system(&mut self, catalog: &ParticleCatalog, id: usize, name: &str) {
        let Some(system_index) = self.systems.iter().position(|system| system.id == id) else {
            return;
        };
        if self
            .emitter_count()
            .saturating_add(self.systems[system_index].emitters.len())
            > HOLDER_CAPACITY
        {
            self.die_system(id);
            return;
        }
        let definition_name = self.systems[system_index].definition.clone();
        let Some((target_definition_index, target_definition)) = catalog
            .get(&definition_name)
            .and_then(|definition| {
                definition
                    .emitters
                    .iter()
                    .enumerate()
                    .find(|(_, emitter)| emitter.name.eq_ignore_ascii_case(name))
            })
            .map(|(index, definition)| (index, definition.clone()))
        else {
            return;
        };
        if !track_is_set(&target_definition.cross_fade_duration) {
            return;
        }

        let source_count = self.systems[system_index].emitters.len();
        for source_index in 0..source_count {
            if self.systems[system_index].emitters[source_index].definition_index
                == target_definition_index
                || self.systems[system_index].emitters[source_index].cross_fade_countdown > 0
            {
                continue;
            }
            let center = self.systems[system_index].emitters[source_index].center;
            let system_time = self.systems[system_index].emitters[source_index].system_time_value;
            let target_emitter = self.new_emitter(
                target_definition_index,
                &target_definition,
                center[0],
                center[1],
            );
            self.systems[system_index].emitters.push(target_emitter);
            let target_index = self.systems[system_index].emitters.len() - 1;
            self.update_emitter(catalog, system_index, target_index);

            let duration = track_evaluate(
                &target_definition.cross_fade_duration,
                system_time,
                self.rng.next_f32(1.0),
            ) as i32;
            let duration = duration.max(1);
            if !track_is_set(&target_definition.system_duration) {
                self.systems[system_index].emitters[target_index].system_duration = duration;
            }
            self.systems[system_index].emitters[source_index].cross_fade_emitter =
                Some(target_index);
            self.systems[system_index].emitters[source_index].cross_fade_countdown = duration;

            let particle_count = self.systems[system_index].emitters[source_index]
                .particles
                .len();
            for particle_index in 0..particle_count {
                self.spawn_particle(&target_definition, system_index, target_index, 0, 1);
                let Some(mut target) = self.systems[system_index].emitters[target_index]
                    .particles
                    .pop()
                else {
                    continue;
                };
                if !track_is_set(&target_definition.particle_duration) {
                    target.duration = duration;
                }
                self.systems[system_index].emitters[source_index].particles[particle_index]
                    .cross_fade = Some(ParticleCrossFade {
                    duration,
                    target: Box::new(target),
                });
            }
        }
    }

    /// Source `TodParticleHolder::AllocParticleSystem` plus
    /// `TodEmitterInitialize`: creates the non-cross-fading emitters, then runs
    /// one update per emitter. Returns the new system's stable id.
    pub fn spawn(
        &mut self,
        catalog: &ParticleCatalog,
        name: &str,
        x: f32,
        y: f32,
        z: i32,
    ) -> Option<usize> {
        let definition = catalog.get(name)?;
        if self.system_count() == HOLDER_CAPACITY
            || self
                .emitter_count()
                .saturating_add(definition.emitters.len())
                > HOLDER_CAPACITY
        {
            return None;
        }
        let system_id = self.next_system_id;
        self.systems.push(ParticleSystem {
            id: system_id,
            definition: name.to_owned(),
            emitters: Vec::new(),
            z,
            image_override: None,
            frame_override: None,
        });
        let system_index = self.systems.len() - 1;

        for (index, emitter) in definition.emitters.iter().enumerate() {
            if track_is_set(&emitter.cross_fade_duration) {
                continue;
            }
            if test_bit(emitter.particle_flags, flags::DIE_IF_OVERLOADED) && self.is_overloaded() {
                self.systems.pop();
                return None;
            }
            let live_emitter = self.new_emitter(index, emitter, x, y);
            self.systems[system_index].emitters.push(live_emitter);
            let emitter_index = self.systems[system_index].emitters.len() - 1;
            self.update_emitter(catalog, system_index, emitter_index);
        }
        if self.systems[system_index].emitters.is_empty() {
            self.systems.pop();
            return None;
        }
        self.next_system_id = self
            .next_system_id
            .checked_add(1)
            .expect("particle system id overflow");
        Some(system_id)
    }

    fn new_emitter(
        &mut self,
        definition_index: usize,
        definition: &EmitterDefinition,
        x: f32,
        y: f32,
    ) -> Emitter {
        // The target samples duration before field and track interpolation.
        let duration = if track_is_set(&definition.system_duration) {
            track_evaluate(&definition.system_duration, 0.0, self.rng.next_f32(1.0))
        } else {
            track_or_default(&definition.particle_duration, 100.0, 0.0, 1.0)
        };
        let mut field_interp = [[0.0; 2]; MAX_PARTICLE_FIELDS];
        for slot in field_interp
            .iter_mut()
            .take(definition.system_fields.len().min(MAX_PARTICLE_FIELDS))
        {
            slot[0] = self.rng.next_f32(1.0);
            slot[1] = self.rng.next_f32(1.0);
        }
        let mut track_interp = [0.0; SYSTEM_TRACK_COUNT];
        for slot in &mut track_interp {
            *slot = self.rng.next_f32(1.0);
        }

        Emitter {
            definition_index,
            center: [x, y],
            system_age: -1,
            system_duration: (duration as i32).max(1),
            system_time_value: 0.0,
            system_last_time_value: -1.0,
            spawn_accumulator: 0.0,
            particles_launched: 0,
            cross_fade_emitter: None,
            cross_fade_countdown: 0,
            dead: false,
            particles: Vec::new(),
            track_interp,
            field_interp,
        }
    }

    /// Source `TodParticleSystem::Update` over every live system.
    pub fn update(&mut self, catalog: &ParticleCatalog) {
        for system_index in 0..self.systems.len() {
            let emitter_count = self.systems[system_index].emitters.len();
            for emitter_index in 0..emitter_count {
                self.update_emitter(catalog, system_index, emitter_index);
            }
        }
        self.systems.retain(|system| !system.is_finished());
    }

    /// Source `TodParticleEmitter::Update`.
    fn update_emitter(
        &mut self,
        catalog: &ParticleCatalog,
        system_index: usize,
        emitter_index: usize,
    ) {
        let Some(system) = self.systems.get(system_index) else {
            return;
        };
        if system.emitters[emitter_index].dead {
            return;
        }
        let Some(definition) = catalog
            .emitter(
                &system.definition,
                system.emitters[emitter_index].definition_index,
            )
            .cloned()
        else {
            self.systems[system_index].emitters[emitter_index].dead = true;
            return;
        };

        let cross_fade_emitter =
            self.systems[system_index].emitters[emitter_index].cross_fade_emitter;
        let cross_fade_emitter_dead = cross_fade_emitter.is_some_and(|target_index| {
            self.systems[system_index]
                .emitters
                .get(target_index)
                .is_none_or(|emitter| emitter.dead)
        });
        let emitter = &mut self.systems[system_index].emitters[emitter_index];
        emitter.system_age += 1;
        let mut die = false;
        if emitter.system_age >= emitter.system_duration {
            if test_bit(definition.particle_flags, flags::SYSTEM_LOOPS) {
                emitter.system_age = 0;
            } else {
                emitter.system_age = emitter.system_duration - 1;
                die = true;
            }
        }
        if emitter.cross_fade_countdown > 0 {
            emitter.cross_fade_countdown -= 1;
            if emitter.cross_fade_countdown == 0 {
                die = true;
            }
        }
        if cross_fade_emitter_dead {
            die = true;
        }
        emitter.system_time_value =
            emitter.system_age as f32 / (emitter.system_duration - 1).max(1) as f32;

        // Source `UpdateSystemField` runs before the particle updates.
        let time_value = emitter.system_time_value;
        let last_time_value = emitter.system_last_time_value;
        for (index, field) in definition
            .system_fields
            .iter()
            .take(MAX_PARTICLE_FIELDS)
            .enumerate()
        {
            if field.field_type != FIELD_SYSTEM_POSITION {
                continue;
            }
            let interp = emitter.field_interp[index];
            let x = track_evaluate(&field.x, time_value, interp[0]);
            let y = track_evaluate(&field.y, time_value, interp[1]);
            let last_x = track_evaluate_from_last_time(&field.x, last_time_value, interp[0]);
            let last_y = track_evaluate_from_last_time(&field.y, last_time_value, interp[1]);
            emitter.center[0] += x - last_x;
            emitter.center[1] += y - last_y;
        }

        let center = emitter.center;
        let cross_fade_target = cross_fade_emitter.and_then(|target_index| {
            let target = self.systems[system_index].emitters.get(target_index)?;
            let definition = catalog
                .emitter(
                    &self.systems[system_index].definition,
                    target.definition_index,
                )?
                .clone();
            Some((definition, target.center))
        });
        let emitter = &mut self.systems[system_index].emitters[emitter_index];
        let mut particles = std::mem::take(&mut emitter.particles);
        particles.retain_mut(|particle| {
            if !update_particle(&definition, particle, center) {
                return false;
            }
            match (&mut particle.cross_fade, &cross_fade_target) {
                (Some(cross_fade), Some((target_definition, target_center))) => {
                    update_particle(target_definition, &mut cross_fade.target, *target_center)
                }
                (Some(_), None) => false,
                (None, _) => true,
            }
        });
        self.systems[system_index].emitters[emitter_index].particles = particles;

        if cross_fade_emitter.is_none() {
            self.update_spawning(&definition, system_index, emitter_index);
        }

        let emitter = &mut self.systems[system_index].emitters[emitter_index];
        if die {
            // Source `DeleteNonCrossFading` preserves paired source particles
            // until their target particles finish.
            emitter
                .particles
                .retain(|particle| particle.cross_fade.is_some());
            if emitter.particles.is_empty() {
                emitter.dead = true;
                return;
            }
        }
        emitter.system_last_time_value = emitter.system_time_value;
    }

    /// Source `TodParticleEmitter::UpdateSpawning`.
    fn update_spawning(
        &mut self,
        definition: &EmitterDefinition,
        system_index: usize,
        emitter_index: usize,
    ) {
        let emitter = &mut self.systems[system_index].emitters[emitter_index];
        let time_value = emitter.system_time_value;
        let spawn_rate = track_or_default(
            &definition.spawn_rate,
            0.0,
            time_value,
            emitter.track_interp[TRACK_SPAWN_RATE],
        );
        emitter.spawn_accumulator += spawn_rate * 0.01;
        let mut spawn_count = emitter.spawn_accumulator as i32;
        emitter.spawn_accumulator -= spawn_count as f32;

        let active = i32::try_from(emitter.particles.len()).unwrap_or(i32::MAX);
        let min_active = track_or_default(
            &definition.spawn_min_active,
            -1.0,
            time_value,
            emitter.track_interp[TRACK_SPAWN_MIN_ACTIVE],
        ) as i32;
        if min_active >= 0 && spawn_count < min_active - active {
            spawn_count = min_active - active;
        }
        let max_active = track_or_default(
            &definition.spawn_max_active,
            -1.0,
            time_value,
            emitter.track_interp[TRACK_SPAWN_MAX_ACTIVE],
        ) as i32;
        if max_active >= 0 && spawn_count > max_active - active {
            spawn_count = max_active - active;
        }
        let max_launched = track_or_default(
            &definition.spawn_max_launched,
            -1.0,
            time_value,
            emitter.track_interp[TRACK_SPAWN_MAX_LAUNCHED],
        ) as i32;
        if max_launched >= 0 && spawn_count > max_launched - emitter.particles_launched {
            spawn_count = max_launched - emitter.particles_launched;
        }

        for index in 0..spawn_count.max(0) {
            if self.particle_count() >= MAX_PARTICLES {
                break;
            }
            self.spawn_particle(definition, system_index, emitter_index, index, spawn_count);
        }
    }

    /// Source `TodParticleEmitter::SpawnParticle`.
    fn spawn_particle(
        &mut self,
        definition: &EmitterDefinition,
        system_index: usize,
        emitter_index: usize,
        index: i32,
        spawn_count: i32,
    ) {
        let mut field_interp = [[0.0; 2]; MAX_PARTICLE_FIELDS];
        for slot in field_interp
            .iter_mut()
            .take(definition.particle_fields.len().min(MAX_PARTICLE_FIELDS))
        {
            slot[0] = self.rng.next_f32(1.0);
            slot[1] = self.rng.next_f32(1.0);
        }
        let mut interp = [0.0; PARTICLE_TRACK_COUNT];
        for slot in &mut interp {
            *slot = self.rng.next_f32(1.0);
        }

        let particle_duration_interp = self.rng.next_f32(1.0);
        let launch_speed_interp = self.rng.next_f32(1.0);
        let emitter_offset_x_interp = self.rng.next_f32(1.0);
        let emitter_offset_y_interp = self.rng.next_f32(1.0);

        let time_value = self.systems[system_index].emitters[emitter_index].system_time_value;
        let track_interp = self.systems[system_index].emitters[emitter_index].track_interp;

        let duration = track_or_default(
            &definition.particle_duration,
            100.0,
            time_value,
            particle_duration_interp,
        );
        let duration = (duration as i32).max(1);
        let age = if test_bit(definition.particle_flags, flags::RANDOM_START_TIME) {
            self.rng.next_range(duration)
        } else {
            0
        };

        let launch_speed = track_or_default(
            &definition.launch_speed,
            0.0,
            time_value,
            launch_speed_interp,
        ) * 0.01;
        let launch_angle_interp = self.rng.next_f32(1.0);
        let launch_angle_degrees = track_or_default(
            &definition.launch_angle,
            0.0,
            time_value,
            launch_angle_interp,
        );
        let launch_angle = match definition.emitter_type {
            EMITTER_CIRCLE_PATH => {
                track_evaluate(
                    &definition.emitter_path,
                    time_value,
                    track_interp[TRACK_EMITTER_PATH],
                ) * 2.0
                    * PI
                    + launch_angle_degrees.to_radians()
            }
            EMITTER_CIRCLE_EVEN_SPACING => {
                2.0 * PI * index as f32 / spawn_count.max(1) as f32
                    + launch_angle_degrees.to_radians()
            }
            // An unset launch angle on the other emitters means "any
            // direction", per the source constant-zero check.
            _ if track_is_constant_zero(&definition.launch_angle) => self.rng.next_f32(2.0 * PI),
            _ => launch_angle_degrees.to_radians(),
        };

        // The source measures angles from straight down.
        let (offset_x, offset_y) = match definition.emitter_type {
            EMITTER_CIRCLE | EMITTER_CIRCLE_PATH | EMITTER_CIRCLE_EVEN_SPACING => {
                let radius = track_or_default(
                    &definition.emitter_radius,
                    0.0,
                    time_value,
                    self.rng.next_f32(1.0),
                );
                (launch_angle.sin() * radius, launch_angle.cos() * radius)
            }
            EMITTER_BOX => {
                let box_x = track_or_default(
                    &definition.emitter_box_x,
                    0.0,
                    time_value,
                    self.rng.next_f32(1.0),
                );
                let box_y = track_or_default(
                    &definition.emitter_box_y,
                    0.0,
                    time_value,
                    self.rng.next_f32(1.0),
                );
                (box_x, box_y)
            }
            EMITTER_BOX_PATH => box_path_offset(definition, time_value, track_interp),
            _ => (0.0, 0.0),
        };

        let skew_x = track_or_default(
            &definition.emitter_skew_x,
            0.0,
            time_value,
            self.rng.next_f32(1.0),
        );
        let skew_y = track_or_default(
            &definition.emitter_skew_y,
            0.0,
            time_value,
            self.rng.next_f32(1.0),
        );
        let emitter_offset_x = track_or_default(
            &definition.emitter_offset_x,
            0.0,
            time_value,
            emitter_offset_x_interp,
        );
        let emitter_offset_y = track_or_default(
            &definition.emitter_offset_y,
            0.0,
            time_value,
            emitter_offset_y_interp,
        );

        let image_frame = if definition.animated != 0 || track_is_set(&definition.animation_rate) {
            0
        } else {
            self.rng.next_range(definition.image_frames.max(1))
        };
        let spin_position = if test_bit(definition.particle_flags, flags::RANDOM_LAUNCH_SPIN) {
            self.rng.next_f32(2.0 * PI)
        } else if test_bit(definition.particle_flags, flags::ALIGN_LAUNCH_SPIN) {
            launch_angle
        } else {
            0.0
        };
        let shake_seed = self.next_shake_seed;
        self.next_shake_seed = self.next_shake_seed.wrapping_add(1).max(1);

        let emitter = &mut self.systems[system_index].emitters[emitter_index];
        let center = emitter.center;
        let mut particle = Particle {
            position: [
                center[0] + offset_x + offset_y * skew_x + emitter_offset_x,
                center[1] + offset_y + offset_x * skew_y + emitter_offset_y,
            ],
            velocity: [
                launch_angle.sin() * launch_speed,
                launch_angle.cos() * launch_speed,
            ],
            spin_position,
            spin_velocity: 0.0,
            duration,
            age,
            time_value: 0.0,
            last_time_value: -1.0,
            animation_time_value: 0.0,
            image_frame,
            interp,
            field_interp,
            cross_fade: None,
            shake_seed,
        };

        emitter.particles_launched += 1;
        // Source `SpawnParticle` ends with one `UpdateParticle`.
        if update_particle(definition, &mut particle, center) {
            emitter.particles.push(particle);
        }
    }

    /// Collects every live particle's render state, newest systems last.
    pub fn render<'a>(&'a self, catalog: &'a ParticleCatalog) -> Vec<RenderedParticle<'a>> {
        let mut rendered = Vec::new();
        for system in &self.systems {
            for emitter in &system.emitters {
                let Some(definition) =
                    catalog.emitter(&system.definition, emitter.definition_index)
                else {
                    continue;
                };
                let Some(image) = system
                    .image_override
                    .as_deref()
                    .or(definition.image.as_deref())
                else {
                    continue;
                };
                for particle in &emitter.particles {
                    let mut params = render_params(definition, emitter, particle);
                    if let (Some(cross_fade), Some(target_index)) =
                        (&particle.cross_fade, emitter.cross_fade_emitter)
                        && let Some(target_emitter) = system.emitters.get(target_index)
                        && let Some(target_definition) =
                            catalog.emitter(&system.definition, target_emitter.definition_index)
                    {
                        let target_params =
                            render_params(target_definition, target_emitter, &cross_fade.target);
                        let fraction =
                            cross_fade.target.age as f32 / (cross_fade.duration - 1).max(1) as f32;
                        params = cross_fade_render_params(
                            params,
                            target_params,
                            definition,
                            target_definition,
                            fraction,
                        );
                    }
                    if let Some(frame) = system.frame_override {
                        params.frame = frame.clamp(0, definition.image_frames.max(1) - 1)
                            + definition.image_col;
                    }
                    // Source `DrawParticle` rounds alpha to a byte and skips 0.
                    if params.alpha.round() <= 0.0 {
                        continue;
                    }
                    rendered.push(RenderedParticle {
                        image,
                        z: system.z,
                        params,
                    });
                }
            }
        }
        rendered
    }
}

/// Source `EMITTER_BOX_PATH` perimeter walk.
fn box_path_offset(
    definition: &EmitterDefinition,
    time_value: f32,
    track_interp: [f32; SYSTEM_TRACK_COUNT],
) -> (f32, f32) {
    let path_position = track_evaluate(
        &definition.emitter_path,
        time_value,
        track_interp[TRACK_EMITTER_PATH],
    );
    let min_x = track_evaluate(&definition.emitter_box_x, time_value, 0.0);
    let max_x = track_evaluate(&definition.emitter_box_x, time_value, 1.0);
    let min_y = track_evaluate(&definition.emitter_box_y, time_value, 0.0);
    let max_y = track_evaluate(&definition.emitter_box_y, time_value, 1.0);
    let distance_x = max_x - min_x;
    let distance_y = max_y - min_y;
    let path = path_position * 2.0 * (distance_x + distance_y);

    if path < distance_y {
        (min_x, min_y + path)
    } else if path < distance_y + distance_x {
        (min_x + (path - distance_y), max_y)
    } else if path < 2.0 * distance_y + distance_x {
        (max_x, max_y - (path - distance_y - distance_x))
    } else {
        (max_x - (path - 2.0 * distance_y - distance_x), min_y)
    }
}

/// Source `TodParticleEmitter::UpdateParticle`; `false` deletes the particle.
fn update_particle(
    definition: &EmitterDefinition,
    particle: &mut Particle,
    center: [f32; 2],
) -> bool {
    if particle.age >= particle.duration {
        if test_bit(definition.particle_flags, flags::PARTICLE_LOOPS) {
            particle.age = 0;
        } else if particle.cross_fade.is_some() {
            particle.age = particle.duration - 1;
        } else {
            return false;
        }
    }

    particle.time_value = particle.age as f32 / (particle.duration - 1).max(1) as f32;
    for (index, field) in definition
        .particle_fields
        .iter()
        .take(MAX_PARTICLE_FIELDS)
        .enumerate()
    {
        update_particle_field(definition, particle, field, index, center);
    }
    particle.position[0] += particle.velocity[0];
    particle.position[1] += particle.velocity[1];

    let spin_speed = track_or_default(
        &definition.particle_spin_speed,
        0.0,
        particle.time_value,
        particle.interp[TRACK_PARTICLE_SPIN_SPEED],
    ) * 0.01;
    let spin_angle = track_or_default(
        &definition.particle_spin_angle,
        0.0,
        particle.time_value,
        particle.interp[TRACK_PARTICLE_SPIN_ANGLE],
    );
    let last_spin_angle = if particle.last_time_value < 0.0 {
        0.0
    } else {
        track_or_default(
            &definition.particle_spin_angle,
            0.0,
            particle.last_time_value,
            particle.interp[TRACK_PARTICLE_SPIN_ANGLE],
        )
    };
    particle.spin_position +=
        (spin_speed + spin_angle - last_spin_angle).to_radians() + particle.spin_velocity;

    if track_is_set(&definition.animation_rate) {
        let rate = track_evaluate(
            &definition.animation_rate,
            particle.time_value,
            particle.interp[TRACK_PARTICLE_ANIMATION_RATE],
        ) * 0.01;
        particle.animation_time_value = (particle.animation_time_value + rate).rem_euclid(1.0);
    }

    particle.age += 1;
    particle.last_time_value = particle.time_value;
    true
}

/// Source `TodParticleEmitter::UpdateParticleField`.
fn update_particle_field(
    definition: &EmitterDefinition,
    particle: &mut Particle,
    field: &ParticleField,
    field_index: usize,
    center: [f32; 2],
) {
    let interp = particle.field_interp[field_index];
    let time_value = particle.time_value;
    let x = track_evaluate(&field.x, time_value, interp[0]);
    let y = track_evaluate(&field.y, time_value, interp[1]);

    match field.field_type {
        FIELD_FRICTION => {
            particle.velocity[0] *= 1.0 - x;
            particle.velocity[1] *= 1.0 - y;
        }
        FIELD_ACCELERATION => {
            particle.velocity[0] += 0.01 * x;
            particle.velocity[1] += 0.01 * y;
        }
        FIELD_ATTRACTOR => {
            particle.velocity[0] += 0.01 * (x - (particle.position[0] - center[0]));
            particle.velocity[1] += 0.01 * (y - (particle.position[1] - center[1]));
        }
        FIELD_MAX_VELOCITY => {
            particle.velocity[0] = particle.velocity[0].clamp(-x.abs(), x.abs());
            particle.velocity[1] = particle.velocity[1].clamp(-y.abs(), y.abs());
        }
        FIELD_VELOCITY => {
            particle.position[0] += 0.01 * x;
            particle.position[1] += 0.01 * y;
        }
        FIELD_POSITION => {
            let last_x =
                track_evaluate_from_last_time(&field.x, particle.last_time_value, interp[0]);
            let last_y =
                track_evaluate_from_last_time(&field.y, particle.last_time_value, interp[1]);
            particle.position[0] += x - last_x;
            particle.position[1] += y - last_y;
        }
        FIELD_GROUND_CONSTRAINT => {
            if particle.position[1] > center[1] + y {
                particle.position[1] = center[1] + y;
                let reflect = track_or_default(
                    &definition.collision_reflect,
                    0.0,
                    time_value,
                    particle.interp[TRACK_PARTICLE_COLLISION_REFLECT],
                );
                let spin = track_or_default(
                    &definition.collision_spin,
                    0.0,
                    time_value,
                    particle.interp[TRACK_PARTICLE_COLLISION_SPIN],
                ) / 1000.0;
                particle.spin_velocity = particle.velocity[1] * spin;
                particle.velocity[0] *= reflect;
                particle.velocity[1] *= -reflect;
            }
        }
        FIELD_SHAKE => {
            let last_x =
                track_evaluate_from_last_time(&field.x, particle.last_time_value, interp[0]);
            let last_y =
                track_evaluate_from_last_time(&field.y, particle.last_time_value, interp[1]);
            // Undo the previous frame's shake, then apply this frame's.
            let previous_age = if particle.age == 0 {
                particle.duration - 1
            } else {
                particle.age - 1
            };
            let (previous_x, previous_y) = shake_offsets(particle.shake_seed, previous_age);
            particle.position[0] -= last_x * previous_x;
            particle.position[1] -= last_y * previous_y;
            let (shake_x, shake_y) = shake_offsets(particle.shake_seed, particle.age);
            particle.position[0] += x * shake_x;
            particle.position[1] += y * shake_y;
        }
        FIELD_CIRCLE => {
            let to_particle = [
                particle.position[0] - center[0],
                particle.position[1] - center[1],
            ];
            let radius = (to_particle[0] * to_particle[0] + to_particle[1] * to_particle[1]).sqrt();
            if radius > 0.0 {
                // `SexyVector2::Perp` is (-y, x), normalized by the radius.
                let scale = 0.01 * (x + radius * y) / radius;
                particle.position[0] += -to_particle[1] * scale;
                particle.position[1] += to_particle[0] * scale;
            }
        }
        FIELD_AWAY => {
            let to_particle = [
                particle.position[0] - center[0],
                particle.position[1] - center[1],
            ];
            let radius = (to_particle[0] * to_particle[0] + to_particle[1] * to_particle[1]).sqrt();
            if radius > 0.0 {
                let scale = 0.01 * (x + radius * y) / radius;
                particle.position[0] += to_particle[0] * scale;
                particle.position[1] += to_particle[1] * scale;
            }
        }
        _ => {}
    }
}

/// Deterministic stand-in for the source shake `srand`/`rand` pair. Both
/// components land in `[-1, 1]` and depend only on the particle and its age.
fn shake_offsets(seed: u32, age: i32) -> (f32, f32) {
    let mut rng = ParticleRng::new(u64::from(seed) << 32 | u64::from(age.unsigned_abs()));
    (rng.next_f32(2.0) - 1.0, rng.next_f32(2.0) - 1.0)
}

/// Source `TodParticleEmitter::GetRenderParams` and `RenderParticle`, for the
/// tracks neopvz draws. Color channels come out on the source 0-255 scale
/// because the emitter's color override defaults to `Color::White`.
fn render_params(
    definition: &EmitterDefinition,
    emitter: &Emitter,
    particle: &Particle,
) -> ParticleRenderParams {
    let system_time = emitter.system_time_value;
    let particle_time = particle.time_value;

    let system = |track: &ParameterTrack, slot: usize| {
        track_or_default(track, 1.0, system_time, emitter.track_interp[slot])
    };
    let per_particle = |track: &ParameterTrack, slot: usize| {
        track_or_default(track, 1.0, particle_time, particle.interp[slot])
    };

    let brightness = per_particle(&definition.particle_brightness, TRACK_PARTICLE_BRIGHTNESS)
        * system(&definition.system_brightness, TRACK_SYSTEM_BRIGHTNESS);
    let white = 255.0;
    let red = per_particle(&definition.particle_red, TRACK_PARTICLE_RED)
        * system(&definition.system_red, TRACK_SYSTEM_RED)
        * white
        * brightness;
    let green = per_particle(&definition.particle_green, TRACK_PARTICLE_GREEN)
        * system(&definition.system_green, TRACK_SYSTEM_GREEN)
        * white
        * brightness;
    let blue = per_particle(&definition.particle_blue, TRACK_PARTICLE_BLUE)
        * system(&definition.system_blue, TRACK_SYSTEM_BLUE)
        * white
        * brightness;
    let alpha = per_particle(&definition.particle_alpha, TRACK_PARTICLE_ALPHA)
        * system(&definition.system_alpha, TRACK_SYSTEM_ALPHA)
        * white
        * brightness;

    let scale = per_particle(&definition.particle_scale, TRACK_PARTICLE_SCALE);
    let stretch = per_particle(&definition.particle_stretch, TRACK_PARTICLE_STRETCH);
    let clip_top = track_or_default(
        &definition.clip_top,
        0.0,
        particle_time,
        particle.interp[TRACK_PARTICLE_CLIP_TOP],
    );
    let clip_bottom = track_or_default(
        &definition.clip_bottom,
        0.0,
        particle_time,
        particle.interp[TRACK_PARTICLE_CLIP_BOTTOM],
    );
    let clip_left = track_or_default(
        &definition.clip_left,
        0.0,
        particle_time,
        particle.interp[TRACK_PARTICLE_CLIP_LEFT],
    );
    let clip_right = track_or_default(
        &definition.clip_right,
        0.0,
        particle_time,
        particle.interp[TRACK_PARTICLE_CLIP_RIGHT],
    );

    let frames = definition.image_frames.max(1);
    let frame = if track_is_set(&definition.animation_rate) {
        ((particle.animation_time_value * frames as f32) as i32).clamp(0, frames - 1)
    } else if definition.animated != 0 {
        ((particle_time * frames as f32) as i32).clamp(0, frames - 1)
    } else {
        particle.image_frame
    };

    let (x, y) = if test_bit(definition.particle_flags, flags::ALIGN_TO_PIXELS) {
        (particle.position[0].round(), particle.position[1].round())
    } else {
        (particle.position[0], particle.position[1])
    };

    ParticleRenderParams {
        x,
        y,
        scale,
        stretch,
        spin: particle.spin_position,
        red,
        green,
        blue,
        alpha,
        frame: frame + definition.image_col,
        image_row: definition.image_row,
        image_frames: frames,
        clip_top,
        clip_bottom,
        clip_left,
        clip_right,
        additive: test_bit(definition.particle_flags, flags::ADDITIVE),
        fullscreen: test_bit(definition.particle_flags, flags::FULLSCREEN),
    }
}

#[derive(Clone, Copy)]
struct RenderParamSet {
    red: bool,
    green: bool,
    blue: bool,
    alpha: bool,
    scale: bool,
    stretch: bool,
    spin: bool,
    position: bool,
}

fn render_param_set(definition: &EmitterDefinition) -> RenderParamSet {
    RenderParamSet {
        red: track_is_set(&definition.system_red) || track_is_set(&definition.particle_red),
        green: track_is_set(&definition.system_green) || track_is_set(&definition.particle_green),
        blue: track_is_set(&definition.system_blue) || track_is_set(&definition.particle_blue),
        alpha: track_is_set(&definition.system_alpha) || track_is_set(&definition.particle_alpha),
        scale: track_is_set(&definition.particle_scale),
        stretch: track_is_set(&definition.particle_stretch),
        spin: track_is_set(&definition.particle_spin_speed)
            || track_is_set(&definition.particle_spin_angle)
            || test_bit(definition.particle_flags, flags::RANDOM_LAUNCH_SPIN)
            || test_bit(definition.particle_flags, flags::ALIGN_LAUNCH_SPIN),
        position: !definition.particle_fields.is_empty()
            || track_is_set(&definition.emitter_radius)
            || track_is_set(&definition.emitter_offset_x)
            || track_is_set(&definition.emitter_offset_y)
            || track_is_set(&definition.emitter_box_x)
            || track_is_set(&definition.emitter_box_y),
    }
}

fn cross_fade_lerp(from: f32, to: f32, from_set: bool, to_set: bool, fraction: f32) -> f32 {
    match (from_set, to_set) {
        (true, false) => from,
        (false, true) => to,
        _ => from + (to - from) * fraction,
    }
}

fn cross_fade_render_params(
    mut from: ParticleRenderParams,
    to: ParticleRenderParams,
    from_definition: &EmitterDefinition,
    to_definition: &EmitterDefinition,
    fraction: f32,
) -> ParticleRenderParams {
    let from_set = render_param_set(from_definition);
    let to_set = render_param_set(to_definition);
    from.red = cross_fade_lerp(from.red, to.red, from_set.red, to_set.red, fraction);
    from.green = cross_fade_lerp(from.green, to.green, from_set.green, to_set.green, fraction);
    from.blue = cross_fade_lerp(from.blue, to.blue, from_set.blue, to_set.blue, fraction);
    from.alpha = cross_fade_lerp(from.alpha, to.alpha, from_set.alpha, to_set.alpha, fraction);
    from.scale = cross_fade_lerp(from.scale, to.scale, from_set.scale, to_set.scale, fraction);
    from.stretch = cross_fade_lerp(
        from.stretch,
        to.stretch,
        from_set.stretch,
        to_set.stretch,
        fraction,
    );
    from.spin = cross_fade_lerp(from.spin, to.spin, from_set.spin, to_set.spin, fraction);
    from.x = cross_fade_lerp(from.x, to.x, from_set.position, to_set.position, fraction);
    from.y = cross_fade_lerp(from.y, to.y, from_set.position, to_set.position, fraction);
    from
}

impl ParticleRenderParams {
    /// Builds the affine sprite for this particle against a resolved texture.
    ///
    /// Source `TodScaleRotateTransformMatrix` scales x by the particle scale
    /// and y by scale times stretch.
    pub fn sprite(
        &self,
        resource_id: u32,
        z: i32,
        image_columns: u32,
        image_rows: u32,
    ) -> AffineSpriteCommand {
        let alpha = (self.alpha / 255.0).clamp(0.0, 1.0);
        let tint = [
            (self.red / 255.0).clamp(0.0, 1.0),
            (self.green / 255.0).clamp(0.0, 1.0),
            (self.blue / 255.0).clamp(0.0, 1.0),
        ];
        let blend_mode = if self.additive {
            BlendMode::Additive
        } else {
            BlendMode::Alpha
        };
        if self.fullscreen {
            return AffineSpriteCommand {
                resource_id,
                x: LOGICAL_WIDTH as f32 * 0.5,
                y: LOGICAL_HEIGHT as f32 * 0.5,
                m00: LOGICAL_WIDTH as f32,
                m01: 0.0,
                m10: 0.0,
                m11: LOGICAL_HEIGHT as f32,
                z,
                alpha,
                tint,
                blend_mode,
                source: None,
            };
        }

        let (sine, cosine) = self.spin.sin_cos();
        let scale_x = self.scale;
        let scale_y = self.scale * self.stretch;
        let columns = image_columns.max(1) as f32;
        let rows = image_rows.max(1) as f32;
        let column = self.frame.clamp(0, image_columns.max(1) as i32 - 1) as f32;
        let row = self.image_row.clamp(0, image_rows.max(1) as i32 - 1) as f32;
        let cell_size = [1.0 / columns, 1.0 / rows];
        let cell_min = [column * cell_size[0], row * cell_size[1]];
        let clip_left = self.clip_left.clamp(0.0, 1.0);
        let clip_right = self.clip_right.clamp(0.0, 1.0 - clip_left);
        let clip_top = self.clip_top.clamp(0.0, 1.0);
        let clip_bottom = self.clip_bottom.clamp(0.0, 1.0 - clip_top);
        AffineSpriteCommand {
            resource_id,
            x: self.x,
            y: self.y,
            m00: cosine * scale_x,
            m01: -sine * scale_y,
            m10: sine * scale_x,
            m11: cosine * scale_y,
            z,
            alpha,
            tint,
            blend_mode,
            source: Some(AffineSpriteSource {
                uv_min: [
                    cell_min[0] + clip_left * cell_size[0],
                    cell_min[1] + clip_top * cell_size[1],
                ],
                uv_max: [
                    cell_min[0] + (1.0 - clip_right) * cell_size[0],
                    cell_min[1] + (1.0 - clip_bottom) * cell_size[1],
                ],
                pivot_uv: [
                    cell_min[0] + 0.5 * cell_size[0],
                    cell_min[1] + 0.5 * cell_size[1],
                ],
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use neopvz_data::TrackNode;

    use super::*;

    fn track(value: f32) -> ParameterTrack {
        ParameterTrack {
            nodes: vec![TrackNode {
                time: 0.0,
                low: value,
                high: value,
                curve: 1,
                distribution: 0,
            }],
        }
    }

    fn close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.0001, "{actual} != {expected}");
    }

    #[test]
    fn particle_system_spawns_moves_renders_and_expires() {
        let mut catalog = ParticleCatalog::default();
        catalog.insert(
            "Test",
            ParticleDefinition {
                emitters: vec![EmitterDefinition {
                    image: Some("IMAGE_TEST".to_owned()),
                    image_frames: 1,
                    particle_flags: 1 << flags::ADDITIVE,
                    emitter_type: EMITTER_BOX,
                    system_duration: track(3.0),
                    spawn_rate: track(100.0),
                    particle_duration: track(4.0),
                    launch_speed: track(100.0),
                    launch_angle: track(90.0),
                    particle_red: track(0.5),
                    particle_alpha: track(0.5),
                    particle_spin_speed: track(100.0),
                    particle_scale: track(2.0),
                    particle_stretch: track(0.5),
                    clip_top: track(0.25),
                    ..EmitterDefinition::default()
                }],
            },
        );

        let mut holder = ParticleHolder::new(7);
        let system = holder.spawn(&catalog, "Test", 100.0, 200.0, 13).unwrap();
        assert_eq!(holder.system_count(), 1);
        assert_eq!(holder.particle_count(), 1);

        {
            let rendered = holder.render(&catalog);
            assert_eq!(rendered.len(), 1);
            assert_eq!(rendered[0].image, "IMAGE_TEST");
            assert_eq!(rendered[0].z, 13);
            let params = rendered[0].params;
            close(params.x, 101.0);
            close(params.y, 200.0);
            close(params.spin, 1.0_f32.to_radians());
            close(params.red, 127.5);
            close(params.green, 255.0);
            close(params.alpha, 127.5);
            close(params.clip_top, 0.25);
            close(params.clip_bottom, 0.0);

            let sprite = params.sprite(99, rendered[0].z, 4, 2);
            close(sprite.tint[0], 0.5);
            close(sprite.tint[1], 1.0);
            close(sprite.alpha, 0.5);
            assert_eq!(sprite.blend_mode, BlendMode::Additive);
            assert_eq!(
                sprite.source,
                Some(AffineSpriteSource {
                    uv_min: [0.0, 0.125],
                    uv_max: [0.25, 0.5],
                    pivot_uv: [0.125, 0.25],
                })
            );

            let fullscreen = ParticleRenderParams {
                fullscreen: true,
                ..params
            }
            .sprite(99, rendered[0].z, 4, 2);
            close(fullscreen.x, LOGICAL_WIDTH as f32 * 0.5);
            close(fullscreen.y, LOGICAL_HEIGHT as f32 * 0.5);
            close(fullscreen.m00, LOGICAL_WIDTH as f32);
            close(fullscreen.m01, 0.0);
            close(fullscreen.m10, 0.0);
            close(fullscreen.m11, LOGICAL_HEIGHT as f32);
            assert_eq!(fullscreen.source, None);
        }

        holder.move_system(&catalog, system, 110.0, 210.0);
        let moved = holder.render(&catalog)[0].params;
        close(moved.x, 111.0);
        close(moved.y, 210.0);

        holder.update(&catalog);
        assert_eq!(holder.particle_count(), 2);
        holder.update(&catalog);
        assert_eq!(holder.particle_count(), 3);
        holder.update(&catalog);
        assert_eq!(holder.system_count(), 0);
        assert_eq!(holder.particle_count(), 0);
    }

    #[test]
    fn particle_system_image_and_frame_overrides_apply_to_live_particles() {
        let mut catalog = ParticleCatalog::default();
        catalog.insert(
            "Override",
            ParticleDefinition {
                emitters: vec![EmitterDefinition {
                    image: Some("IMAGE_DEFAULT".to_owned()),
                    image_frames: 4,
                    image_col: 1,
                    system_duration: track(3.0),
                    spawn_rate: track(100.0),
                    particle_duration: track(3.0),
                    ..EmitterDefinition::default()
                }],
            },
        );
        let mut holder = ParticleHolder::new(7);
        let system = holder.spawn(&catalog, "Override", 0.0, 0.0, 0).unwrap();
        holder.override_system_image(system, "IMAGE_MUSTACHE3");
        holder.override_system_frame(system, 2);

        let rendered = holder.render(&catalog);
        assert_eq!(rendered[0].image, "IMAGE_MUSTACHE3");
        assert_eq!(rendered[0].params.frame, 3);
    }

    #[test]
    fn particle_system_ids_survive_removal_and_cross_fades_finish() {
        let definition =
            |image: &str, flags: i32, duration: f32, particle_duration: f32| ParticleDefinition {
                emitters: vec![EmitterDefinition {
                    image: Some(image.to_owned()),
                    particle_flags: flags,
                    system_duration: track(duration),
                    spawn_rate: track(100.0),
                    particle_duration: track(particle_duration),
                    ..EmitterDefinition::default()
                }],
            };
        let mut catalog = ParticleCatalog::default();
        catalog.insert("Short", definition("IMAGE_SHORT", 0, 2.0, 10.0));
        let mut trail = definition("IMAGE_TRAIL", 1 << flags::SYSTEM_LOOPS, 2.0, 10.0);
        trail.emitters.push(EmitterDefinition {
            name: "FadeOut".to_owned(),
            cross_fade_duration: track(4.0),
            particle_alpha: ParameterTrack {
                nodes: vec![
                    TrackNode {
                        time: 0.0,
                        low: 1.0,
                        high: 1.0,
                        curve: 1,
                        distribution: 1,
                    },
                    TrackNode {
                        time: 1.0,
                        low: 0.0,
                        high: 0.0,
                        curve: 1,
                        distribution: 1,
                    },
                ],
            },
            ..EmitterDefinition::default()
        });
        catalog.insert("Trail", trail);

        let mut holder = ParticleHolder::new(7);
        let short = holder.spawn(&catalog, "Short", 0.0, 0.0, 1).unwrap();
        let trail = holder.spawn(&catalog, "Trail", 0.0, 0.0, 1).unwrap();
        assert_ne!(short, trail);

        holder.update(&catalog);
        holder.update(&catalog);
        assert_eq!(holder.system_count(), 1);

        holder.move_system(&catalog, trail, 50.0, 60.0);
        let moved = holder.render(&catalog)[0].params;
        close(moved.x, 50.0);
        close(moved.y, 60.0);

        let alpha = holder.render(&catalog)[0].params.alpha;
        holder.cross_fade_system(&catalog, trail, "FadeOut");
        assert_eq!(holder.particle_count(), 6);
        holder.update(&catalog);
        assert!(holder.render(&catalog)[0].params.alpha < alpha);
        assert_eq!(holder.particle_count(), 6);
        holder.update(&catalog);
        holder.update(&catalog);
        holder.update(&catalog);
        assert_eq!(holder.system_count(), 0);
    }

    #[test]
    fn holder_uses_source_overload_threshold_and_allocation_capacity() {
        let definition = |particle_flags| ParticleDefinition {
            emitters: vec![EmitterDefinition {
                particle_flags,
                system_duration: track(10_000.0),
                ..EmitterDefinition::default()
            }],
        };
        let mut catalog = ParticleCatalog::default();
        catalog.insert("Normal", definition(0));
        catalog.insert(
            "OverloadSensitive",
            definition(1 << flags::DIE_IF_OVERLOADED),
        );

        let mut holder = ParticleHolder::new(1);
        for _ in 0..OVERLOAD_THRESHOLD - 1 {
            assert!(holder.spawn(&catalog, "Normal", 0.0, 0.0, 0).is_some());
        }
        assert!(
            holder
                .spawn(&catalog, "OverloadSensitive", 0.0, 0.0, 0)
                .is_some()
        );
        assert_eq!(holder.system_count(), OVERLOAD_THRESHOLD);
        assert!(
            holder
                .spawn(&catalog, "OverloadSensitive", 0.0, 0.0, 0)
                .is_none()
        );

        while holder.system_count() < HOLDER_CAPACITY {
            assert!(holder.spawn(&catalog, "Normal", 0.0, 0.0, 0).is_some());
        }
        assert!(holder.spawn(&catalog, "Normal", 0.0, 0.0, 0).is_none());

        catalog.insert(
            "ParticleCapacity",
            ParticleDefinition {
                emitters: vec![EmitterDefinition {
                    system_duration: track(10_000.0),
                    spawn_rate: track(200_000.0),
                    particle_duration: track(10_000.0),
                    ..EmitterDefinition::default()
                }],
            },
        );
        let mut holder = ParticleHolder::new(2);
        assert!(
            holder
                .spawn(&catalog, "ParticleCapacity", 0.0, 0.0, 0)
                .is_some()
        );
        assert_eq!(holder.particle_count(), HOLDER_CAPACITY);
    }
}
