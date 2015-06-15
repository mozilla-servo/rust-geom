// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "geom"]
#![crate_type = "rlib"]

#![feature(asm, simd, zero_one, test)]

#[macro_use]
extern crate log;
extern crate rustc_serialize;

extern crate rand;
extern crate test;
extern crate num as num_lib;

pub use matrix::Matrix4;
pub use matrix2d::Matrix2D;
pub use point::{Point2D, Point3D};
pub use rect::Rect;
pub use side_offsets::SideOffsets2D;
pub use side_offsets::SideOffsets2DSimdI32;
pub use size::Size2D;

pub mod approxeq;
pub mod length;
pub mod matrix;
pub mod matrix2d;
pub mod num;
pub mod point;
pub mod rect;
pub mod scale_factor;
pub mod side_offsets;
pub mod size;
