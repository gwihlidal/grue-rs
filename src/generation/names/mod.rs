pub mod doughty;
pub mod draconic;
pub mod elegant_evil;
pub mod empyreal;
pub mod faery_kind;
pub mod fair_noble;
pub mod family_name;
pub mod generic_fantasy;
pub mod homely;
pub mod infernal;
pub mod malevolent;
pub mod military_unit;
pub mod mystic_order;
pub mod primitive;
pub mod specific_fantasy;
pub mod tavern;
pub mod thieves_assassins;
pub mod vile_crude;

pub use doughty::*;
pub use draconic::*;
pub use elegant_evil::*;
pub use empyreal::*;
pub use faery_kind::*;
pub use fair_noble::*;
pub use family_name::*;
pub use generic_fantasy::*;
pub use homely::*;
pub use infernal::*;
pub use malevolent::*;
pub use military_unit::*;
pub use mystic_order::*;
pub use primitive::*;
pub use specific_fantasy::*;
pub use tavern::*;
pub use thieves_assassins::*;
pub use vile_crude::*;

pub fn is_vowel(c: char) -> bool {
	c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
