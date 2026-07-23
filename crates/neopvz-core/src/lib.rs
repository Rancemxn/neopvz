use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

pub const LOGICAL_WIDTH: u32 = 800;
pub const LOGICAL_HEIGHT: u32 = 600;
pub const SIMULATION_HZ: u32 = 100;

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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InputAction {
    Pause,
    Resume,
    SelectSeed { slot: u8 },
    Plant { row: u8, column: u8 },
    Shovel { row: u8, column: u8 },
    CollectSun { entity: u32 },
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct InputFrame {
    pub actions: Vec<InputAction>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GameState {
    pub seed: u64,
    pub tick: Tick,
    pub scene: SceneKind,
    pub sun: u32,
    pub wave: u32,
    pub paused: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum GameEvent {
    Started,
    Paused,
    Resumed,
    StateChanged,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateHash(pub [u8; 32]);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Replay {
    pub seed: u64,
    pub scene: SceneKind,
    pub frames: Vec<InputFrame>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayOutcome {
    pub events: Vec<Vec<GameEvent>>,
    pub final_state: GameState,
    pub final_hash: StateHash,
}

impl Replay {
    pub fn run(&self) -> Result<ReplayOutcome, CoreError> {
        let mut game = Game::new(self.seed, self.scene);
        let events = self
            .frames
            .iter()
            .cloned()
            .map(|frame| game.advance(frame))
            .collect();
        Ok(ReplayOutcome {
            events,
            final_state: game.state.clone(),
            final_hash: game.snapshot_hash()?,
        })
    }
}

pub struct Game {
    state: GameState,
}

impl Game {
    pub fn new(seed: u64, scene: SceneKind) -> Self {
        Self {
            state: GameState {
                seed,
                tick: 0,
                scene,
                sun: 50,
                wave: 0,
                paused: false,
            },
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
    fn replay_round_trip_is_deterministic() {
        let replay = Replay {
            seed: 7,
            scene: SceneKind::Day,
            frames: vec![
                InputFrame {
                    actions: vec![InputAction::CollectSun { entity: 1 }],
                },
                InputFrame {
                    actions: vec![InputAction::Pause],
                },
                InputFrame {
                    actions: vec![InputAction::CollectSun { entity: 2 }],
                },
                InputFrame {
                    actions: vec![InputAction::Resume],
                },
            ],
        };
        let encoded = serde_json::to_vec(&replay).unwrap();
        let decoded: Replay = serde_json::from_slice(&encoded).unwrap();

        let first = replay.run().unwrap();
        let second = decoded.run().unwrap();
        assert_eq!(first, second);
        assert_eq!(first.final_state.tick, 2);
        assert_eq!(first.final_state.sun, 100);
    }

    #[test]
    fn seed_is_part_of_the_final_state_hash() {
        let frames = vec![InputFrame {
            actions: vec![InputAction::CollectSun { entity: 1 }],
        }];
        let first = Replay {
            seed: 7,
            scene: SceneKind::Day,
            frames: frames.clone(),
        }
        .run()
        .unwrap();
        let second = Replay {
            seed: 8,
            scene: SceneKind::Day,
            frames,
        }
        .run()
        .unwrap();

        assert_ne!(first.final_hash, second.final_hash);
    }
}
