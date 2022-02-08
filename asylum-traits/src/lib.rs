#![cfg_attr(not(feature = "std"), no_std)]

pub mod interpretation;
pub mod item;
mod proposal;
pub mod template;
pub mod types;

pub use interpretation::Interpretable;
pub use item::{Item, Properties};
pub use proposal::Proposal;
pub use template::ItemTemplate;
pub use types::*;

pub mod primitives {
	pub type ItemId = u32;
	pub type ItemTemplateId = u32;
	pub type InterpretationTypeId = u32;
	pub type InterpretationId = u32;
	pub type ProposalId = u32;
}
