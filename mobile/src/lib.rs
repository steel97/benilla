//! A 3d Scene with a button and playing sound.

use bevy::prelude::bevy_main;

use crate::{
    helpers::{postprocess_env, transform_path},
    runner::run_benilla,
};

pub mod helpers;
pub mod runner;

#[bevy_main]
pub fn main() {
    unsafe {
        std::env::set_var("WOW_HOST", "192.168.1.20:3724");
        std::env::set_var("WOW_USER", "ivan");
        std::env::set_var("WOW_PASS", "changeme");

        std::env::set_var("WOW_DATA", transform_path("WoW/Data").to_str().unwrap());
        std::env::set_var("WOW_BUILTIN", transform_path("").to_str().unwrap());
    }

    postprocess_env();
    run_benilla();
}
