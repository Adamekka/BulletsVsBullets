use bevy::prelude::*;
use virtual_joystick::{
    TintColor, VirtualJoystickAxis, VirtualJoystickBundle, VirtualJoystickEvent,
    VirtualJoystickEventType, VirtualJoystickInteractionArea, VirtualJoystickNode,
    VirtualJoystickPlugin, VirtualJoystickType,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VirtualJoystickPlugin::<JoySticks>::default())
            .init_resource::<Input>()
            .add_systems(Startup, spawn_joysticks)
            .add_systems(Update, get_input);
    }
}

#[derive(Clone, Debug, Default, Hash, Reflect)]
enum JoySticks {
    #[default]
    Movement,
    Combat,
}

#[derive(Debug, Default, Resource)]
struct Input {
    movement: Option<Vec2>,
    combat: Option<f32>,
}

fn spawn_joysticks(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Movement joystick
    commands
        .spawn(
            VirtualJoystickBundle::new(VirtualJoystickNode {
                id: JoySticks::Movement,
                border_image: asset_server.load("joystick/outline.png"),
                knob_image: asset_server.load("joystick/knob.png"),
                knob_size: Vec2::new(80.0, 80.0),
                dead_zone: 0.,
                axis: VirtualJoystickAxis::Both,
                behaviour: VirtualJoystickType::Floating,
            })
            .set_color(TintColor(Color::WHITE.with_a(0.2)))
            .set_style(Style {
                width: Val::Px(150.),
                height: Val::Px(150.),
                position_type: PositionType::Absolute,
                left: Val::Px(35.),
                bottom: Val::Px(15.),
                ..default()
            }),
        )
        .insert(VirtualJoystickInteractionArea);

    // Combat joystick
    commands
        .spawn(
            VirtualJoystickBundle::new(VirtualJoystickNode {
                id: JoySticks::Combat,
                border_image: asset_server.load("joystick/outline.png"),
                knob_image: asset_server.load("joystick/knob.png"),
                knob_size: Vec2::new(80.0, 80.0),
                dead_zone: 0.,
                axis: VirtualJoystickAxis::Both,
                behaviour: VirtualJoystickType::Floating,
            })
            .set_color(TintColor(Color::WHITE.with_a(0.2)))
            .set_style(Style {
                width: Val::Px(150.),
                height: Val::Px(150.),
                position_type: PositionType::Absolute,
                right: Val::Px(35.),
                bottom: Val::Px(15.),
                ..default()
            }),
        )
        .insert(VirtualJoystickInteractionArea);
}

fn get_input(
    mut joystick_inputs: EventReader<VirtualJoystickEvent<JoySticks>>,
    mut input: ResMut<Input>,
) {
    for joystick in joystick_inputs.iter() {
        let Vec2 { x, y } = joystick.axis();

        match joystick.get_type() {
            VirtualJoystickEventType::Drag => match joystick.id() {
                JoySticks::Movement => input.movement = Some(Vec2::new(x, y)),
                JoySticks::Combat => {
                    input.combat = {
                        let angle = x.atan2(y);
                        Some(angle)
                    }
                }
            },
            VirtualJoystickEventType::Up => match joystick.id() {
                JoySticks::Movement => input.movement = None,
                JoySticks::Combat => input.combat = None,
            },
            VirtualJoystickEventType::Press => (),
        }
    }
}
