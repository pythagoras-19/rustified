/// usage:: cargo run --bin snake
/// Actually, NOT A SNAKE GAME ;)\
/// TODO: fix hardcoded Z values

use bevy::prelude::*;
use bevy::asset::AssetContainer;
use bevy::render::color::Color;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use bevy::ui::RelativeCursorPosition;
use bevy_rapier2d::prelude::*;

use colored::Colorize;

/// TODO: Fix these names, they're dumb, lets change to fibonacci
static FIB_1: f32 = 1.0;
static FIB_5: f32 = 5.0;
static FIB_12: f32 = 12.0;
static FIB_20: f32 = 20.0;
static FIB_30: f32 = 30.0;
static FIB_144: f32 = 144.0;

#[derive(Component)]
struct Orange;

#[derive(Component)]
struct MainCamera;

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
struct NavySquare2;

#[derive(Component)]
struct OrangeCircle;

#[derive(Component)]
struct BlueSquare;

#[derive(Component)]
struct BlueCircle;

#[derive(Component)]
struct RedCircle;

#[derive(Component)]
struct A; // circle

#[derive(Component)]
struct B; // circle

#[derive(Component)]
struct C; // circle

#[derive(Component)]
struct D; // circle

#[derive(Component)]
struct E; // circle

#[derive(Component)]
struct F; // circle

#[derive(Component)]
struct Player {
    movement_speed: f32,
    jump_force: f32,
    player_colliding: bool,
    facing_right: bool,
}

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

#[derive(Resource)]
struct GameState {
    paused: bool
}

impl GameState {
    fn new() -> Self {
        Self { paused: false }
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Game::new())
        .insert_resource(GameState::new())
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup, add_people, add_monster))
        .add_systems(Update, (draw_cursor, (update_people, greet_people, show_monster, print_interactions).chain()))
        .add_systems(Update, move_entities)
        .run();
}

fn toggle_pause_system(mut game_state: ResMut<GameState>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyP) {
        println!("Pressed {:?}", KeyCode::KeyP );
        game_state.toggle_pause();
    }
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
            1.0,
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
            2.0,
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
            3.0,
        ),
        ..default()
    }).id();
    game.add(circle_entity);

    let blue_circle = Mesh2dHandle(meshes.add(Circle::new(200.)));
    let blue_circle_entity = commands.spawn(MaterialMesh2dBundle{
        mesh: blue_circle.into(),
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(
            -200.,
            -150.0,
            4.,
        ),
        ..default()
    }).id();
    commands.entity(blue_circle_entity).insert(BlueCircle);
    game.add(blue_circle_entity);

    let circle = Mesh2dHandle(meshes.add(Circle::new(400.0)));
    let orange_red_circle_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: circle.into(),
        material: materials.add(Color::ORANGE_RED),
        transform: Transform::from_xyz(
            -500.0,
            -300.0,
            5.,
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
            6.,
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
            7.,
        ),
        ..default()
    }).id();
    game.add(pink_rectangle_entity);

    let square = Mesh2dHandle(meshes.add(Rectangle::new(500.0, 500.0)));
    let blue_square_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: square.into(),
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(
            0.0,
            0.0,
            8.0,
        ),
        ..default()
    }).id();
    commands.entity(blue_square_entity).insert(BlueSquare);
    game.add(blue_square_entity);

    let square = Mesh2dHandle(meshes.add(Rectangle::new(350., 350.)));
    let navy_square_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: square.into(),
        material: materials.add(Color::NAVY),
        transform: Transform::from_xyz(
            10.0,
            10.0,
            9.0,
        ),
        ..default()
    }).id();
    commands.entity(navy_square_entity).insert(NavySquare2);
    game.add(navy_square_entity);

    let circle = Mesh2dHandle(meshes.add(Circle::new(400.0)));
    let red_circle_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: circle.into(),
        material: materials.add(Color::RED),
        transform:Transform::from_xyz(
            -20.0,
            -20.0,
            10.,
        ),
        ..default()
    }).id();
    commands.entity(red_circle_entity).insert(RedCircle);
    game.add(red_circle_entity);

    let t = Mesh2dHandle(meshes.add(Circle::new(200.0)));
    let t_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: t.into(),
        material: materials.add(Color::GRAY),
        transform: Transform::from_xyz(
            -30.,
            -30.,
            11.,
        ),
        ..default()
    }).id();
    commands.entity(t_entity).insert(A);
    game.add(t_entity);

    let c = Mesh2dHandle(meshes.add(Circle::new(200.0)));
    let c_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: c.into(),
        material: materials.add(Color::DARK_GRAY),
        transform: Transform::from_xyz(
            -3.,
            20.,
            12.,
        ),
        ..default()
    }).id();
    commands.entity(c_entity).insert(C);
    game.add(c_entity);

    let l = Mesh2dHandle(meshes.add(Circle::new(100.)));
    let l_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: l.into(),
        material: materials.add(Color::PURPLE),
        transform: Transform::from_xyz(
            -100.,
            -75.,
            13.,
        ),
        ..default()
    }).id();
    commands.entity(l_entity).insert(B);
    game.add(l_entity);

    let d_circle = Mesh2dHandle(meshes.add(Circle::new(140.)));
    let d_circle_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: d_circle.into(),
        material: materials.add(Color::SEA_GREEN),
        transform: Transform::from_xyz(
            -25.,
            25.,
            14.
        ),
        ..default()
    }).id();
    commands.entity(d_circle_entity).insert(D);
    game.add(d_circle_entity);

    let e_circle = Mesh2dHandle(meshes.add(Circle::new(120.)));
    let e_circle_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: e_circle.into(),
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(
            250.,
            250.,
            15.
        ),
        ..default()
    }).id();
    commands.entity(e_circle_entity).insert(E);
    game.add(e_circle_entity);

    let f_circle = Mesh2dHandle(meshes.add(Circle::new(220.)));
    let f_circle_entity = commands.spawn(MaterialMesh2dBundle {
        mesh: f_circle.into(),
        material: materials.add(Color::FUCHSIA),
        transform: Transform::from_xyz(
            250.,
            250.,
            16.
        ),
        ..default()
    }).id();
    commands.entity(f_circle_entity).insert(F);
    game.add(f_circle_entity);

    let button_1 = commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(50.),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::End,
                ..Default::default()
            },
            background_color: BackgroundColor::from(Color::RED),
            ..Default::default()
        })
        .id();

    commands
        .entity(button_1)
        .insert(RelativeCursorPosition::default());
}

