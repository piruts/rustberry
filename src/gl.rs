#[path = "fb.rs"]
mod fb;

use crate::cpu;
use core::convert::TryInto;

use embedded_graphics::{
    pixelcolor::Bgr888,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::PrimitiveStyle,
    DrawTarget,
};

pub struct Display {}

impl DrawTarget<Bgr888> for Display {
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Bgr888>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            unsafe {
                let w = fb::fb_get_width();
                let h = fb::fb_get_height();

                let x: u32 = coord.x.try_into().unwrap();
                let y: u32 = coord.y.try_into().unwrap();

                if (x < w) && (y < h) {
                    // Calculate the index in the framebuffer.
                    let index: u32 = (fb::fb_get_depth() * x) + (y * fb::fb_get_pitch());

                    let framebuffer = fb::fb_get_draw_buffer() as *mut u8;

                    *framebuffer.offset(index as isize) = color.b();
                    *framebuffer.offset((index + 1) as isize) = color.g();
                    *framebuffer.offset((index + 2) as isize) = color.r();
                    *framebuffer.offset((index + 3) as isize) = 0xff;
                }
            }
        }
        Ok(())
    }

    fn size(&self) -> Size {
        unsafe { Size::new(fb::fb_get_width(), fb::fb_get_height()) }
    }

    fn draw_pixel(
        &mut self,
        pixel: embedded_graphics::drawable::Pixel<Bgr888>,
    ) -> Result<(), Self::Error> {
        unsafe {
            let color = pixel.1;

            let x: u32 = pixel.0.x.try_into().unwrap();
            let y: u32 = pixel.0.y.try_into().unwrap();

            let w = fb::fb_get_width();
            let h = fb::fb_get_height();

            if (x < w) && (y < h) {
                let index: u32 = (fb::fb_get_depth() * x) + (y * fb::fb_get_pitch());

                let framebuffer = fb::fb_get_draw_buffer() as *mut u8;

                *framebuffer.offset(index as isize) = color.b();
                *framebuffer.offset((index + 1) as isize) = color.g();
                *framebuffer.offset((index + 2) as isize) = color.r();
                *framebuffer.offset((index + 3) as isize) = 0xff;
            }
        }

        Ok(())
    }
}

#[test_case]
pub fn test_gl() {
    unsafe {
        space_invaders();
        //_gl_test();
    }
}

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
                    self.pos_y - self.size - self.prev_dy,
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

struct EnemyRow {
    row_size: i32,
    start_x: i32,
    start_y: i32,
    size: i32,
    prev_dx: i32,
    prev_dy: i32,
    direction: i32,
    max_x: i32,
    min_x: i32,
    // 1 is destroyed, 0 is alive
    status: u32,
}

impl EnemyRow {
    pub fn ship_status(&self, ship_num: i32) -> bool {
        (self.status & (1 << ship_num)) > 0
    }

    pub fn clear_ship(&mut self, ship_num: i32) {
        self.status = self.status & !(1 << ship_num);
    }

    pub fn draw(&self) -> Result<(), core::convert::Infallible> {
        let mut display = Display {};
        let red = PrimitiveStyle::with_fill(Bgr888::RED);
        let black = PrimitiveStyle::with_fill(Bgr888::BLACK);

        for i in 0..self.row_size {
            let alive: bool = self.ship_status(i);

            let base_x: i32 = self.start_x + 2 * self.size * i;
            let base_y: i32 = self.start_y;

            Triangle::new(
                Point::new(base_x, base_y + self.size),
                Point::new(base_x - self.size / 2, base_y),
                Point::new(base_x + self.size / 2, base_y),
            )
            .into_styled(if alive { red } else { black })
            .draw(&mut display)?;

            Triangle::new(
                Point::new(base_x + self.size, base_y + self.size),
                Point::new(base_x + self.size / 2, base_y),
                Point::new(base_x + 3 * self.size / 2, base_y),
            )
            .into_styled(black)
            .draw(&mut display)?;
        }

        Ok(())
    }

    pub fn clear(&self) -> Result<(), core::convert::Infallible> {
        let mut display = Display {};
        let black = PrimitiveStyle::with_fill(Bgr888::BLACK);

        for i in 0..self.row_size {
            let base_x: i32 = self.start_x + (2 * self.size * i) - self.prev_dx;
            let base_y: i32 = self.start_y - self.prev_dy;
            Triangle::new(
                Point::new(base_x, base_y + self.size),
                Point::new(base_x - self.size / 2, base_y),
                Point::new(base_x + self.size / 2, base_y),
            )
            .into_styled(black)
            .draw(&mut display)?;
        }

        Ok(())
    }

    pub fn move_by(&mut self, amount: i32) {
        // are we going to be off the screen
        self.start_x += self.direction * amount;

        self.prev_dx = self.direction * amount;
        self.prev_dy = 0;

        if self.start_x < self.min_x {
            self.start_x += amount;
            self.start_y += self.size;
            self.direction *= -1;

            self.prev_dx = 0;
            self.prev_dy = self.size;
        } else if self.start_x + (2 * self.size * self.row_size) - self.size > self.max_x {
            self.start_x -= amount;
            self.start_y += self.size;
            self.direction *= -1;

            self.prev_dx = 0;
            self.prev_dy = self.size;
        }
    }
}

