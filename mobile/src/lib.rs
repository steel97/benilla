//! A 3d Scene with a button and playing sound.

use benilla_app::BuildId;
use bevy::prelude::bevy_main;

use crate::{
    fs::{prepare_fs, transform_path},
    helpers::postprocess_env,
    hooks::register_hooks,
};

pub mod fs;
pub mod helpers;
pub mod hooks;
pub mod joystick;

#[bevy_main]
pub fn main() {
    println!("running benilla mobile!");

    prepare_fs();

    unsafe {
        std::env::set_var("WOW_DATA", transform_path("Data").to_str().unwrap());
        std::env::set_var("BENILLA_HOME", transform_path("").to_str().unwrap());
    }

    postprocess_env();
    register_hooks();

    benilla_app::run(BuildId {
        sha: env!("BENILLA_GIT_SHA"),
        short: env!("BENILLA_GIT_SHORT"),
        date: env!("BENILLA_GIT_DATE"),
        profile: env!("BENILLA_PROFILE"),
    });
}
