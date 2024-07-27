// Copyright 2024 bmc::labs GmbH. All rights reserved.

//! We use [`Option`] extensively in Rust, and for good reason. Sometimes, however, we need to be
//! able to represent something that can either be empty, a single element or a list of elements.
//! One example would be the pull policy field in Docker configurations: you can omit it to defer
//! to the global configuration, or you can specify a single pull policy, or you can specify a
//! list of pull policies. The containers in this crate implement these semantics.

mod maybe_multiple;
mod multiple;

pub use maybe_multiple::MaybeMultiple;
pub use multiple::Multiple;
