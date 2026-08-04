//! `benilla` — a from-scratch World of Warcraft 1.12.1 client on Bevy, talking to a local vmangos server.
//!
//! Opens the vanilla patch chain from `$WOW_DATA` (default `WoW/Data`) and streams the world around the
//! player through Bevy's `AssetServer` (the `benilla-assets` `mpq://` pipeline): ADT terrain tiles within
//! `$WOW_TILE_RADIUS` — their splat-blended ground (tiling `$WOW_TEX_TILES`), doodads/WMOs, water, and
//! ground clutter — plus the avian colliders the character controller walks on. Lit by a time-of-day WoW
//! lighting model (`Light.dbc` sampled against the server clock) with a sky dome, sun/moon discs, and
//! distance fog; a faithful `EffectGlow` bloom on top.
//!
//! In parallel a background thread ([`net`]) logs in (`$WOW_USER`/`$WOW_PASS`/`$WOW_HOST`, default
//! `one`/`pone`/`localhost`), enters the world, and streams object updates. NPCs and GameObjects
//! render as their real models (resolved from the display id via the creature/GameObject catalogs); other
//! players stay cyan cubes, and our own avatar is blue until we take third-person control of it.
//! **The world is loaded when a character enters it, and released when they leave** — the glue
//! screens have no world behind them (decision 0777). With no server there is no world: the client
//! sits at the login screen, which is what the real one does. The scene harness (`$WOW_CAPTURE`)
//! boots straight in-world and is unaffected.
//!
//! Controls: WASD walks the avatar (Ctrl sprints); right-drag turns it, left-drag orbits the camera
//! (both hide/freeze the cursor while held), scroll wheel zooms; `F` toggles free-fly (then WASD flies
//! the camera with Space up / C down, Ctrl boost).

mod area;
mod area_trigger;
mod art_scope;
mod asset_churn;
mod assets;
mod aura_visual;
mod bgwin;
mod billboard;
mod blob_shadow;
mod bowstring;
mod build_id;
mod capture;
mod char_create;
mod char_select;
mod chat_bubble;
mod clouds;
mod clutter;
mod collision;
mod combat_text;
mod cooldowns;
mod creature_anim;
mod cursor;
mod cvars;
mod dbg_trace;
mod death;
mod debug_panel;
mod decal;
mod doodad_anim;
mod entities;
mod entity_shade;
mod exterior_cull;
mod ffx_glow;
mod glue;
mod glue_strings;
mod go_anim;
mod go_templates;
mod ground_fx;
mod hover_log;
mod instance_tint;
mod interact;
mod interior;
mod items;
mod lighting;
mod liquid;
mod loading_screen;
mod local_state;
mod login;
mod map_proj;
mod mesh_tag;
mod minimap;
mod model_fade;
mod model_forms;
mod model_render;
mod modkeys;
mod nameplates;
mod names;
mod net;
mod npc_text;
mod particles;
mod pending_item_ops;
mod perf;
mod pipe_warm;
mod player;
mod portrait;
mod preflight;
mod probe_shield;
mod quest_markers;
mod raid_marks;
mod ribbons;
mod rig_palette;
mod schedule;
mod sky;
mod sky_order;
mod smart_rect;
mod sound;
mod sun;
mod target;
mod terrain;
mod terrain_stream;
mod textinput;
mod thread_qos;
mod transport;
mod ui_action;
mod ui_aura;
mod ui_bank;
mod ui_cast;
mod ui_char;
mod ui_chat;
mod ui_craft;
mod ui_duel;
mod ui_follow;
mod ui_gamma;
mod ui_gossip;
mod ui_hide;
mod ui_inspect;
mod ui_item_text;
mod ui_items;
mod ui_logout;
mod ui_loot;
mod ui_loot_roll;
mod ui_mail;
mod ui_merchant;
mod ui_mirror;
mod ui_net;
mod ui_party;
mod ui_pass;
mod ui_quest;
mod ui_quest_log;
mod ui_script;
mod ui_session;
mod ui_shapeshift;
mod ui_social;
mod ui_spellbook;
mod ui_talent;
mod ui_taxi;
mod ui_text;
mod ui_tooltip;
mod ui_trade;
mod ui_tradeskill;
mod ui_trainer;
mod ui_unit;
mod ui_world_map;
mod view;
mod vplates;
mod water_fx;
mod wdl;
mod weather;
mod wmo_portal;
mod wmo_sky;
mod world_map;
mod world_state;
mod zfill;

use benilla::main_shared;
use bevy::prelude::*;

/// Anchor the loaded terrain block on the Human start (Northshire), where `one`/`One`
/// logs in — so the player sits in the middle of the block instead of Stormwind's edge.
const SPAWN_XY: (f32, f32) = (-8949.95, -132.49);

fn main() -> AppExit {
    main_shared()
}
