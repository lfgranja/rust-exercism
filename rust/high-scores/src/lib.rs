//! rust/high-scores/src/lib.rs
/*
Instructions
Manage a game player's High Score list.

Your task is to build a high-score component of the classic Frogger game, one of the highest selling and most addictive games of all time, and a classic of the arcade era. Your task is to write methods that return the highest score from the list, the last added score and the three highest scores.

Source
Tribute to the eighties' arcade game Frogger
*/

use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self(scores)
    }

    pub fn scores(&self) -> &[u32] {
        self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.0.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.0
            .iter()
            .copied()
            .collect::<BinaryHeap<_>>() // BinaryHeap is already sorted more efficiently than using sort()
            .into_sorted_vec()
            .into_iter()
            .rev()
            .take(3)
            .collect::<Vec<_>>()
    }
}
