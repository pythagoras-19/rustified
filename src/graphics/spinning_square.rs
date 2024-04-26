extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::sync::{Arc, Mutex};
use piston_window::types::Color;
use piston_window::ellipse::Border as PistonBorder;
use std::time::SystemTime;
use glutin_window::{GlutinWindow as Window, GlutinWindow};
use graphics::color::{NAVY, TRANSPARENT};
use graphics::{clear, DrawState, Graphics, rectangle, Rectangle};
use graphics::math::Matrix2d;
use graphics::types::{Radius, Resolution};
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

// GAME OBJECT SIZES
const SPINNING_SQUARE_SIZE: f64 = 50.0;

// WINDOW CONSTANTS
const WINDOW_WIDTH: u32 = 1600;
const WINDOW_HEIGHT: u32 = 900;

// TRANSLATION CONSTANTS
const SPINNING_SQUARE_MOVE_DISTANCE: f64 = 4.0;
const RIGHT_WINDOW_BORDER: u32 = WINDOW_WIDTH - (SPINNING_SQUARE_SIZE/2.0) as u32;
const LEFT_WINDOW_BORDER: u32 = 0 + (SPINNING_SQUARE_SIZE/2.0) as u32;
const TOP_WINDOW_BORDER: u32 = 0 + (SPINNING_SQUARE_SIZE/2.0) as u32;
const BOTTOM_WINDOW_BORDER: u32 = WINDOW_HEIGHT - (SPINNING_SQUARE_SIZE/2.0) as u32;

pub fn entry() {
    let window: Window = WindowSettings::new("==SQUARE DANCING==", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    SpinningSquare::setup(window);
}

#[derive(Clone)]
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
}

#[derive(Copy, Clone)]
pub struct Border {
    pub color: Color,
    pub radius: Radius,
}

pub struct Line {
    pub color: [f32; 4],
    pub radius: Radius,
}

pub struct Ellipse {
    pub color: SquareColor,
    pub border: Option<Border>,
    pub resolution: Resolution,
}

impl Ellipse {
    pub fn new(color: SquareColor) -> Ellipse {
        Ellipse {
            color,
            border: None,
            resolution: 128,
        }
    }

    pub fn new_border(color: SquareColor, radius: Radius) -> Ellipse {
        let color_clone = color.clone();
        Ellipse {
            color,
            border: Some(Border {
                color: color_clone.value(),
                radius,
            }),
            resolution: 128,
        }
    }

    pub fn color(mut self, value: SquareColor) -> Self {
        self.color = value;
        self
    }

    pub fn border(mut self, value: Border) -> Self {
        self.border = Some(value);
        self
    }

    #[inline(always)]
    pub fn draw<R: Into<[f64; 4]>, G>(&mut self, rectangle: R, draw_state: &DrawState, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {
        println!("drawing ellipse");
        let ellipse = piston_window::Ellipse::new(self.color.value())
            .resolution(self.resolution)
            .border(PistonBorder {
                color: self.border.as_ref().map(|b| b.color).unwrap_or([0.0; 4]),
                radius: self.border.as_ref().map(|b| b.radius).unwrap_or(0.0),
            });

        g.ellipse(&ellipse, rectangle.into(), draw_state, transform);
    }

    fn randomize_ellipse_color(&mut self) -> Color {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..8);

        match num {
            0 => RED,
            1 => BLUE,
            2 => GREEN,
            3 => YELLOW,
            4 => PURPLE,
            5 => ORANGE,
            6 => BLACK,
            _ => YELLOW,
        }
    }
}

pub struct Ellipse2 {
    gl: Arc<Mutex<GlGraphics>>,
    path: Vec<([f64; 2], SquareColor)>, // HACK, change this to ellipse data structure TODO: refactor
    x_pos: f64,
    y_pos: f64,
    size: f64,
    moving_x_or_y: bool,
    x_direction: bool,
    y_direction: bool,
    color: SquareColor
}

