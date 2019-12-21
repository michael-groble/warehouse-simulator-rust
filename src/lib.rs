use std::collections::HashMap;

pub mod line_member;
pub use self::line_member::LineMember;

pub mod picker;
pub use self::picker::Picker;

pub mod checker;
pub use self::checker::Checker;

pub type SimulationTime = f64;
pub type PickQuantity = i32;
pub type ItemPicks<'a> = HashMap<&'a str, PickQuantity>;
