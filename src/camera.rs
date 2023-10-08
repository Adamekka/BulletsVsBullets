use bevy::{prelude::*, render::camera::ScalingMode};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera);
    }
}

#[derive(Component)]
pub struct MovesWithCamera;

fn create_camera(mut commands: Commands) {
    // Camera
    commands
        .spawn(Camera3dBundle {
            projection: OrthographicProjection {
                scale: 2.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(0., 10., 10.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        })
        .insert(MovesWithCamera);

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0., 8.0, 0.),
        ..Default::default()
    });
}
