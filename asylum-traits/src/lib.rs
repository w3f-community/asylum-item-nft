#![cfg_attr(not(feature = "std"), no_std)]

pub mod game;
pub mod item;
pub mod resource;

pub use game::{Game, GameInfo};
pub use item::{Item, ItemInfo};
pub use resource::{Resource, ResourceInfo};

pub mod primitives {
	pub type GameId = u32;
	pub type ResourceId = u32;
	pub type ItemId = u32;
}
