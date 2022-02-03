#![cfg_attr(not(feature = "std"), no_std)]

pub mod game;
pub mod item;

pub use game::{Game, GameInfo};
pub use item::{Item, ItemMetadata, ItemAttributes, ItemInfo};

pub mod primitives {
	pub type GameId = u32;
	pub type ItemId = u32;
}
