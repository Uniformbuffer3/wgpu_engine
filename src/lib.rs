#![allow(dead_code)]

mod engine;

pub mod common;
pub use common::*;

pub mod entity_manager;
pub use entity_manager::*;

pub mod utils;
pub use utils::*;

pub use engine::*;

#[cfg(feature = "wgpu_custom")]
pub use wgpu_custom as wgpu;
#[cfg(feature = "wgpu_standard")]
pub use wgpu_standard as wgpu;

#[cfg(test)]
pub mod tests;
