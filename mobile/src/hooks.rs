use benilla_app::register_hook;
use bevy::{
    input::{InputSystems, mouse::AccumulatedMouseMotion},
    prelude::*,
    window::PrimaryWindow,
};

use crate::joystick::*;

pub fn register_hooks() {
    register_hook(|app: &mut App| {
        // override logging (commented out, not possible to override at runtime..)
        /*println!("[benilla-mobile] override logging");
        app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            filter: "wgpu=error,naga=warn,winit=error,my_game=debug".into(),
            level: bevy::log::Level::DEBUG,
            ..default()
        }));*/

        // virtual joystick
        app.add_plugins(VirtualJoystickPlugin::<String>::default())
            .add_systems(Startup, init_joystick)
            .add_systems(PreUpdate, update_joystick.after(InputSystems));

        // virtual input
        println!("[benilla-mobile] registered virtual input");
        app.init_state::<VirtualInput>();
        app.add_systems(PreUpdate, emulate_input_system.after(InputSystems));
    });
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
        Some(Color::srgba(1.0, 0.27, 0.0, 0.3)),
        Vec2::new(75., 75.),
        Vec2::new(150., 150.),
        Node {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.),
            bottom: Val::Percent(15.),
            ..default()
        },
        JoystickFloating,
        NoAction,
    );
}

fn update_joystick(
    mut reader: MessageReader<VirtualJoystickMessage<String>>,
    mut input: ResMut<ButtonInput<KeyCode>>,
) {
    for joystick in reader.read() {
        let axis = joystick.snap_axis(Some(0.3_f32));
        let x = axis.x;
        let y = axis.y;

        if x < 0.0_f32 {
            input.press(KeyCode::KeyA);
        } else if x > 0.0_f32 {
            input.press(KeyCode::KeyD);
        } else {
            input.release(KeyCode::KeyA);
            input.release(KeyCode::KeyD);
        }

        if y > 0.0_f32 {
            input.press(KeyCode::KeyW);
        } else if y < 0.0_f32 {
            input.press(KeyCode::KeyS);
        } else {
            input.release(KeyCode::KeyW);
            input.release(KeyCode::KeyS);
        }
    }
}

// virtual input
#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub struct VirtualInput {
    pub right_click_track: Vec<u64>,
    pub left_click_consumed: bool,
    pub left_click_touch_id: u64,
}
impl Default for VirtualInput {
    fn default() -> Self {
        Self {
            right_click_track: vec![],
            left_click_consumed: false,
            left_click_touch_id: 0,
        }
    }
}
pub fn emulate_input_system(
    touches: Res<Touches>,
    mut motion: ResMut<AccumulatedMouseMotion>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_input: ResMut<ButtonInput<MouseButton>>,
    vinp_state: Res<State<VirtualInput>>,
    mut next_vinp_state: ResMut<NextState<VirtualInput>>,
) {
    let Ok(mut window) = window.single_mut() else {
        return;
    };
    let mut cur_state = vinp_state.get().clone();

    // if two touches pressed at the same time, simulate right click
    let mut right_touches: Vec<u64> = Vec::new();

    for touch in touches.iter() {
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
        for touch in touches.iter() {
            if touch.id() == right_touches[0] {
                window.set_cursor_position(Some(touch.position()));
                break;
            }
        }
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
        if cur_state.right_click_track.contains(&touch.id()) {
            continue;
        }

        if touches.just_pressed(touch.id()) {
            window.set_cursor_position(Some(touch.position()));
            mouse_input.press(MouseButton::Left);

            cur_state.left_click_consumed = true;
            cur_state.left_click_touch_id = touch.id();
        }
    }
    for touch in touches.iter_just_released() {
        if touches.just_released(touch.id()) && cur_state.left_click_consumed {
            mouse_input.release(MouseButton::Left);
            cur_state.left_click_consumed = false;
        }
    }

    for touch in touches.iter_just_canceled() {
        if touches.just_canceled(touch.id()) && cur_state.left_click_consumed {
            mouse_input.release(MouseButton::Left);
            cur_state.left_click_consumed = false;
        }
    }

    next_vinp_state.set(cur_state);
    /*else {
        window.set_cursor_position(None);
    }*/
}
