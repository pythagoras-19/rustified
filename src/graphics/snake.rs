/// usage:: cargo run --bin snake

use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    let square = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: square,
        material: materials.add(Color::ORANGE),
        transform: Transform::from_xyz(
            50.0,
            20.0,
            20.0,
        ),
        ..default()
    });

    let square = Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0)));
    commands.spawn(MaterialMesh2dBundle {
        mesh: square,
        material: materials.add(Color::NAVY),
        transform: Transform::from_xyz(
            200.0,
            100.0,
            100.0,
        ),
        ..default()
    });

    let circle = Mesh2dHandle(meshes.add(Circle::new(400.0)));
    commands.spawn(MaterialMesh2dBundle {
       mesh: circle,
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(
            600.0,
            600.0,
            200.0,
        ),
        ..default()
    });

    let circle = Mesh2dHandle(meshes.add(Circle::new(400.0)));
    commands.spawn(MaterialMesh2dBundle {
        mesh: circle,
        material: materials.add(Color::ORANGE_RED),
        transform: Transform::from_xyz(
            -500.0,
            -300.0,
            -600.0,
        ),
        ..default()
    });

    let square = Mesh2dHandle(meshes.add(Rectangle::new(60.0, 60.0)));
    commands.spawn(MaterialMesh2dBundle {
        mesh: square,
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(
            -320.0,
            -320.0,
            -100.9,
        ),
        ..default()
    });
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    if let Some(cursor_position) = windows.single().cursor_position() {
        // Calculate a world position based on the cursor's position.
        if let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            gizmos.circle_2d(point, 10., Color::BLUE);
        }
    }
}
