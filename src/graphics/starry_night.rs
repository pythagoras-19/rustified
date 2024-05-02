use bevy:: {
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};


pub fn entry() {
    build();
}

#[derive(Component)]
struct Position(Vec2); // component

fn move_system(mut query: Query<&mut Position>) {
    for mut pos in query.iter_mut() {
        pos.0.x += 1.0;
    }
}

fn build() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}


const X_EXTENT: f32 = 900.0;

fn setup( mut commands: Commands,
          mut meshes: ResMut<Assets<Mesh>>,
          mut materials: ResMut<Assets<ColorMaterial>>, ) {

    commands.spawn(Camera2dBundle::default());

    let shapes = [
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
        Mesh2dHandle(meshes.add(Circle { radius: 50.0}))
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 100.0, 0.7);

        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        });
    }
}