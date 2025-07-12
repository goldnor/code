pub use log::*;

// Rust Std usings

pub use std::rc::Rc;

// Constants

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Common Headers

pub use crate::{color::*, interval::Interval, ray::*, vec3::*};
