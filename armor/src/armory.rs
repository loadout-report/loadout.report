use itertools::Itertools;
use crate::model::{ArmorSet, ArmorSlot, StrippedInventoryArmor};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Armory {
    pub helmets: Vec<StrippedInventoryArmor>,
    pub gauntlets: Vec<StrippedInventoryArmor>,
    pub chests: Vec<StrippedInventoryArmor>,
    pub legs: Vec<StrippedInventoryArmor>,
    pub class_items: Vec<StrippedInventoryArmor>,
}

impl Armory {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.helmets.len()
            + self.gauntlets.len()
            + self.chests.len()
            + self.legs.len()
            + self.class_items.len()
    }

    /// partitions the armory into n chunks depending on the largest inner collection
    pub fn chunk(self, n: usize) -> Vec<Armory> {
        // if n == 1 {
        //     return Box::new(once((self).clone()));
        // }
        let (collection_to_chunk, max) = self.index_of_max();
        let chunk_size = max / n;

        match collection_to_chunk {
            0 => self
                .helmets
                .chunks(chunk_size)
                .map(|chunk| Armory {
                    helmets: chunk.to_vec(),
                    gauntlets: self.gauntlets.clone(),
                    chests: self.chests.clone(),
                    legs: self.legs.clone(),
                    class_items: self.class_items.clone(),
                })
                .collect(),
            1 => self
                .gauntlets
                .chunks(chunk_size)
                .map(|chunk| Armory {
                    helmets: self.helmets.clone(),
                    gauntlets: chunk.to_vec(),
                    chests: self.chests.clone(),
                    legs: self.legs.clone(),
                    class_items: self.class_items.clone(),
                })
                .collect(),
            2 => self
                .chests
                .chunks(chunk_size)
                .map(|chunk| Armory {
                    helmets: self.helmets.clone(),
                    gauntlets: self.gauntlets.clone(),
                    chests: chunk.to_vec(),
                    legs: self.legs.clone(),
                    class_items: self.class_items.clone(),
                })
                .collect(),
            3 => self
                .legs
                .chunks(chunk_size)
                .map(|chunk| Armory {
                    helmets: self.helmets.clone(),
                    gauntlets: self.gauntlets.clone(),
                    chests: self.chests.clone(),
                    legs: chunk.to_vec(),
                    class_items: self.class_items.clone(),
                })
                .collect(),
            _ => unreachable!(),
        }
    }

    pub fn index_of_max(&self) -> (usize, usize) {
        let sets: [usize; 4] = [
            self.helmets.len(),
            self.gauntlets.len(),
            self.chests.len(),
            self.legs.len(),
        ];
        let max = sets.iter().max().unwrap();
        (sets.iter().position(|l| l == max).unwrap(), *max)
    }
}

impl IntoIterator for Armory {
    type Item = ArmorSet;
    type IntoIter = impl Iterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.helmets.into_iter()
            .cartesian_product(self.gauntlets.into_iter())
            .cartesian_product(self.chests.into_iter())
            .cartesian_product(self.legs.into_iter())
            .map(|(((helmet, gauntlets), chest), legs)| {
                ArmorSet::new(helmet, gauntlets, chest, legs)
            })
    }
}

impl From<Vec<StrippedInventoryArmor>> for Armory {
    fn from(input: Vec<StrippedInventoryArmor>) -> Self {
        input.iter().fold(Default::default(), |mut t, i| {
            match i.slot {
                ArmorSlot::ArmorSlotNone => (),
                ArmorSlot::ArmorSlotHelmet => t.helmets.push(*i),
                ArmorSlot::ArmorSlotGauntlet => t.gauntlets.push(*i),
                ArmorSlot::ArmorSlotChest => t.chests.push(*i),
                ArmorSlot::ArmorSlotLegs => t.legs.push(*i),
                ArmorSlot::ArmorSlotClass => t.class_items.push(*i),
            }
            t
        })
    }
}
