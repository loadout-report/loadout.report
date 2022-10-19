use std::ops::{Add, Index};
use serde::{Deserialize, Serialize};
use crate::model::{ModGroup, SimpleArmorStat, SimpleModifierValue};

#[derive(Copy, Clone, Serialize, Deserialize, Default)]
pub struct Stats {
    values: [u8; 6],
}

impl Stats {
    pub fn new(values: [u8; 6]) -> Self {
        Self {
            values
        }
    }

    fn modify(mut self, stat: SimpleArmorStat, val: i8) -> Self {
        let mut stats = self.values;
        match stat {
            SimpleArmorStat::Mobility => stats[0] = ((stats[0] as i8) + val) as u8,
            SimpleArmorStat::Resilience => stats[1] = ((stats[1] as i8) + val) as u8,
            SimpleArmorStat::Recovery => stats[2] = ((stats[2] as i8) + val) as u8,
            SimpleArmorStat::Discipline => stats[3] = ((stats[3] as i8) + val) as u8,
            SimpleArmorStat::Intellect => stats[4] = ((stats[4] as i8) + val) as u8,
            SimpleArmorStat::Strength => stats[5] = ((stats[5] as i8) + val) as u8,
        };
        self
    }

    pub fn modify_all(self, val: i8) -> Self {
        Self {
            values: self.values.map(|i| ((i as i8) + val) as u8)
        }
    }
}

impl Add<Stats> for Stats {
    type Output = Stats;

    fn add(self, rhs: Stats) -> Self::Output {
        Stats {
            values: self.values.zip(rhs.values).map(|(a, b)| a + b)
        }
    }
}

impl Add<StatsMod> for Stats {
    type Output = Stats;

    fn add(self, rhs: StatsMod) -> Self::Output {
        Stats {
            values: self.values.zip(rhs.values).map(|(a, b)| ((a as i8) + b) as u8)
        }
    }
}

impl Add<SimpleModifierValue> for Stats {
    type Output = Stats;

    fn add(self, rhs: SimpleModifierValue) -> Self::Output {
        self.modify(rhs.stat, rhs.value)
    }
}

impl Add<ModGroup> for Stats {
    type Output = Stats;

    fn add(self, rhs: ModGroup) -> Self::Output {
        match rhs {
            ModGroup::Single(val) => self + val,
            ModGroup::Double(v1, v2) => self + v1 + v2
        }
    }
}

impl Index<SimpleArmorStat> for Stats {
    type Output = u8;

    fn index(&self, index: SimpleArmorStat) -> &Self::Output {
        &self.values[index as usize]
    }
}

impl Index<usize> for Stats {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

/// A modifier struct for Stats. This is separate as it is represented by an array of i8 instead
/// of an array of u8 like Stats. As such it is possible to implement negative stat modifiers.
#[derive(Copy, Clone, Serialize, Deserialize, Default)]
pub struct StatsMod {
    values: [i8; 6],
}

impl StatsMod {
    pub fn modify(mut self, stat: SimpleArmorStat, val: i8) -> Self {
        let mut stats = self.values;
        match stat {
            SimpleArmorStat::Mobility => stats[0] += val,
            SimpleArmorStat::Resilience => stats[1] += val,
            SimpleArmorStat::Recovery => stats[2] += val,
            SimpleArmorStat::Discipline => stats[3] += val,
            SimpleArmorStat::Intellect => stats[4] += val,
            SimpleArmorStat::Strength => stats[5] += val,
        }
        self
    }
}

impl Add<ModGroup> for StatsMod {
    type Output = StatsMod;

    fn add(self, rhs: ModGroup) -> Self::Output {
        match rhs {
            ModGroup::Single(val) => self + val,
            ModGroup::Double(v1, v2) => self + v1 + v2
        }
    }
}

impl Add<SimpleModifierValue> for StatsMod {
    type Output = StatsMod;

    fn add(self, rhs: SimpleModifierValue) -> Self::Output {
        self.modify(rhs.stat, rhs.value)
    }
}
