//! The `benilla` launcher — a shim whose whole job is to carry `build.rs`'s git stamp.
//!
//! Everything real lives in `benilla-app`; this package exists so the build-id stamp — whose
//! watched git paths make cargo re-dirty the package on **every** commit, rebase, and checkout
//! (see `build.rs`'s header) — invalidates these few lines and a relink instead of the whole app
//! crate, its clippy pass, and its integration-test links. Decision 0993 has the measurements.

use benilla_app::BuildId;

fn main() -> benilla_app::AppExit {
    benilla_app::run(BuildId {
        sha: env!("BENILLA_GIT_SHA"),
        short: env!("BENILLA_GIT_SHORT"),
        date: env!("BENILLA_GIT_DATE"),
        profile: env!("BENILLA_PROFILE"),
    })
}

// hooks
use std::cell::RefCell;
thread_local! {
    static HOOKS: RefCell<Vec<Box<dyn Fn(&mut App) + 'static>>> = RefCell::new(Vec::new());
}

pub fn register_hook<F>(f: F)
where
    F: Fn(&mut App) + 'static,
{
    HOOKS.with(|hooks| {
        let mut guard = hooks.borrow_mut();
        guard.push(Box::new(f));
    });
}

fn execute_hooks(app: &mut App) {
    HOOKS.with(|hooks| {
        let mut guard = hooks.borrow_mut();
        for (_i, hook) in guard.iter_mut().enumerate() {
            hook(app);
        }
    });
}
