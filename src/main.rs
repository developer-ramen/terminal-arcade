//! # Terminal Arcade
//!
//! Terminal Arcade is an arcade machine
//! replica-concept-reinvention-do-it-myself thingymajingy of the arcade
//! machine! The preceding sentence should have already given a solid indication
//! of the quality of this crate.
//!
//! This crate contains an interface for extending and building more games, as
//! well as a (hopefully) lot other pre-built games as well.

#![deny(unused_must_use, unused_imports, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic, missing_docs)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::module_name_repetitions)]

use crate::core::{
	Outcome,
	TerminalArcade,
};

pub mod core;
pub mod screens;

/// Entry point of a program! What, you expected a `main` function to be
/// documented well?
fn main() -> Outcome<()> {
	TerminalArcade::new().startup()
}
