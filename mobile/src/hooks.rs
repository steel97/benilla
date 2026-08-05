use benilla_app::register_hook;
use bevy::{
    input::{
        ButtonState, InputSystems,
        keyboard::{Key, KeyboardInput, NativeKey},
        mouse::AccumulatedMouseMotion,
        touch::Touch,
    },
    prelude::*,
    window::PrimaryWindow,
};

use crate::joystick::*;

const JOYSTICK_SIZE: f32 = 150.0;

pub fn register_hooks() {
    register_hook(|app: &mut App| {
        println!("[benilla-mobile] registered mobile systems");

        // override logging (commented out, not possible to override at runtime..)
        /*println!("[benilla-mobile] override logging");
        app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            filter: "wgpu=error,naga=warn,winit=error,my_game=debug".into(),
            level: bevy::log::Level::DEBUG,
            ..default()
        }));*/

        // enable sound
        app.init_state::<UnmuteSoundState>();
        app.add_systems(PreUpdate, unmute_sound_system.after(InputSystems));

        // virtual joystick
        app.add_plugins(VirtualJoystickPlugin::<String>::default())
            .add_message::<KeyboardInput>()
            .add_systems(Startup, init_joystick)
            .add_systems(Update, update_joystick.after(InputSystems));

        // virtual input
        app.init_state::<VirtualInput>();
        app.insert_resource(TapOffset(0.0));
        app.add_systems(PreUpdate, emulate_input_system.after(InputSystems));
    });
}

// unmute
#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum UnmuteSoundState {
    Value(i32),
}

impl Default for UnmuteSoundState {
    fn default() -> Self {
        UnmuteSoundState::Value(0)
    }
}

pub fn unmute_sound_system(
    mut input: ResMut<ButtonInput<KeyCode>>,
    state: Res<State<UnmuteSoundState>>,
    mut next_state: ResMut<NextState<UnmuteSoundState>>,
) {
    let UnmuteSoundState::Value(state) = **state;
    let mut cur_state = state;

    if cur_state > 2 {
        return;
    }

    if cur_state == 0 {
        input.press(KeyCode::ControlLeft);
        input.press(KeyCode::ShiftLeft);
    }

    if cur_state > 0 {
        input.press(KeyCode::KeyM);
    }

    if cur_state > 1 {
        input.release(KeyCode::ControlLeft);
        input.release(KeyCode::ShiftLeft);
        input.release(KeyCode::KeyM);
    }

    cur_state += 1;
    next_state.set(UnmuteSoundState::Value(cur_state));
}

// joystick
fn init_joystick(mut cmd: bevy::prelude::Commands, asset_server: Res<AssetServer>) {
    create_joystick(
        &mut cmd,
        "UniqueJoystick".to_string(),
        asset_server.load("mobile/joystick/Knob.png"),
        asset_server.load("mobile/joystick/Outline.png"),
        None,
        None,
        Some(Color::srgba(1.0, 0.27, 0.0, 0.0)),
        Vec2::new(75., 75.),
        Vec2::new(JOYSTICK_SIZE, JOYSTICK_SIZE),
        Node {
            width: Val::Px(JOYSTICK_SIZE),
            height: Val::Px(JOYSTICK_SIZE),
            position_type: PositionType::Absolute,
            right: Val::Px(JOYSTICK_SIZE),
            bottom: Val::Percent(15.),
            ..default()
        },
        (JoystickInvisible, JoystickFixed),
        NoAction,
    );
}

