use std::{collections::HashMap, iter::repeat};

use serde_derive::{Deserialize, Serialize};

use crate::database::epoch::{LouisEpoch, epoch_to_unix, now};

#[derive(Serialize, Deserialize, Clone)]
pub struct Day {
    date: f64,
    msg_hours: Vec<u64>,
    emoji_hours: HashMap<String, Vec<u64>>,
}

impl Day {
    pub fn new(date: f64) -> Self {
        Self {
            date,
            msg_hours: generate_empty_hours(),
            emoji_hours: HashMap::new(),
        }
    }
    pub fn new_with_epoch(epoch: LouisEpoch) -> Self {
        let timestamp = epoch_to_unix(epoch).timestamp() as f64;
        Self::new(timestamp)
    }
    pub fn new_now() -> Self {
        Self::new(now())
    }
    pub fn new_from_timeof(t: LouisEpoch) -> Self {
        Self::new(epoch_to_unix(t).timestamp() as f64)
    }
    pub fn increment(&mut self, hour: usize, value: usize) {
        if hour > self.msg_hours.len() {
            panic!(
                "{hour} out of range, {hour} > {} = false",
                self.msg_hours.len()
            )
        }
        self.msg_hours[hour] += value as u64;
    }
    pub fn total(&self) -> usize {
        self.msg_hours.iter().sum::<u64>() as usize
    }
    pub fn total_reactions_of(&self, reaction: &str) -> usize {
        self.emoji_hours
            .get(reaction)
            .map(|reactions| reactions.iter().sum::<u64>() as usize)
            .unwrap_or(0)
    }
    pub fn increment_reaction(&mut self, reaction: &str, hour: usize, count: usize) {
        if hour > 23 {
            panic!("Hour invalid {hour} > 23")
        }
        if let Some(r) = self.emoji_hours.get_mut(reaction) {
            r[hour] += count as u64;
        } else {
            self.emoji_hours
                .insert(reaction.to_string(), generate_empty_hours());
            self.emoji_hours.get_mut(reaction).unwrap()[hour] += count as u64;
        }
    }
    fn get_reaction(&self, reaction: &str) -> Vec<u64> {
        self.emoji_hours
            .get(reaction)
            .map(|a| a.to_owned())
            .unwrap_or(generate_empty_hours())
    }
    fn avg_hours(&self) -> f64 {
        self.msg_hours.iter().sum::<u64>() as f64 / self.msg_hours.len() as f64
    }
    fn avg_reactions_of(&self, reaction: &str) -> f64 {
        self.emoji_hours
            .get(reaction)
            .map(|h| h.iter().sum::<u64>() as f64 / h.len() as f64)
            .unwrap_or(0.0)
    }
}
fn generate_empty_hours() -> Vec<u64> {
    repeat(0).take(24).collect()
}
