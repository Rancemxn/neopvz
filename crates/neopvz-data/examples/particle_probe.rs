//! Local probe: decodes every compiled particle definition in an external
//! 1.0.0.1051 resource tree and reports emitter, image, field, and node totals.
use std::{env, fs, path::PathBuf};

use neopvz_data::CompiledDefinition;

fn main() {
    let root = PathBuf::from(
        env::args()
            .nth(1)
            .expect("usage: particle_probe <data-dir>"),
    )
    .join("compiled/particles");
    let detail = env::args().nth(2);
    let mut files: Vec<PathBuf> = fs::read_dir(&root)
        .expect("particle directory")
        .map(|entry| entry.expect("entry").path())
        .collect();
    files.sort();

    let mut ok = 0usize;
    let mut trails = 0usize;
    let mut failed = 0usize;
    let mut emitters = 0usize;
    let mut images = 0usize;
    let mut fields = 0usize;
    let mut nodes = 0usize;

    for path in files {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let bytes = fs::read(&path).expect("read");
        let decoded = match CompiledDefinition::decode(&bytes) {
            Ok(decoded) => decoded,
            Err(error) => {
                println!("COMPILED-FAIL {name}: {error}");
                failed += 1;
                continue;
            }
        };
        if name.ends_with(".trail.compiled") {
            match decoded.trail() {
                Ok(trail) => {
                    trails += 1;
                    println!(
                        "TRAIL {name}: image={:?} max_points={} min_distance={} flags={} \
                         duration={} width_len={} width_time={} alpha_len={} alpha_time={}",
                        trail.image,
                        trail.max_points,
                        trail.min_point_distance,
                        trail.trail_flags,
                        trail.trail_duration.nodes.len(),
                        trail.width_over_length.nodes.len(),
                        trail.width_over_time.nodes.len(),
                        trail.alpha_over_length.nodes.len(),
                        trail.alpha_over_time.nodes.len(),
                    );
                }
                Err(error) => {
                    println!("TRAIL-FAIL {name}: {error}");
                    failed += 1;
                }
            }
            continue;
        }

        match decoded.particles() {
            Ok(definition) => {
                if detail
                    .as_ref()
                    .is_some_and(|wanted| name.eq_ignore_ascii_case(wanted))
                {
                    println!("{definition:#?}");
                }
                ok += 1;
                emitters += definition.emitters.len();
                for emitter in &definition.emitters {
                    images += usize::from(emitter.image.is_some());
                    fields += emitter.particle_fields.len() + emitter.system_fields.len();
                    nodes += emitter.spawn_rate.nodes.len()
                        + emitter.particle_duration.nodes.len()
                        + emitter.launch_speed.nodes.len()
                        + emitter.launch_angle.nodes.len()
                        + emitter.particle_scale.nodes.len()
                        + emitter.particle_alpha.nodes.len();
                }
            }
            Err(error) => {
                println!("PARTICLE-FAIL {name}: {error}");
                failed += 1;
            }
        }
    }

    println!(
        "particles={ok} trails={trails} failed={failed} emitters={emitters} images={images} fields={fields} sampled_nodes={nodes}"
    );
}
