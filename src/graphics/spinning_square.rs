extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::sync::{Arc, Mutex};
use piston_window::types::Color;
use piston_window::ellipse::Border as PistonBorder;
use std::time::SystemTime;
use glutin_window::{GlutinWindow as Window};
use graphics::color::*;
use graphics::{clear, DrawState, Graphics, rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston_window::{Context,WindowSettings};
use rand::Rng;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const PURPLE: [f32; 4] = [0.5, 0.0, 0.5, 1.0];
const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// GAME OBJECT SIZES
const SPINNING_SQUARE_SIZE: f64 = 50.0;
const MAX_TAIL_SIZE: usize = 100;
const SIZE_INCREMENT: f64 = 0.05;

/// WINDOW CONSTANTS
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

/// TRANSLATION CONSTANTS
const SPINNING_SQUARE_MOVE_DISTANCE: f64 = 15.0;
const RIGHT_WINDOW_BORDER: u32 = WINDOW_WIDTH - (SPINNING_SQUARE_SIZE/2.0) as u32;
const LEFT_WINDOW_BORDER: u32 = 0 + (SPINNING_SQUARE_SIZE/2.0) as u32;
const TOP_WINDOW_BORDER: u32 = 0 + (SPINNING_SQUARE_SIZE/2.0) as u32;
const BOTTOM_WINDOW_BORDER: u32 = WINDOW_HEIGHT - (SPINNING_SQUARE_SIZE/2.0) as u32;

/// TRIANGLE OBSTACLE
const TRIANGLE_VERTICES: [[f64; 2]; 3] = [
    [100.0, 80.0],
    [350.0, 150.0],
    [150.0, 250.0],
];


pub fn entry() {
    let window: Window = WindowSettings::new("==SQUARE DANCING==", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    SpinningSquare::setup(window);
}

trait GameObject {
    fn new(gl: Arc<Mutex<GlGraphics>>, window: Window) -> Self;
    fn render(&mut self, args: &RenderArgs, c: Context);
    fn update(&mut self, args: &UpdateArgs);
    fn setup(window: Window);
    fn switch_xy_direction(&mut self);
    fn randomize_square_color(&mut self);
    fn randomize_path_color(&mut self) -> SquareColor;
    fn change_bg_color(&mut self) -> Color;
    fn adjust_size(&mut self);
    fn randomize_color();

    /// Collisions
    fn check_collision(&self, square_center: (f64, f64), square_size: f64, triangle_vertices: [(f64, f64); 3]) -> bool;
    fn point_in_triangle(p: (f64, f64), v1: (f64, f64), v2: (f64, f64), v3: (f64, f64)) -> bool;
    fn get_square_vertices(center: (f64, f64), size: f64) -> [(f64, f64); 4];
}

#[derive(Clone, Debug)]
pub enum SquareColor {
    RED,
    BLUE,
    GREEN,
    YELLOW,
    PURPLE,
    ORANGE,
    BLACK,
}

impl SquareColor {
    fn value(&self) -> [f32; 4] {
        match *self {
            SquareColor::RED => RED,
            SquareColor::BLUE => BLUE,
            SquareColor::GREEN => GREEN,
            SquareColor::YELLOW => YELLOW,
            SquareColor::PURPLE => PURPLE,
            SquareColor::ORANGE => ORANGE,
            SquareColor::BLACK => BLACK,
        }
    }

    fn to_color(&self) -> Color {
        match *self {
            SquareColor::RED => RED,
            SquareColor::BLUE => BLUE,
            SquareColor::GREEN => GREEN,
            SquareColor::YELLOW => YELLOW,
            SquareColor::PURPLE => PURPLE,
            SquareColor::ORANGE => ORANGE,
            SquareColor::BLACK => BLACK,
        }
    }
}

fn random_square_color() -> SquareColor {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..8);

    match num {
        0 => SquareColor::RED,
        1 => SquareColor::BLUE,
        2 => SquareColor::GREEN,
        3 => SquareColor::YELLOW,
        4 => SquareColor::PURPLE,
        5 => SquareColor::ORANGE,
        6 => SquareColor::BLACK,
        _ => SquareColor::YELLOW,
    }
}

/**
TODO: FIX EVIL ELLIPSE (IT DOESNT RENDER)
**/
pub struct EvilEllipse {
    gl: Arc<Mutex<GlGraphics>>,
    path: Vec<([f64; 2], SquareColor)>,
    x_pos: f64,
    y_pos: f64,
    size: f64,
    moving_x_or_y: bool,
    x_direction: bool,
    y_direction: bool,
    color: SquareColor,
    x_speed: f64,
    y_speed: f64,
}

impl EvilEllipse {
    pub fn new(gl: Arc<Mutex<GlGraphics>>) -> Self {
        Self {
            gl,
            color: random_square_color(),
            x_pos: 200.0,
            y_pos: 200.0,
            moving_x_or_y: true, // true = x direction, false = y direction
            x_direction: false,  // true = right, false = left
            y_direction: false, // true = up, false = down
            x_speed: 2.0,
            y_speed: 2.0,
            path: vec![],
            size: 100.0,
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Calculate the new position
        self.x_pos += if self.x_direction { self.x_speed } else { -self.x_speed };
        self.y_pos += if self.y_direction { self.y_speed } else { -self.y_speed };

        // Clamp the position within the window boundaries
        self.x_pos = self.x_pos.clamp(0.0, WINDOW_WIDTH as f64 - self.size);
        self.y_pos = self.y_pos.clamp(0.0, WINDOW_HEIGHT as f64 - self.size);

        // Update the direction if the ellipse hits a boundary
        if self.x_pos <= 0.0 {
            self.x_direction = true; // Switch to right
        } else if self.x_pos >= WINDOW_WIDTH as f64 - self.size {
            self.x_direction = false; // Switch to left
        }

        // todo: IF EvilEllipse hits the triangle, collision logic here

        if self.y_pos <= 0.0 {
            self.y_direction = true; // Switch to down
        } else if self.y_pos >= WINDOW_HEIGHT as f64 - self.size {
            self.y_direction = false; // Switch to up
        }
    }

    pub fn render(&mut self, args: &RenderArgs, c: Context, gl: &mut GlGraphics) {
        use graphics::*;

        let transform = c.transform.trans(self.x_pos, self.y_pos);

        // Create the ellipse with appropriate size and color
        // let ellipse = Ellipse::new(self.color.value())
        //     .border(ellipse::Border {
        //         color: BLACK,
        //         radius: 2.0,
        //     });

        // Draw the ellipse with the calculated transform
        //ellipse.draw([0.0, 0.0, self.size, self.size], &DrawState::default(), transform, gl);

        // Draw the path of the ellipse
        for i in 1..self.path.len() {
            let ([x1, y1], color1) = &self.path[i - 1];
            let ([x2, y2], _) = &self.path[i];
            line(color1.value(), 1.0, [*x1, *y1, *x2, *y2], c.transform, gl);
        }

        // Log render information
        // println!(
        //     "Render - Position: ({:.2}, {:.2}), Size: {:.2}, Color: {:?}",
        //     self.x_pos, self.y_pos, self.size, self.color
        // );
    }


    fn randomize_path_color(&mut self) -> SquareColor {
        random_square_color()
    }
}

struct SpinningSquare {
    gl: Arc<Mutex<GlGraphics>>,
    color: SquareColor,
    rotation: f64,
    x_pos: f64,
    x_direction: bool,
    y_pos: f64,
    y_direction: bool,
    moving_x_or_y: bool,
    path: Vec<([f64; 2], SquareColor)>,
    size: f64,
    increasing_size: bool,
    window: Window,
}

impl GameObject for SpinningSquare {
    fn new(gl: Arc<Mutex<GlGraphics>>, window: Window) -> Self {
        Self {
            gl,
            color: random_square_color(),
            rotation: 0.0,
            x_pos: 600.0,  // initialize to the center of the screen
            y_pos: 200.0,
            moving_x_or_y: false, // true = x direction, false = y direction
            x_direction: true,  // true = right, false = left
            y_direction: true, // true = up, false = down
            window,
            path: vec![],
            size: SPINNING_SQUARE_SIZE,
            increasing_size: true,
        }
    }

    fn render(&mut self, args: &RenderArgs, c: Context) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, self.size);
        let rotation = self.rotation;
        let (x, y) = (self.x_pos, self.y_pos);
        let bg_color = self.change_bg_color(); // alternate bg color

        // Unlock the Mutex to get the GlGraphics instance
        let mut gl = self.gl.lock().unwrap(); // Unlock the mutex

        gl.draw(args.viewport(), |c, gl| {
            clear(bg_color, gl);

            let transform = c
                .transform
                .trans(x, y)  // update position with x_pos
                .rot_rad(rotation)  // apply rotation
                .trans(-25.0, -25.0);  //center the square

            // Draw a spinning square.
            let color = self.color.value();
            rectangle(color, square, transform, gl);


            let draw_state = &DrawState::default();
            let ellipse = Ellipse::new(random_square_color().value())
                .border(PistonBorder {
                    color: BLACK,
                    radius: 3.0,
                });

            ellipse.draw(square, draw_state, transform, gl);

            for i in 1..self.path.len() {
                let ([x1, y1], color1) = &self.path[i - 1];
                let ([x2, y2], _) = &self.path[i];
                line(color1.value(), 10.0, [*x1, *y1, *x2, *y2], c.transform, gl);
            }

            let triangle_color = random_square_color().to_color();
            let triangle = Polygon::new(triangle_color);
            triangle.draw(
                &TRIANGLE_VERTICES,
                &DrawState::default(),
                c.transform,
                gl,
            );
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 9.0 * args.dt;
        self.color = random_square_color();

        /**
        ████████╗██████╗  █████╗ ███╗   ██╗███████╗██╗      █████╗ ████████╗██╗ ██████╗ ███╗   ██╗███████╗
        ╚══██╔══╝██╔══██╗██╔══██╗████╗  ██║██╔════╝██║     ██╔══██╗╚══██╔══╝██║██╔═══██╗████╗  ██║██╔════╝
           ██║   ██████╔╝███████║██╔██╗ ██║███████╗██║     ███████║   ██║   ██║██║   ██║██╔██╗ ██║███████╗
           ██║   ██╔══██╗██╔══██║██║╚██╗██║╚════██║██║     ██╔══██║   ██║   ██║██║   ██║██║╚██╗██║╚════██║
           ██║   ██║  ██║██║  ██║██║ ╚████║███████║███████╗██║  ██║   ██║   ██║╚██████╔╝██║ ╚████║███████║
           ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝
        **/
        if self.moving_x_or_y == true {
            // Update position based on direction
            if self.x_direction {
                self.x_pos += SPINNING_SQUARE_MOVE_DISTANCE;  // Move right
            } else {
                self.x_pos -= SPINNING_SQUARE_MOVE_DISTANCE;  // Move left
            }

            // change direction when hit boundaries
            if self.x_pos >= RIGHT_WINDOW_BORDER as f64 {
                self.x_direction = false;  // Switch to moving left
            } else if self.x_pos <= LEFT_WINDOW_BORDER as f64 {
                self.x_direction = true;  // Switch to moving right
            }
        } else {
            if self.y_direction {
                self.y_pos -= SPINNING_SQUARE_MOVE_DISTANCE;  // Move up
            } else {
                self.y_pos += SPINNING_SQUARE_MOVE_DISTANCE;  // Move down
            }

            // change direction when hit boundaries
            if self.y_pos <= TOP_WINDOW_BORDER as f64 {
                self.y_direction = false;  // Switch to moving down
            } else if self.y_pos >= BOTTOM_WINDOW_BORDER as f64 {
                self.y_direction = true;  // Switch to moving up
            }
        }

        /// convert triangle vertices
        /// TODO: might be hack
        let converted_triangle_vertices: [(f64, f64); 3] = [
            (TRIANGLE_VERTICES[0][0], TRIANGLE_VERTICES[0][1]),
            (TRIANGLE_VERTICES[1][0], TRIANGLE_VERTICES[1][1]),
            (TRIANGLE_VERTICES[2][0], TRIANGLE_VERTICES[2][1]),
        ];

        // Check for collision with the triangle
        let collision = self.check_collision(
            (self.x_pos, self.y_pos),
            self.size,
            converted_triangle_vertices
        );
        if collision {
            println!("Collision detected!");
            println!("Position: {}, {}", self.x_pos, self.y_pos);
            // Reverse direction as a simple response to collision
            if self.moving_x_or_y {
                self.x_direction = !self.x_direction; // Reverse horizontal direction
            } else {
                self.y_direction = !self.y_direction; // Reverse vertical direction
            }
        }

        /**
         ██████╗██╗  ██╗ █████╗ ███╗   ██╗ ██████╗ ███████╗
        ██╔════╝██║  ██║██╔══██╗████╗  ██║██╔════╝ ██╔════╝
        ██║     ███████║███████║██╔██╗ ██║██║  ███╗█████╗
        ██║     ██╔══██║██╔══██║██║╚██╗██║██║   ██║██╔══╝
        ╚██████╗██║  ██║██║  ██║██║ ╚████║╚██████╔╝███████╗
         ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚══════╝

        ██████╗ ██╗██████╗ ███████╗ ██████╗████████╗██╗ ██████╗ ███╗   ██╗
        ██╔══██╗██║██╔══██╗██╔════╝██╔════╝╚══██╔══╝██║██╔═══██╗████╗  ██║
        ██║  ██║██║██████╔╝█████╗  ██║        ██║   ██║██║   ██║██╔██╗ ██║
        ██║  ██║██║██╔══██╗██╔══╝  ██║        ██║   ██║██║   ██║██║╚██╗██║
        ██████╔╝██║██║  ██║███████╗╚██████╗   ██║   ██║╚██████╔╝██║ ╚████║
        ╚═════╝ ╚═╝╚═╝  ╚═╝╚══════╝ ╚═════╝   ╚═╝   ╚═╝ ╚═════╝ ╚═╝  ╚═══╝
        **/
        if rand::random() {
            self.switch_xy_direction();
        }

        let path_color = self.randomize_path_color();
        self.path.push(([self.x_pos, self.y_pos], path_color));

        if self.path.len() > MAX_TAIL_SIZE {
            let drop_amt = self.path.len() - MAX_TAIL_SIZE;
            self.path.drain(0..drop_amt);
        }

        /*

        ███████╗ ██████╗ ██╗   ██╗ █████╗ ██████╗ ███████╗
        ██╔════╝██╔═══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝
        ███████╗██║   ██║██║   ██║███████║██████╔╝█████╗
        ╚════██║██║▄▄ ██║██║   ██║██╔══██║██╔══██╗██╔══╝
        ███████║╚██████╔╝╚██████╔╝██║  ██║██║  ██║███████╗
        ╚══════╝ ╚══▀▀═╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝

        ███████╗██╗███████╗███████╗
        ██╔════╝██║╚══███╔╝██╔════╝
        ███████╗██║  ███╔╝ █████╗
        ╚════██║██║ ███╔╝  ██╔══╝
        ███████║██║███████╗███████╗
        ╚══════╝╚═╝╚══════╝╚══════╝

        */
        // For Slingshot version uncomment next line!
        //self.adjust_size();
    }

    fn setup(window: Window) {
        let opengl = OpenGL::V3_2;
        let gl = Arc::new(Mutex::new(GlGraphics::new(opengl)));

        // Initialize objects with the shared GlGraphics instance
        let mut evil_ellipse = EvilEllipse::new(gl.clone());
        let mut app = SpinningSquare::new(gl.clone(), window);

        let mut events = Events::new(EventSettings::new());

        while let Some(ev) = events.next(&mut app.window) {
            if let Some(args) = ev.update_args() {
                app.update(&args);  // Update state for SpinningSquare
                evil_ellipse.update(&args);  // Update state for evil_ellipse
            }

            if let Some(args) = ev.render_args() {
                // Create a new `Context`
                let context = Context::new();  // Use an existing method to initialize the context

                // After dropping the immutable borrow, you can now use `app` for mutable operations
                app.render(&args, context);  // Pass the newly created context to render

                // Separate the immutable and mutable borrow scopes to avoid conflicts

                let mut gl = gl.lock().unwrap();  // Use the shared instance
                evil_ellipse.render(&args, context, &mut gl);
                // gl.draw(args.viewport(), |c, gl| {
                //     clear([0.5, 0.5, 0.5, 1.0], gl);
                //     evil_ellipse.render(&args, c, gl);  // Render evil_ellipse within this scope
                // });
            }
        }
    }

    fn switch_xy_direction(&mut self) {
        self.moving_x_or_y = !self.moving_x_or_y;
    }

    fn randomize_square_color(&mut self) {
        self.color = random_square_color();
    }

    fn randomize_path_color(&mut self) -> SquareColor {
        random_square_color()
    }

    fn change_bg_color(&mut self) -> Color {
        let now = SystemTime::now();
        let seconds = match now.duration_since(SystemTime::UNIX_EPOCH) {
            /// error handling
            Ok(duration) => duration.as_secs(),
            Err(e) => {
                eprintln!("An error occurred: {}", e);
                return RED;
            },
        };

        if seconds % 40 < 10 {
            YELLOW
        } else if seconds % 40 < 20 {
            NAVY
        } else if seconds % 40 < 30 {
            PURPLE
        } else {
            BLACK
        }
    }

    fn adjust_size(&mut self) {
        if self.increasing_size {
            self.size += SIZE_INCREMENT;
        } else {
            self.size -= SIZE_INCREMENT;
        }
        // Switch between increasing and decreasing every 10 seconds
        let now = SystemTime::now();
        let secs = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        if secs % 20 < 5 {
            self.increasing_size = true;
        } else {
            self.increasing_size = false;
        }
    }

    fn randomize_color() {
        random_square_color();
    }


    /**
     ██████╗ ██████╗ ██╗     ██╗     ██╗███████╗██╗ ██████╗ ███╗   ██╗███████╗
    ██╔════╝██╔═══██╗██║     ██║     ██║██╔════╝██║██╔═══██╗████╗  ██║██╔════╝
    ██║     ██║   ██║██║     ██║     ██║███████╗██║██║   ██║██╔██╗ ██║███████╗
    ██║     ██║   ██║██║     ██║     ██║╚════██║██║██║   ██║██║╚██╗██║╚════██║
    ╚██████╗╚██████╔╝███████╗███████╗██║███████║██║╚██████╔╝██║ ╚████║███████║
     ╚═════╝ ╚═════╝ ╚══════╝╚══════╝╚═╝╚══════╝╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝
    **/

    /// Check if the square collides with the triangle
    fn check_collision(&self, square_center: (f64, f64), square_size: f64, triangle_vertices: [(f64, f64); 3]) -> bool {
        let square_vertices = SpinningSquare::get_square_vertices(square_center, square_size);
        square_vertices.iter().any(|&vert| SpinningSquare::point_in_triangle(vert, triangle_vertices[0], triangle_vertices[1], triangle_vertices[2]))
    }

    /// Returns true if point p is inside the triangle defined by vertices v1, v2, v3
    fn point_in_triangle(p: (f64, f64), v1: (f64, f64), v2: (f64, f64), v3: (f64, f64)) -> bool {
        let sign = |p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)| {
            (p1.0 - p3.0) * (p2.1 - p3.1) - (p2.0 - p3.0) * (p1.1 - p3.1)
        };

        let d1 = sign(p, v1, v2);
        let d2 = sign(p, v2, v3);
        let d3 = sign(p, v3, v1);

        let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
        let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

        !(has_neg && has_pos) // no sign difference means p is inside the triangle
    }

    /// Calculates the vertices of the square given its center position and size
    fn get_square_vertices(center: (f64, f64), size: f64) -> [(f64, f64); 4] {
        [
            (center.0 - size / 2.0, center.1 - size / 2.0),
            (center.0 + size / 2.0, center.1 - size / 2.0),
            (center.0 + size / 2.0, center.1 + size / 2.0),
            (center.0 - size / 2.0, center.1 + size / 2.0),
        ]
    }
}