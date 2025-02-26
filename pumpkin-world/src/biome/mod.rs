use std::sync::LazyLock;

use enum_dispatch::enum_dispatch;
use multi_noise::{BiomeEntries, SearchTree};
use pumpkin_data::chunk::Biome;
mod multi_noise;

pub static BIOME_ENTRIES: LazyLock<SearchTree<Biome>> = LazyLock::new(|| {
    SearchTree::create(
        serde_json::from_str::<BiomeEntries>(include_str!("../../../assets/multi_noise.json"))
            .expect("Could not parse synced_registries.json registry.")
            .nodes,
    )
    .expect("entries cannot be empty")
});

#[enum_dispatch]
pub trait BiomeSupplier {
    fn biome(&self, x: i32, y: i32, z: i32, noise: &MultiNoiseSampler) -> Biome;
}

#[derive(Clone)]
pub struct DebugBiomeSupplier {}

impl BiomeSupplier for DebugBiomeSupplier {
    fn biome(&self, _x: i32, _y: i32, _z: i32, _noise: &MultiNoiseSampler) -> Biome {
        Biome::Plains
    }
}

// TODO: Implement
pub struct MultiNoiseSampler {}