impl Ellipse2 {
    pub fn new(gl: Arc<Mutex<GlGraphics>>) -> Self {
        Self {
            gl,
            color: SquareColor::BLUE,
            x_pos: 100.0,  // initialize to the center of the screen
            y_pos: 100.0,
            moving_x_or_y: true, // true = x direction, false = y direction
            x_direction: false,  // true = right, false = left
            y_direction: false, // true = up, false = down
            path: vec![],
            size: SPINNING_SQUARE_SIZE,
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let x2 = rand::random::<f64>() * WINDOW_WIDTH as f64;
        let y2 = rand::random::<f64>() * WINDOW_HEIGHT as f64;

        // push coords
        let path_color = self.randomize_path_color();
        self.path.push(([x2,y2], path_color));
        const MAX_PATH_SIZE: usize = 250;
        if self.path.len() > MAX_PATH_SIZE {
            let drop_amt = self.path.len() - MAX_PATH_SIZE;
            self.path.drain(0..drop_amt);
        }
    }

    pub fn render(&mut self, args: &RenderArgs, c: Context, gl: &mut GlGraphics) {
        use graphics::*;

        // Create an ellipse with the current color of Ellipse2
        let ellipse = ellipse::Ellipse::new(self.color.value());

        // Draw the ellipse at the current position of Ellipse2
        let transform = c.transform.trans(self.x_pos, self.y_pos);
        ellipse.draw([0.0, 0.0, self.size, self.size], &DrawState::default(), transform, gl);

        // Draw the path of Ellipse2
        for i in 1..self.path.len() {
            println!("PATH LENGTH OF ELLIPSE 2: {}", self.path.len());
            let ([x1, y1], color1) = &self.path[i - 1];
            let ([x2, y2], _) = &self.path[i];
            line(color1.value(), 1.0, [*x1, *y1, *x2, *y2], c.transform, gl);
        }
    }

    // TODO: REFACTOR ME BECUASE DUPLICATED CODE !!!
    fn randomize_path_color(&mut self) -> SquareColor {
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

}

pub struct SpinningSquare {
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

impl SpinningSquare {
    pub fn new(gl: Arc<Mutex<GlGraphics>>, window: Window) -> Self {
        Self {
            gl,
            color: SquareColor::BLUE,
            rotation: 0.0,
            x_pos: 200.0,  // initialize to the center of the screen
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

            let ellipse_transform = c
                .transform
                .trans(x + 10.0, y + 10.0)
                .rot_rad(rotation)
                .trans(-250.0, -250.0);

            let draw_state = &DrawState::default();
            let ellipse = Ellipse::new(BLUE)
                .border(PistonBorder {
                    color: BLACK,
                    radius: 2.0,
                });

            ellipse.draw(square, draw_state, transform, gl);

            println!("Path size: {}", self.path.len());
            for i in 1..self.path.len() {
                let ([x1, y1], color1) = &self.path[i - 1];
                let ([x2, y2], _) = &self.path[i];
                line(color1.value(), 10.0, [*x1, *y1, *x2, *y2], c.transform, gl);
            }
        });
    }


    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 9.0 * args.dt;

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

        self.randomize_square_color();

        let path_color = self.randomize_path_color();
        self.path.push(([self.x_pos, self.y_pos], path_color));

        const MAX_PATH_SIZE: usize = 250;
        if self.path.len() > MAX_PATH_SIZE {
            let drop_amt = self.path.len() - MAX_PATH_SIZE;
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

    pub fn setup(window: Window) {
        let opengl = OpenGL::V3_2;
        let gl = Arc::new(Mutex::new(GlGraphics::new(opengl)));

        // Initialize objects with the shared GlGraphics instance
        let mut ellipse2 = Ellipse2::new(gl.clone());
        let mut app = SpinningSquare::new(gl.clone(), window);

        let mut events = Events::new(EventSettings::new());

        while let Some(ev) = events.next(&mut app.window) {
            if let Some(args) = ev.update_args() {
                app.update(&args);  // Update state for SpinningSquare
                ellipse2.update(&args);  // Update state for Ellipse2
            }

            if let Some(args) = ev.render_args() {
                // Separate the immutable and mutable borrow scopes to avoid conflicts
                {
                    let mut gl = gl.lock().unwrap();  // Use the shared instance
                    gl.draw(args.viewport(), |c, gl| {
                        clear([0.5, 0.5, 0.5, 1.0], gl);  // Example of clearing the screen
                        ellipse2.render(&args, c, gl);  // Render Ellipse2 within this scope
                    });  // The immutable borrow ends here
                }

                // Create a new `Context`
                let context = Context::new();  // Use an existing method to initialize the context

                // After dropping the immutable borrow, you can now use `app` for mutable operations
                app.render(&args, context);  // Pass the newly created context to render
            }
        }
    }

    fn switch_xy_direction(&mut self) {
        self.moving_x_or_y = !self.moving_x_or_y;
    }

    fn randomize_square_color(&mut self) {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..8);  // Generate a random number in the range 0-7

        self.color = match num {
            0 => SquareColor::RED,
            1 => SquareColor::BLUE,
            2 => SquareColor::GREEN,
            3 => SquareColor::YELLOW,
            4 => SquareColor::PURPLE,
            5 => SquareColor::ORANGE,
            6 => SquareColor::BLACK,
            _ => SquareColor::YELLOW,
        };
    }

    fn randomize_path_color(&mut self) -> SquareColor {
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

    fn change_bg_color(&mut self) -> Color {
        let now = SystemTime::now();
        let seconds = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        if seconds % 40 < 10 {
            YELLOW
        } else if seconds % 40 < 20 {
            NAVY
        } else if seconds % 40 < 30 {
            RED
        } else {
            TRANSPARENT
        }
    }

    fn adjust_size(&mut self) {
        const SIZE: f64 = 0.05;
        if self.increasing_size {
            self.size += SIZE;
        } else {
            self.size -= SIZE;
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
}