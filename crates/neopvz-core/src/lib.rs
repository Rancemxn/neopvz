use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

pub const LOGICAL_WIDTH: u32 = 800;
pub const LOGICAL_HEIGHT: u32 = 600;
pub const SIMULATION_HZ: u32 = 60;

pub type Tick = u64;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("state serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum SceneKind {
    #[default]
    Title,
    AdventureSelect,
    SeedChooser,
    Day,
    Night,
    Pool,
    Roof,
    Complete,
    GameOver,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum InputAction {
    Pause,
    Resume,
    SelectSeed { slot: u8 },
    Plant { row: u8, column: u8 },
    Shovel { row: u8, column: u8 },
    CollectSun { entity: u32 },
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InputFrame {
    pub actions: Vec<InputAction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameState {
    pub tick: Tick,
    pub scene: SceneKind,
    pub sun: u32,
    pub wave: u32,
    pub paused: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameEvent {
    Started,
    Paused,
    Resumed,
    StateChanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateHash(pub [u8; 32]);

pub struct Game {
    state: GameState,
    rng: ChaCha8Rng,
}

impl Game {
    pub fn new(seed: u64, scene: SceneKind) -> Self {
        Self {
            state: GameState {
                tick: 0,
                scene,
                sun: 50,
                wave: 0,
                paused: false,
            },
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn advance(&mut self, input: InputFrame) -> Vec<GameEvent> {
        let mut events = Vec::new();
        for action in input.actions {
            match action {
                InputAction::Pause => {
                    self.state.paused = true;
                    events.push(GameEvent::Paused);
                }
                InputAction::Resume => {
                    self.state.paused = false;
                    events.push(GameEvent::Resumed);
                }
                InputAction::CollectSun { .. } => {
                    self.state.sun = self.state.sun.saturating_add(25);
                }
                InputAction::SelectSeed { .. }
                | InputAction::Plant { .. }
                | InputAction::Shovel { .. } => {}
            }
        }

        if !self.state.paused {
            self.state.tick = self.state.tick.saturating_add(1);
            self.state.wave = (self.state.tick / SIMULATION_HZ as u64) as u32;
            let _ = self.rng.next_u32();
            events.push(GameEvent::StateChanged);
        }

        events
    }

    pub fn snapshot_hash(&self) -> Result<StateHash, CoreError> {
        let bytes = serde_json::to_vec(&self.state)?;
        let mut digest = Sha256::new();
        digest.update(bytes);
        Ok(StateHash(digest.finalize().into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_and_input_are_replayable() {
        let mut first = Game::new(7, SceneKind::Day);
        let mut second = Game::new(7, SceneKind::Day);
        for _ in 0..10 {
            let input = InputFrame {
                actions: vec![InputAction::CollectSun { entity: 1 }],
            };
            first.advance(input.clone());
            second.advance(input);
        }
        assert_eq!(
            first.snapshot_hash().unwrap(),
            second.snapshot_hash().unwrap()
        );
    }
}
