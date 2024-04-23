extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston_window::types::Color;
use std::time::SystemTime;
use glutin_window::GlutinWindow as Window;
use graphics::rectangle;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston_window::WindowSettings;
use rand::Rng;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const PURPLE: [f32; 4] = [0.5, 0.0, 0.5, 1.0];
const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

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

pub fn entry() {
    let window: Window = WindowSettings::new("==SQUARE DANCING==", [400, 400])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    SpinningSquare::setup(window);
}

pub struct SpinningSquare {
    gl: GlGraphics,
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
    pub fn new(gl: GlGraphics, window: Window) -> Self {
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
            size: 50.0,
            increasing_size: true,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, self.size);
        let rotation = self.rotation;
        let (x, y) = (self.x_pos, self.y_pos);
        let bg_color = self.change_bg_color(); // alternate bg color

        self.gl.draw(args.viewport(), |c, gl| {
            // clear with new bg color
            clear(bg_color, gl);

            let transform = c
                .transform
                .trans(x, y)  // update position with x_pos
                .rot_rad(rotation)  // apply rotation
                .trans(-25.0, -25.0);  //center the square

            // Draw a spinning square.
            let color = self.color.value();
            rectangle(color, square, transform, gl);

            //draw path
            println!("Path size: {}", self.path.len());
            for i in 1..self.path.len() {
                let ([x1, y1], color1) = &self.path[i - 1];
                let ([x2, y2], _) = &self.path[i];
                line(color1.value(), 1.0, [*x1, *y1, *x2, *y2], c.transform, gl);
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
                self.x_pos += 2.0;  // Move right
            } else {
                self.x_pos -= 2.0;  // Move left
            }

            // change direction when hit boundaries
            if self.x_pos >= 375.0 {
                self.x_direction = false;  // Switch to moving left
            } else if self.x_pos <= 25.0 {
                self.x_direction = true;  // Switch to moving right
            }
        } else {
            if self.y_direction {
                self.y_pos -= 2.0;  // Move up
            } else {
                self.y_pos += 2.0;  // Move down
            }

            // change direction when hit boundaries
            if self.y_pos <= 25.0 {
                self.y_direction = false;  // Switch to moving down
            } else if self.y_pos >= 375.0 {
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
        const MAX_PATH_SIZE: usize = 1000;
        if self.path.len() > MAX_PATH_SIZE {
            let drop_amt = self.path.len() - MAX_PATH_SIZE;
            self.path.drain(0..drop_amt);
        }

        /**

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

        **/
        if self.increasing_size {
            self.size += 0.05;
        } else {
            self.size -= 0.05;
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

    fn setup(window: Window) {
        let opengl = OpenGL::V3_2;

        // Create the application instance
        let mut app = SpinningSquare::new(GlGraphics::new(opengl), window);

        let mut events = Events::new(EventSettings::new());
        while let Some(ev) = events.next(&mut app.window) {
            if let Some(args) = ev.render_args() {
                app.render(&args);  // Render square
            }

            if let Some(args) = ev.update_args() {
                app.update(&args);  // Update the square state
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
            PURPLE
        } else if seconds % 40 < 20 {
            BLUE
        } else if seconds % 40 < 30 {
            RED
        } else {
            ORANGE
        }
    }
}