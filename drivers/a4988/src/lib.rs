//! A4988 Driver
//!
//! Platform-agnostic driver library for the A4988 stepper motor driver.
//! This crate is a specialized facade of the [Step/Dir] library. Please
//! consider using Step/Dir directly, as it provides drivers for more stepper
//! motor drivers, as well as an interface to abstract over them.
//!
//! See [Step/Dir] for more documentation and usage examples.
//!
//! [Step/Dir]: https://crates.io/crates/step-dir

#![no_std]
#![deny(missing_docs)]

pub use step_dir::{a4988::*, *};
