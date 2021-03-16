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

struct Display {}

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
    size: u32,
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
        let yellow = PrimitiveStyle::with_fill(Bgr888::YELLOW);
        Circle::new(Point::new(self.pos_x, self.pos_y), self.size)
            .into_styled(yellow)
            .draw(&mut display)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), core::convert::Infallible> {
        let mut display = Display {};
        let black = PrimitiveStyle::with_fill(Bgr888::BLACK);
        if self.second_set {
            Circle::new(
                Point::new(self.pos_x - self.prev_dx, self.pos_y - self.prev_dy),
                self.size,
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

    loop {
        ship.clear();
        ship.move_by(dx, 0);
        ship.draw();
        fb::fb_swap_buffer();
        cpu::sleep(120000);
        if (ship.pos_x + 30 > w - 30 && dx > 0) || 
            (ship.pos_x - 30 < 30 && dx < 0) {
            dx *= -1
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

        /*Rectangle::new(Point::new(0, 0), Point::new(w - 1, h - 1))
        .into_styled(black)
        .draw(&mut display)?;*/

        Circle::new(Point::new(50 + 5 * (i - 1), 50 + 5 * (i - 1)), 40)
            .into_styled(black)
            .draw(&mut display)?;

        Circle::new(Point::new(50 + 5 * i, 50 + 5 * i), 40)
            .into_styled(yellow)
            .draw(&mut display)?;

        /*Triangle::new(
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
        .draw(&mut display)?;*/

        fb::fb_swap_buffer();

        Circle::new(Point::new(50 + 5 * (i - 1), 50 + 5 * (i - 1)), 40)
            .into_styled(black)
            .draw(&mut display)?;

        Circle::new(Point::new(50 + 5 * i, 50 + 5 * i), 40)
            .into_styled(yellow)
            .draw(&mut display)?;

        cpu::sleep(15000);
    }

    Ok(())
}