fn update_joystick(
    mut reader: MessageReader<VirtualJoystickMessage<String>>,
    mut keyboard_writer: MessageWriter<KeyboardInput>,
) {
    for joystick in reader.read() {
        let axis = joystick.snap_axis(Some(0.3_f32));
        let x = axis.x;
        let y = axis.y;

        println!("JOYSTICK: {}x{}", x, y);

        if x < 0.0_f32 {
            keyboard_writer.write(KeyboardInput {
                key_code: KeyCode::KeyA,
                logical_key: Key::Unidentified(NativeKey::Unidentified),
                state: ButtonState::Pressed,
                text: None,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
        } else if x > 0.0_f32 {
            keyboard_writer.write(KeyboardInput {
                key_code: KeyCode::KeyD,
                logical_key: Key::Unidentified(NativeKey::Unidentified),
                state: ButtonState::Pressed,
                text: None,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
        } else {
            keyboard_writer.write(KeyboardInput {
                key_code: KeyCode::KeyA,
                logical_key: Key::Unidentified(NativeKey::Unidentified),
                state: ButtonState::Released,
                text: None,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
            keyboard_writer.write(KeyboardInput {
                key_code: KeyCode::KeyD,
                logical_key: Key::Unidentified(NativeKey::Unidentified),
                state: ButtonState::Released,
                text: None,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
        }

        if y > 0.0_f32 {
            keyboard_writer.write(KeyboardInput {
                key_code: KeyCode::KeyW,
                logical_key: Key::Unidentified(NativeKey::Unidentified),
                state: ButtonState::Pressed,
                text: None,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
        } else if y < 0.0_f32 {
            keyboard_writer.write(KeyboardInput {
                key_code: KeyCode::KeyS,
                logical_key: Key::Unidentified(NativeKey::Unidentified),
                state: ButtonState::Pressed,
                text: None,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
        } else {
            keyboard_writer.write(KeyboardInput {
                key_code: KeyCode::KeyW,
                logical_key: Key::Unidentified(NativeKey::Unidentified),
                state: ButtonState::Released,
                text: None,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
            keyboard_writer.write(KeyboardInput {
                key_code: KeyCode::KeyS,
                logical_key: Key::Unidentified(NativeKey::Unidentified),
                state: ButtonState::Released,
                text: None,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
        }
    }
}

// virtual input
#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub struct VirtualInput {
    pub right_click_track: Vec<u64>,
    pub left_click_consumed: bool,
    pub left_click_touch_id: u64,
    pub release_click_on_next_frame: bool,
}

impl Default for VirtualInput {
    fn default() -> Self {
        Self {
            right_click_track: vec![],
            left_click_consumed: false,
            left_click_touch_id: 0,
            release_click_on_next_frame: false,
        }
    }
}

#[derive(Resource)]
pub struct TapOffset(f32);

pub fn check_touch(ww: f32, wh: f32, touch: &Touch) -> bool {
    println!("touch {} and {}", touch.position().x, ww);
    let bad_x = touch.position().x > ww - JOYSTICK_SIZE - JOYSTICK_SIZE / 2.;
    let bad_y = touch.position().y > wh - JOYSTICK_SIZE - JOYSTICK_SIZE / 2.;
    if bad_x && bad_y {
        return false;
    }

    true
}

pub fn emulate_input_system(
    touches: Res<Touches>,
    mut motion: ResMut<AccumulatedMouseMotion>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_input: ResMut<ButtonInput<MouseButton>>,
    vinp_state: Res<State<VirtualInput>>,
    mut next_vinp_state: ResMut<NextState<VirtualInput>>,
    mut tap_offset: ResMut<TapOffset>,
    time: Res<Time>,
) {
    let Ok(mut window) = window.single_mut() else {
        return;
    };
    let mut cur_state = vinp_state.get().clone();

    if cur_state.release_click_on_next_frame {
        cur_state.release_click_on_next_frame = false;
        mouse_input.release(MouseButton::Left);
    }

    // if two touches pressed at the same time, simulate right click
    let mut right_touches: Vec<u64> = Vec::new();

    for touch in touches.iter() {
        if !check_touch(window.width(), window.height(), touch) {
            continue;
        }
        if touches.just_pressed(touch.id()) {
            right_touches.push(touch.id())
        }
    }

    let mut touch_consumed = cur_state.right_click_track.clone();
    let prev_len = touch_consumed.len();
    for touch in touches.iter_just_released() {
        if touches.just_released(touch.id()) {
            touch_consumed.retain(|x| *x != touch.id());
        }
    }

    cur_state.right_click_track = touch_consumed;

    if right_touches.len() >= 2 {
        mouse_input.press(MouseButton::Left);
        cur_state.right_click_track = right_touches[..2].to_vec();
    } else if prev_len > 0 {
        let consumed = cur_state.right_click_track.len() > 0;

        if !consumed {
            mouse_input.release(MouseButton::Left);
        } else {
            for touch in touches.iter() {
                if touch.id() == cur_state.right_click_track[0] {
                    // motion
                    motion.delta += touch.delta();
                    break;
                }
            }
        }
    }

    for touch in touches.iter() {
        if !check_touch(window.width(), window.height(), touch) {
            continue;
        }

        if cur_state.right_click_track.contains(&touch.id()) {
            continue;
        }

        if touches.just_pressed(touch.id()) {
            let now = time.elapsed_secs();
            if now - tap_offset.0 < 0.3 {
                tap_offset.0 = 0.0;

                cur_state.release_click_on_next_frame = true;
                mouse_input.press(MouseButton::Left);
            } else {
                window.set_cursor_position(Some(touch.position()));
                tap_offset.0 = now;
                cur_state.left_click_consumed = true;
                cur_state.left_click_touch_id = touch.id();

                mouse_input.release(MouseButton::Left);
                mouse_input.press(MouseButton::Right);
            }
        }
    }

    if cur_state.left_click_consumed {
        if let Some(touch) = touches.get_pressed(cur_state.left_click_touch_id) {
            motion.delta += touch.delta();
        }
    }

    for touch in touches.iter_just_released() {
        if touches.just_released(touch.id()) && cur_state.left_click_consumed {
            mouse_input.release(MouseButton::Right);
            cur_state.left_click_consumed = false;
        }
    }

    for touch in touches.iter_just_canceled() {
        if touches.just_canceled(touch.id()) && cur_state.left_click_consumed {
            mouse_input.release(MouseButton::Right);
            cur_state.left_click_consumed = false;
        }
    }

    next_vinp_state.set(cur_state);
    /*else {
        window.set_cursor_position(None);
    }*/
}
