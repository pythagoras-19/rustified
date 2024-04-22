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
    direction: bool,
    window: Window,
}

impl SpinningSquare {
    pub fn new(gl: GlGraphics, window: Window) -> Self {
        Self {
            gl,
            rotation: 0.0,
            x_pos: 200.0,  // initialize to the center of the screen
            direction: true,  // true = right, false = left
            window,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (self.x_pos, args.window_size[1] / 2.0);

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

        // Update position based on direction
        if self.direction {
            self.x_pos += 2.0;  // Move right
        } else {
            self.x_pos -= 2.0;  // Move left
        }

        // change direction when hit boundaries
        if self.x_pos >= 375.0 {
            self.direction = false;  // Switch to moving left
        } else if self.x_pos <= 25.0 {
            self.direction = true;  // Switch to moving right
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
}