struct Beam {
    curr_x: i32,
    curr_y: i32,
    width: i32,
    height: i32,
    // 1 is player, -1 is enemy
    player: i32,
    // is the beam on screen
    active: bool,
    prev_dx: i32,
    prev_dy: i32,
    window_height: i32,
}

impl Beam {
    pub fn draw(&self) -> Result<(), core::convert::Infallible> {
        if !self.active {
            return Ok(());
        }
        let mut display = Display {};

        let cyan = PrimitiveStyle::with_fill(Bgr888::CYAN);
        let yellow = PrimitiveStyle::with_fill(Bgr888::YELLOW);

        Rectangle::new(
            Point::new(self.curr_x, self.curr_y),
            Point::new(self.curr_x + self.width, self.curr_y + self.height),
        )
        .into_styled(if self.player == 1 { cyan } else { yellow })
        .draw(&mut display)?;

        Ok(())
    }

    pub fn clear(&self) -> Result<(), core::convert::Infallible> {
        let mut display = Display {};
        let black = PrimitiveStyle::with_fill(Bgr888::BLACK);

        Rectangle::new(
            Point::new(self.curr_x - self.prev_dx, self.curr_y - self.prev_dy),
            Point::new(
                self.curr_x + self.width - self.prev_dx,
                self.curr_y + self.height - self.prev_dy,
            ),
        )
        .into_styled(black)
        .draw(&mut display)?;

        Ok(())
    }

    pub fn move_by(&mut self, amount: i32) {
        if !self.active {
            self.prev_dx = 0;
            self.prev_dy = 0;
            return;
        }

        let delta: i32 = self.player * amount;

        if (self.curr_y - delta) < 0 || (self.curr_y - delta + self.height) > self.window_height {
            self.active = false;
            return;
        }
        self.curr_y -= delta;

        self.prev_dx = 0;
        self.prev_dy = -delta;
    }
}

pub unsafe fn space_invaders() -> Result<(), core::convert::Infallible> {
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

    let mut row1 = EnemyRow {
        row_size: 5,
        start_x: 60,
        start_y: 30,
        size: 30,
        prev_dx: 0,
        prev_dy: 0,
        direction: 1,
        min_x: 60,
        max_x: w - 30,
        status: !0,
    };

    let mut row2 = EnemyRow {
        row_size: 5,
        start_x: 60,
        start_y: 75,
        size: 30,
        prev_dx: 0,
        prev_dy: 0,
        direction: 1,
        min_x: 60,
        max_x: w - 30,
        status: !0,
    };

    let mut beam1 = Beam {
        curr_x: 100,
        curr_y: h - 30,
        width: 10,
        height: 20,
        player: 1,
        active: true,
        prev_dx: 0,
        prev_dy: 0,
        window_height: h,
    };

    let mut beam2 = Beam {
        curr_x: 200,
        curr_y: 75,
        width: 10,
        height: 20,
        player: -1,
        active: true,
        prev_dx: 0,
        prev_dy: 0,
        window_height: h,
    };

    loop {
        row1.clear();
        row1.move_by(4);
        row1.draw();

        row2.clear();
        row2.move_by(4);
        row2.draw();

        ship.clear();
        ship.move_by(dx, 0);
        ship.draw();

        beam1.clear();
        beam1.move_by(5);
        beam1.draw();

        beam2.clear();
        beam2.move_by(5);
        beam2.draw();

        fb::fb_swap_buffer();
        cpu::sleep(170000);
        if (ship.pos_x + 30 > w - 30 && dx > 0) || (ship.pos_x - 30 < 30 && dx < 0) {
            dx *= -1;
        }
    }
}

pub unsafe fn _gl_test() -> Result<(), core::convert::Infallible> {
    fb::fb_init(640, 512, 4, fb::FB_DOUBLEBUFFER);
    let mut display = Display {};

    let cyan = PrimitiveStyle::with_fill(Bgr888::CYAN);
    let magenta = PrimitiveStyle::with_fill(Bgr888::MAGENTA);
    let amber = PrimitiveStyle::with_fill(Bgr888::new(0xff, 0xbf, 0x0));
    let yellow = PrimitiveStyle::with_fill(Bgr888::YELLOW);
    let black = PrimitiveStyle::with_fill(Bgr888::BLACK);

    let w = 640;
    let h = 512;

    for i in 1..60 {
        fb::fb_swap_buffer();

        Rectangle::new(Point::new(0, 0), Point::new(w - 1, h - 1))
            .into_styled(black)
            .draw(&mut display)?;

        Triangle::new(
            Point::new(w / 2 - 200, h / 2 + 200),
            Point::new(w / 2 - 100, h / 2 + 200),
            Point::new(2 - 1, h / 4),
        )
        .into_styled(cyan)
        .draw(&mut display)?;

        Triangle::new(
            Point::new(w / 2 - 200, h / 2 - 100),
            Point::new(w - 1, h - 1),
            Point::new(w / 2 - 100, 0),
        )
        .into_styled(magenta)
        .draw(&mut display)?;

        Triangle::new(
            Point::new(w / 2 - 200, h / 2 + 100),
            Point::new(w / 2, h / 2 + 100),
            Point::new(w / 2 - 100, h / 2 - 100),
        )
        .into_styled(amber)
        .draw(&mut display)?;

        fb::fb_swap_buffer();

        cpu::sleep(15000);
    }

    Ok(())
}
