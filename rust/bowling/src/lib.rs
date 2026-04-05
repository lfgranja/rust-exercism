//! rust/bowling/src/lib.rs
/*
Instructions
Score a bowling game.

Bowling is a game where players roll a heavy ball to knock down pins arranged in a triangle. Write code to keep track of the score of a game of bowling.

Scoring Bowling
The game consists of 10 frames. A frame is composed of one or two ball throws with 10 pins standing at frame initialization. There are three cases for the tabulation of a frame.

An open frame is where a score of less than 10 is recorded for the frame. In this case the score for the frame is the number of pins knocked down.

A spare is where all ten pins are knocked down by the second throw. The total value of a spare is 10 plus the number of pins knocked down in their next throw.

A strike is where all ten pins are knocked down by the first throw. The total value of a strike is 10 plus the number of pins knocked down in the next two throws. If a strike is immediately followed by a second strike, then the value of the first strike cannot be determined until the ball is thrown one more time.

Here is a three frame example:

Frame 1	Frame 2	Frame 3
X (strike)	5/ (spare)	9 0 (open frame)
Frame 1 is (10 + 5 + 5) = 20

Frame 2 is (5 + 5 + 9) = 19

Frame 3 is (9 + 0) = 9

This means the current running total is 48.

The tenth frame in the game is a special case. If someone throws a spare or a strike then they get one or two fill balls respectively. Fill balls exist to calculate the total of the 10th frame. Scoring a strike or spare on the fill ball does not give the player more fill balls. The total value of the 10th frame is the total number of pins knocked down.

For a tenth frame of X1/ (strike and a spare), the total value is 20.

For a tenth frame of XXX (three strikes), the total value is 30.

Requirements
Write code to keep track of the score of a game of bowling. It should support two operations:

roll(pins : int) is called each time the player rolls a ball. The argument is the number of pins knocked down.
score() : int is called only at the very end of the game. It returns the total score for that game.
Source
The Bowling Game Kata from UncleBob
 */

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
