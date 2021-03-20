/*
 * Implementation of traits necessary to use the embedded-graphics
 * crate for drawing shapes on a screen.
 *
 * Author: Ashish Rao <aprao@stanford.edu>
 */

use crate::{cpu, fb};
use core::convert::TryInto;

use embedded_graphics::{
    egtext,
    fonts::Font24x32,
    pixelcolor::Bgr888,
    prelude::*,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::PrimitiveStyle,
    text_style, DrawTarget,
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

pub unsafe fn _gl_test() -> Result<(), core::convert::Infallible> {
    // TODO make non-public
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

/*#[test_case]
pub fn test_gl() {
    unsafe {
        space_invaders();
        //_gl_test();
    }
}*/
