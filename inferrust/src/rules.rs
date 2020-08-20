#![allow(non_snake_case)]
//! The reasoner core

mod _rules;
pub(crate) use self::_rules::*;

mod profiles;
pub use self::profiles::*;

mod alpha_rules;
pub(crate) use self::alpha_rules::*;

mod beta_rules;
pub(crate) use self::beta_rules::*;

mod delta_rules;
pub(crate) use self::delta_rules::*;

mod gamma_rules;
pub(crate) use self::gamma_rules::*;

mod same_as_rules;
pub(crate) use self::same_as_rules::*;

mod zeta_rules;
pub(crate) use self::zeta_rules::*;

mod others;
pub(crate) use self::others::*;
