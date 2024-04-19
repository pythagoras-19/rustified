extern crate piston_window;
use piston_window::*;

struct Ball {
    x: f64,
    y: f64,
    radius: f64,
    velocity_y: f64,
    velocity_x: f64,
}

pub fn entry() {
    let mut window: PistonWindow = WindowSettings::new("Bouncing Ball", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut ball = Ball::new(100.0, 0.0, 35.0);
    let mut cursor = [0.0, 0.0];

    while let Some(e) = window.next() {
        if let Some(_) = e.close_args() {
            break;
        }

        e.mouse_cursor(|pos| {
            cursor = pos;
        });

        if let Some(args) = e.update_args() {
            ball.update(args.dt);
        }

        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            ball.draw(c, g);
        });
    }
}
impl Ball {
    fn new(x: f64, y: f64, radius: f64) -> Ball {
        Ball {
            x,
            y,
            radius,
            velocity_y: 0.0,
            velocity_x: 0.0,
        }
    }

    fn update(&mut self, dt: f64) {
        let gravity = 9.8;
        let ground_friction = 0.9;
        let elasticity = 0.7;

        // Gravity effect
        self.velocity_y += gravity * dt;

        // Update position
        self.x += self.velocity_x * dt;
        self.y += self.velocity_y * dt;

        // Ground collision
        if self.y + self.radius > 300.0 { // assuming ground is at y = 300
            self.y = 300.0 - self.radius;
            self.velocity_y *= -elasticity;
            self.velocity_x *= ground_friction;
        }
    }

    fn draw<G: Graphics>(&self, c: Context, g: &mut G) {
        ellipse([1.0, 0.0, 0.0, 1.0], // red color
                [self.x - self.radius, self.y - self.radius, self.radius * 2.0, self.radius * 2.0],
                c.transform, g);
    }
}
