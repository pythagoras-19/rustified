extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::rectangle;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston_window::WindowSettings;

pub fn entry() {
    let window: Window = WindowSettings::new("spinning-square", [400, 400])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    SpinningSquare::setup(window);
}

pub struct SpinningSquare {
    gl: GlGraphics,
    rotation: f64,
    x_pos: f64,
    x_direction: bool,
    y_pos: f64,
    y_direction: bool,
    moving_x_or_y: bool,
    window: Window,
}

impl SpinningSquare {
    pub fn new(gl: GlGraphics, window: Window) -> Self {
        Self {
            gl,
            rotation: 0.0,
            x_pos: 200.0,  // initialize to the center of the screen
            y_pos: 200.0, // todo: maybe adjust me
            moving_x_or_y: false, // true = x direction, false = y direction
            x_direction: true,  // true = right, false = left
            y_direction: true, // true = up, false = down
            window,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (self.x_pos, self.y_pos);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear screen.
            clear(WHITE, gl);

            let transform = c
                .transform
                .trans(x, y)  // update position with x_pos
                .rot_rad(rotation)  // apply rotation
                .trans(-25.0, -25.0);  //center the square

            // Draw a spinning square.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 9.0 * args.dt;

        /**

    ████████ ██████   █████  ███    ██ ███████ ██       █████  ████████ ██  ██████  ███    ██ ███████
       ██    ██   ██ ██   ██ ████   ██ ██      ██      ██   ██    ██    ██ ██    ██ ████   ██ ██
       ██    ██████  ███████ ██ ██  ██ ███████ ██      ███████    ██    ██ ██    ██ ██ ██  ██ ███████
       ██    ██   ██ ██   ██ ██  ██ ██      ██ ██      ██   ██    ██    ██ ██    ██ ██  ██ ██      ██
       ██    ██   ██ ██   ██ ██   ████ ███████ ███████ ██   ██    ██    ██  ██████  ██   ████ ███████

        **/
        if (self.moving_x_or_y == true) {
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
            if self.y_pos <= 25.0  {
                self.y_direction = false;  // Switch to moving down
            } else if self.y_pos >= 375.0 {
                self.y_direction = true;  // Switch to moving up
            }
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

    fn switch_xy_direction(&self) {
        let _ = !self.moving_x_or_y;
    }
}