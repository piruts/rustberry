#[path = "fb.rs"]
mod fb;

use core::convert::TryInto;

use embedded_graphics::{
    pixelcolor::Bgr888,
    prelude::*,
    primitives::{Circle, Triangle},
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
        _gl_test();
    }
}

pub unsafe fn _gl_test() -> Result<(), core::convert::Infallible> {
    fb::fb_init(640, 512, 4, fb::FB_DOUBLEBUFFER);
    let mut display = Display {};

    let cyan = PrimitiveStyle::with_fill(Bgr888::CYAN);
    let magenta = PrimitiveStyle::with_fill(Bgr888::MAGENTA);
    let amber = PrimitiveStyle::with_fill(Bgr888::new(0xff, 0xbf, 0x0));
    let yellow = PrimitiveStyle::with_fill(Bgr888::YELLOW);

    Circle::new(Point::new(50, 50), 40)
        .into_styled(yellow)
        .draw(&mut display)?;

    let w = 640;
    let h = 512;

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
    Ok(())
}
