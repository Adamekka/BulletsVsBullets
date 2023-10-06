use bevy::{prelude::*, render::camera::ScalingMode};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera);
    }
}

fn create_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 2.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(2., 2., 2.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0., 8.0, 0.),
        ..Default::default()
    });
}
