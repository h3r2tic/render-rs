#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate option_set;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate downcast_rs;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate log;

pub mod allocator;
pub mod backend;
pub mod commands;
pub mod constants;
pub mod debug;
pub mod device;
pub mod encoder;
pub mod error;
pub mod format;
pub mod handles;
pub mod modules;
pub mod profile;
pub mod resources;
pub mod state;
pub mod system;
pub mod types;
pub mod utilities;

pub use crate::error::Result;

#[macro_export]
#[cfg(feature = "profiling_enabled")]
macro_rules! scoped_profile {
    ($section:ident, $name:ident) => {
        let section_name = concat!(stringify!($section), "\0");
        let profile_name = concat!(stringify!($name), "\0");
        ScopedProfile::start_region(profile_name.as_ptr() as _);
        let $name = ScopedProfile {};
    };
}

#[macro_export]
#[cfg(not(feature = "profiling_enabled"))]
macro_rules! scoped_profile {
    ($section:ident, $name:ident) => {
        ()
    };
}
