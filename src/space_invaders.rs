/*#[path = "fb.rs"]
mod fb;
#[path = "gl.rs"]
mod gl;

use crate::cpu;
use core::convert::TryInto;
use gl::Display;

use embedded_graphics::{
    pixelcolor::Bgr888,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::PrimitiveStyle,
    DrawTarget,
};

struct Spaceship {
    pos_x: i32,
    pos_y: i32,
    size: i32,
    first_set: bool,
    prev_dx: i32,
    prev_dy: i32,
    second_set: bool,
    second_prev_dx: i32,
    second_prev_dy: i32,
}

impl Spaceship {
    pub fn draw(&self) -> Result<(), core::convert::Infallible> {
        let mut display = Display {};
        let green = PrimitiveStyle::with_fill(Bgr888::GREEN);
        Triangle::new(
            Point::new(self.pos_x, self.pos_y - self.size),
            Point::new(self.pos_x - self.size / 2, self.pos_y),
            Point::new(self.pos_x + self.size / 2, self.pos_y),
        )
        .into_styled(green)
        .draw(&mut display)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), core::convert::Infallible> {
        let mut display = Display {};
        let black = PrimitiveStyle::with_fill(Bgr888::BLACK);
        if self.second_set {
            Triangle::new(
                Point::new(
                    self.pos_x - self.prev_dx,
                    self.pos_y - self.size - self.second_prev_dy,
                ),
                Point::new(
                    self.pos_x - self.prev_dx - self.size / 2,
                    self.pos_y - self.prev_dy,
                ),
                Point::new(
                    self.pos_x - self.prev_dx + self.size / 2,
                    self.pos_y - self.prev_dy,
                ),
            )
            .into_styled(black)
            .draw(&mut display)?;
        }
        Ok(())
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.pos_x += dx;
        self.pos_y += dy;

        if (self.first_set) {
            self.second_prev_dx = self.second_prev_dx;
            self.second_prev_dy = self.second_prev_dy;
            self.second_set = true;
        } else {
            self.first_set = true;
        }

        self.prev_dx = dx;
        self.prev_dy = dy;
    }
}

pub unsafe fn game() -> Result<(), core::convert::Infallible> {
    // TODO make gl_init
    fb::fb_init(640, 512, 4, fb::FB_DOUBLEBUFFER);
    let w = 640;
    let h = 512;

    let mut ship = Spaceship {
        pos_x: 60,
        pos_y: h - 30,
        size: 30,
        first_set: false,
        prev_dx: 0,
        prev_dy: 0,
        second_set: false,
        second_prev_dx: 0,
        second_prev_dy: 0,
    };

    let mut dx: i32 = 7;

    loop {
        ship.clear();
        ship.move_by(dx, 0);
        ship.draw();
        fb::fb_swap_buffer();
        cpu::sleep(200000);
        if (ship.pos_x + 30 > w - 30 && dx > 0) || (ship.pos_x - 30 < 30 && dx < 0) {
            dx *= -1;
        }
    }
}

#[test_case]
pub fn test_invaders() {
    unsafe {
        game();
    }
}*/
