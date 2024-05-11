/// usage:: cargo run --bin snake
/// Actually, NOT A SNAKE GAME ;)

use bevy::prelude::*;
use bevy::asset::AssetContainer;
use bevy::render::color::Color;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Monster;

#[derive(Component)]
struct Stats {
    name: String,
    health: i64
}

#[derive(Component)]
struct AquaSquare;

#[derive(Component)]
struct NavySquare;

#[derive(Component)]
struct OrangeCircle;

#[derive(Resource)]
struct Game {
    game_objects: Vec<Entity>,
}

impl Game {
    fn new() -> Self {
        Self {
            game_objects: Vec::new(),
        }
    }

    fn add(&mut self, game_object: Entity) {
        self.game_objects.push(game_object);
    }

    fn get_game_objects(&self) -> Vec<Entity> {
        return self.game_objects.clone();
    }

    fn display_objects(&self) {
        println!("Display game objects\n");
        for e in self.game_objects.clone() {
            println!("Entity ID: {:?}, Index: {}, ?: {:?}", e, e.index(),e.to_bits());
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Game::new())
        .add_systems(Startup, (setup, add_people, add_monster))
        .add_systems(Update, (draw_cursor, (update_people, greet_people, show_monster).chain()))
        .add_systems(Update, move_entities)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>, mut game: ResMut<Game>) {
    commands.spawn(Camera2dBundle::default());

    let square = Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0)));


    let square_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: square.into(),
        material: materials.add(Color::AQUAMARINE),
        transform: Transform::from_xyz(
            50.0,
            20.0,
            20.0,
        ),
        ..default()
    }).id();

    commands.entity(square_entity).insert(AquaSquare); // tag it
    game.add(square_entity);
    game.display_objects();

    let larger_square = Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0)));
    let larger_square_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: larger_square.into(),
        material: materials.add(Color::NAVY),
        transform: Transform::from_xyz(
            200.0,
            100.0,
            100.0,
        ),
        ..default()
    }).id();

    commands.entity(larger_square_entity).insert(NavySquare);
    game.add(larger_square_entity);

    let circle = Mesh2dHandle(meshes.add(Circle::new(400.0)));
    let circle_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: circle.into(),
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(
            600.0,
            600.0,
            200.0,
        ),
        ..default()
    }).id();
    game.add(circle_entity);

    let circle = Mesh2dHandle(meshes.add(Circle::new(400.0)));
    let orange_red_circle_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: circle.into(),
        material: materials.add(Color::ORANGE_RED),
        transform: Transform::from_xyz(
            -500.0,
            -300.0,
            -600.0,
        ),
        ..default()
    }).id();
    commands.entity(orange_red_circle_entity).insert(OrangeCircle);
    game.add(orange_red_circle_entity);

    let square = Mesh2dHandle(meshes.add(Rectangle::new(60.0, 60.0)));
    let green_square_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: square.into(),
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(
            -320.0,
            -320.0,
            -100.9,
        ),
        ..default()
    }).id();
    game.add(green_square_entity);

    let rectangle = Mesh2dHandle(meshes.add(Rectangle::new(160.0, 60.0)));
    let pink_rectangle_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: rectangle.into(),
        material: materials.add(Color::PINK),
        transform: Transform::from_xyz(
            420.0,
            -200.0,
            -100.5,
        ),
        ..default()
    }).id();
    game.add(pink_rectangle_entity);
    // todo: Polygon
    // todo: cursor tracking
}

fn move_entities(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, Option<&AquaSquare>, Option<&NavySquare>, Option<&OrangeCircle>)>,
    game: Res<Game>,
    windows:Query<&Window, With<PrimaryWindow>>
) {

    // TODO: make these constants
    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();

    let x_boundary = window_width / 2.0;
    let y_boundary = window_height / 2.0;

    for (entity, mut transform, aqua, navy, orange) in query.iter_mut() {
        if game.game_objects.contains(&entity) {
            if aqua.is_some() {
                println!("Aqua moving!");
                transform.translation.x += 12.0;
            }
            if navy.is_some() {
                transform.translation.x += 15.0;
            }
            if orange.is_some() {
                transform.translation.y += 2.0;
            }
            else {
                let new_x = transform.translation.x - 5.0;
                if new_x > -x_boundary && new_x < x_boundary {
                    transform.translation.x = new_x;
                }
            }
            // out of bounds checker
            transform.translation.x = transform.translation.x.min(x_boundary).max(-x_boundary);
            transform.translation.y = transform.translation.y.min(y_boundary).max(-y_boundary);
        }
    }
}

fn draw_cursor( camera_query: Query<(&Camera, &GlobalTransform)>,
                 windows: Query<&Window>, mut gizmos: Gizmos, ) {
    let (camera, camera_transform) = camera_query.single();

    if let Some(cursor_position) = windows.single().cursor_position() {
        // Calculate a world position based on the cursor's position.
        if let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            gizmos.circle_2d(point, 20., Color::RED);
        }
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("John Doe".to_string())));
    commands.spawn((Person, Name("Jane Doe".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
       //println!("Hello {} !", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "John Doe" {
            name.0 = "MARK TWAIN".to_string();
            break;
        }
    }
}

fn add_monster(mut commands: Commands) {
    commands.spawn((Monster, Stats{name: "Monster1".to_string(), health: 100}));
}

fn show_monster(query: Query<&Stats, With<Monster>>) {
    for monster in &query {
        //println!("RAWR!!!!!!! {}, with health: {}  !", monster.name, monster.health);
    }
}

// transforms
fn transform() {

}