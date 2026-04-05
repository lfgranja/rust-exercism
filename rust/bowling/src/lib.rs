#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

enum GameState {
    Playing { pins_allowed: u16 },
    GameOver,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            return Err(Error::GameComplete);
        }
        if pins > self.pins_allowed() {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }

        let mut total = 0;
        let mut cursor = 0;

        for _ in 0..10 {
            let r1 = *self.rolls.get(cursor).unwrap_or(&0);

            if r1 == 10 {
                let r2 = *self.rolls.get(cursor + 1).unwrap_or(&0);
                let r3 = *self.rolls.get(cursor + 2).unwrap_or(&0);

                total += 10 + r2 + r3;
                cursor += 1;
            } else {
                let r2 = *self.rolls.get(cursor + 1).unwrap_or(&0);

                if r1 + r2 == 10 {
                    let r3 = *self.rolls.get(cursor + 2).unwrap_or(&0);

                    total += 10 + r3;
                    cursor += 2;
                } else {
                    total += r1 + r2;
                    cursor += 2;
                }
            }
        }

        Some(total)
    }

    fn is_complete(&self) -> bool {
        matches!(self.get_state(), GameState::GameOver)
    }

    fn pins_allowed(&self) -> u16 {
        match self.get_state() {
            GameState::Playing { pins_allowed } => pins_allowed,
            GameState::GameOver => 0,
        }
    }

    fn get_state(&self) -> GameState {
        let mut cursor = 0;

        for frame in 1..=10 {
            if cursor == self.rolls.len() {
                return GameState::Playing { pins_allowed: 10 };
            }

            let r1 = self.rolls[cursor];

            if r1 == 10 {
                if frame == 10 {
                    if cursor + 1 == self.rolls.len() {
                        return GameState::Playing { pins_allowed: 10 };
                    }

                    let r2 = self.rolls[cursor + 1];
                    if cursor + 2 == self.rolls.len() {
                        if r2 == 10 {
                            return GameState::Playing { pins_allowed: 10 };
                        } else {
                            return GameState::Playing {
                                pins_allowed: 10 - r2,
                            };
                        }
                    }
                    return GameState::GameOver;
                }

                cursor += 1;
            } else {
                if cursor + 1 == self.rolls.len() {
                    return GameState::Playing {
                        pins_allowed: 10 - r1,
                    };
                }

                let r2 = self.rolls[cursor + 1];

                if frame == 10 {
                    if r1 + r2 == 10 {
                        if cursor + 2 == self.rolls.len() {
                            return GameState::Playing { pins_allowed: 10 };
                        }
                        return GameState::GameOver;
                    }
                    return GameState::GameOver;
                }
                cursor += 2;
            }
        }
        GameState::GameOver
    }
}
