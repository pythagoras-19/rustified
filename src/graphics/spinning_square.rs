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

/**
TODO: Bug with exiting to main menu and then trying to open ballin.rs.
 **/

pub fn entry() {
    let window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();
    SpinningSquare::setup(window);
}

pub struct SpinningSquare {
    gl: GlGraphics, // OpenGL for drawing backend
    rotation: f64,
    window: Window,
}

impl SpinningSquare {
    pub fn new(gl: GlGraphics, window: Window) -> Self {
        Self {
            gl,
            rotation: 0.0,
            window,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x,y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // rotate 2 rads per second.
        self.rotation += 2.0 * args.dt;
    }

    fn setup(window: Window) {
        let opengl = OpenGL::V3_2;

        //create new game
        let mut app = SpinningSquare::new(GlGraphics::new(opengl), window);

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut app.window) {
            if let Some(args) = e.render_args() {
                app.render(&args);
            }

            if let Some(args) = e.update_args() {
                app.update(&args);
            }
        }
    }
}