use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::database::day::Day;
use crate::database::epoch::LouisEpoch;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    id: u64,
    name: String,
    days: HashMap<LouisEpoch, Day>,
}
impl User {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            days: HashMap::new(),
        }
    }
    fn get_day(&self, day: u64) -> Option<&Day> {
        self.days.get(&day)
    }
    // expects hour to be 0-23
    fn update_message_count(&mut self, day: LouisEpoch, hour: usize, messages: usize) {
        if hour > 23 {
            panic!("Hour invalid {hour} > 23")
        }
        if let Some(d) = self.days.get_mut(&day) {
            d.increment(hour, messages);
        } else {
            self.create_new_day(day).increment(hour, messages);
        };
    }
    fn create_new_day(&mut self, day: LouisEpoch) -> &mut Day {
        self.days.insert(day, Day::new_with_epoch(day));
        self.days.get_mut(&day).unwrap()
    }
    fn update_reaction_count(
        &mut self,
        day: LouisEpoch,
        hour: usize,
        reaction: &str,
        count: usize,
    ) {
        if let Some(d) = self.days.get_mut(&day) {
            d.increment_reaction(reaction, hour, count);
        } else {
            self.create_new_day(day)
                .increment_reaction(reaction, hour, count);
        }
    }
    fn combine(&self, other: Self, min: Option<usize>, max: Option<usize>) -> Self {
        // iterate over days hashmap, filter days that are in min..max and not in self.days, add those to new return new
        fn less_than(lhs: &&u64, rhs: Option<usize>) -> bool {
            rhs.map(|rhs| **lhs < rhs as u64).unwrap_or(true)
        }
        fn greater_than(lhs: &&u64, rhs: Option<usize>) -> bool {
            rhs.map(|rhs| **lhs > rhs as u64).unwrap_or(true)
        }
        let mut new = self.clone();
        other
            .days
            .iter()
            .filter(|(a, _)| {
                !(self.days.contains_key(a) && greater_than(a, min) && less_than(a, max))
            })
            .for_each(|(a, b)| {
                new.days.insert(*a, b);
            });
        new
    }
    fn sum(&self) -> usize {
        self.days.values().fold(0, |acc, d| acc + d.total())
    }
    fn sum_reactions(&self, reaction: &str) -> usize {
        self.days
            .values()
            .fold(0, |acc, d| acc + d.total_reactions_of(reaction))
    }

}
