use crate::{camera::MovesWithCamera, input::Input};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_player)
            .add_systems(Update, move_entities);
    }
}

fn create_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.1,
                depth: 0.1,
                ..default()
            })),
            material: materials.add(Color::rgb(0.6, 0.9, 0.7).into()),
            transform: Transform::from_xyz(0., 1.1, 0.),
            ..default()
        })
        .insert(MovesWithCamera);
}

fn move_entities(
    input: Res<Input>,
    mut moves_with_camera_query: Query<(&mut Transform, &MovesWithCamera)>,
) {
    if input.is_empty() {
        return;
    }

    for (mut transform, _) in moves_with_camera_query.iter_mut() {
        if let Some(movement) = input.movement {
            transform.translation.x += movement.x * 0.05;
            transform.translation.z -= movement.y * 0.05;
        }
    }
}