fn print_interactions(nodes: Query<(&RelativeCursorPosition, &ViewVisibility), With<Node>>) {
    for (cursor_position, visibility) in &nodes {
        if visibility.get() && cursor_position.mouse_over() {
            println!("{}", "over".green());
            println!("cursor position not normalized: {:?}", &cursor_position);
            if let Some(normalized) = &cursor_position.normalized {
                println!("Coordinates: {:?}", normalized);
                if normalized.x > 5.0 {
                    println!("At x > 5");
                }
                if normalized.y < 4.0 {
                    println!("At y > 5");
                }
            }
        } else {
            // println!("{}", "Not over".red());
            // if let Some(normalized) = &cursor_position.normalized {
            //      println!("{:?}", normalized);
            // }
        }
    }
}

fn move_entities(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, Option<&AquaSquare>, Option<&NavySquare>,
                      Option<&OrangeCircle>, Option<&BlueSquare>, Option<&BlueCircle>,
                      Option<&NavySquare2>, Option<&RedCircle>, Option<&A>, Option<&B>,
                      Option<&C>, Option<&D>, Option<&E>, Option<&F>)>,
    game: Res<Game>,
    game_state: Res<GameState>,
    windows:Query<&Window, With<PrimaryWindow>>
) {
    if game_state.paused {
        return;
    }

    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();

    let x_boundary = window_width / 2.0;
    let y_boundary = window_height / 2.0;

    for (entity, mut transform, aqua, navy,
        orange, blue_sq, blue_cir,
        navy2, red_cir, a, b,
        c, d, e, f) in query.iter_mut() {
        if game.game_objects.contains(&entity) {
            if let Some(_) = aqua {
                transform.translation.x -= FIB_5;
                transform.translation.y -= FIB_1;
            } else if let Some(_) = navy {
                transform.translation.x += FIB_20;
                transform.translation.y += FIB_12;
            } else if let Some(_) = orange {
                transform.translation.x += FIB_1;
                transform.translation.y -= FIB_30;
            } else if let Some(_) = blue_sq {
                transform.translation.x += FIB_1;
                transform.translation.y -= FIB_1;
            } else if let Some(_) = blue_cir {
                transform.translation.x -= FIB_1;
                transform.translation.y -= FIB_1;
            } else if let Some(_) = navy2 {
                transform.translation.x -= FIB_1;
                transform.translation.y += FIB_1;
            } else if let Some(_) = red_cir {
                transform.translation.x -= FIB_1;
                transform.translation.y += FIB_1;
            } else if let Some(_) = a {
                transform.translation.x -= FIB_1;
                transform.translation.y += FIB_1;
            } else if let Some(_) = b {
                transform.translation.x += FIB_1;
                transform.translation.y -= FIB_1;
            } else if let Some(_) = c {
                transform.translation.x -= FIB_1;
                transform.translation.y -= FIB_1;
            } else if let Some(_) = d {
                transform.translation.x -= FIB_1;
                transform.translation.y += FIB_1;
            } else if let Some(_) = e {
                transform.translation.x -= FIB_1;
                transform.translation.y += FIB_1;
            }
            else if let Some(_) = e {
                //transform.translation.x -= FIB_1;
                // transform.translation.y += FIB_1;
            }

            // Out of bounds checker
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
            gizmos.circle_2d(point, FIB_144, Color::RED);
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
        //println!("{}, with health: {}  !", monster.name, monster.health);
    }
}

// transforms
fn transform() {
}

/// Mouse events
fn my_cursor_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}