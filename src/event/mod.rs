pub mod flags;
pub mod raw;

use flags::*;
pub use raw::*;

#[derive(Debug, PartialEq)]
pub enum Event {
    CombatLogVersion {
        version: u8,
        build: String,
    },
    ZoneChange {
        id: usize,
        name: String,
        difficulty: Difficulty,
    },
    MapChange {
        id: usize,
        name: String,
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
    },
    StaggerClear {
        guid: String,
        value: f32,
    },
    EncounterStart {
        id: usize,
        name: String,
        difficulty: Difficulty,
        size: usize,
        instance: usize,
    },
    EncounterEnd {
        id: usize,
        name: String,
        difficulty: Difficulty,
        size: usize,
        success: bool,
        length: u64,
    },
}
