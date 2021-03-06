#![no_std]
#![feature(trait_alias)]
#![allow(clippy::type_complexity)]

//! A Neural Network library that does not make use of allocations or the standard library at all.
//! It does all its work on the stack.
//!
//! This has some advantages:
//!
//! ## Embedded
//! Embedded devices without any operating system are now able to run at least simple neural
//! networks.
//!
//! ## Compile-Time checks
//! Since the whole network layout needs to be known at compile time the dimensions of inputs and
//! outputs are checked.
//!
//! ## Easy to get started
//! No need for OpenCL or CUDA, it just runs on your CPU. Or basically any other CPU for that
//! matter.
//!
//! ## Optimization
//! The whole network being known to the compiler might enable some optimizations. That said the
//! library is currently not very well optimized.
//!
//!
//! Also it was a fun challenge and actually worked out :)
//!
//! Check the examples directory for some simple networks to get started.

#[macro_use]
extern crate serde_derive;

extern crate generic_array;

pub mod activation;
//pub mod convolution;
pub mod layers;
//pub mod loss;
pub mod prelude;
pub mod softmax;
