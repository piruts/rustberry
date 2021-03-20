/*
 * Space invaders in Rust! With the included Makefile, can be run with 
 * make run.
 *
 * Authors:
 * - Ashish Rao <aprao@stanford.edu>
 * - Flynn Dreilinger 
 * - Xiluo He
 */

use crate::gl::Display;
use crate::{cpu, fb, gl, keyboard};
use core::convert::TryInto;

use core::cell::UnsafeCell;

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

    pub unsafe fn ship_hit(&self, b: &Beam) -> i32 {
        if !((self.start_y <= *(b.curr_y.get()))
            && (*(b.curr_y.get()) <= (self.start_y + self.size)))
        {
            return -1;
        }
        for i in 0..self.row_size {
            let alive: bool = self.ship_status(i);
            if !alive {
                continue;
            }
            let base_x: i32 = self.start_x + 2 * self.size * i;
            if (base_x - self.size / 2 <= *(b.curr_x.get()))
                && (*(b.curr_x.get()) <= base_x + self.size / 2)
            {
                return i;
            }
        }
        return -1;
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
    curr_x: UnsafeCell<i32>,
    curr_y: UnsafeCell<i32>,
    width: UnsafeCell<i32>,
    height: UnsafeCell<i32>,
    // 1 is player, -1 is enemy
    player: UnsafeCell<i32>,
    // is the beam on screen
    active: UnsafeCell<bool>,
    prev_dx: UnsafeCell<i32>,
    prev_dy: UnsafeCell<i32>,
    window_height: UnsafeCell<i32>,
    available: UnsafeCell<bool>,
    //enemy_row1: &'a EnemyRow,
    //enemy_row2: &'a EnemyRow,
}

impl Beam {
    pub unsafe fn draw(&self) -> Result<(), core::convert::Infallible> {
        if !*(self.active.get()) {
            return Ok(());
        }
        let mut display = Display {};

        let cyan = PrimitiveStyle::with_fill(Bgr888::CYAN);
        let yellow = PrimitiveStyle::with_fill(Bgr888::YELLOW);

        Rectangle::new(
            Point::new(*(self.curr_x.get()), *(self.curr_y.get())),
            Point::new(
                *(self.curr_x.get()) + *(self.width.get()),
                *(self.curr_y.get()) + *(self.height.get()),
            ),
        )
        .into_styled(if *(self.player.get()) == 1 {
            cyan
        } else {
            yellow
        })
        .draw(&mut display)?;

        Ok(())
    }

    pub unsafe fn clear(&self) -> Result<(), core::convert::Infallible> {
        let mut display = Display {};
        let black = PrimitiveStyle::with_fill(Bgr888::BLACK);

        Rectangle::new(
            Point::new(
                *(self.curr_x.get()) - *(self.prev_dx.get()),
                *(self.curr_y.get()) - *(self.prev_dy.get()),
            ),
            Point::new(
                *(self.curr_x.get()) + *(self.width.get()) - *(self.prev_dx.get()),
                *(self.curr_y.get()) + *(self.height.get()) - *(self.prev_dy.get()),
            ),
        )
        .into_styled(black)
        .draw(&mut display)?;

        Ok(())
    }

    pub unsafe fn move_by(&self, amount: i32) {
        if !*(self.active.get()) {
            *(self.prev_dx.get()) = 0;
            *(self.prev_dy.get()) = 0;
            return;
        }

        let delta: i32 = *(self.player.get()) * amount;

        if (*(self.curr_y.get()) - delta) < 0
            || (*(self.curr_y.get()) - delta + *(self.height.get())) > *(self.window_height.get())
        {
            *(self.active.get()) = false;
            return;
        }
        *(self.curr_y.get()) -= delta;

        *(self.prev_dx.get()) = 0;
        *(self.prev_dy.get()) = -delta;

        /*if *(self.player.get()) == 1 {
            let hit_ship: i32 = self.enemy_row2.ship_hit(self);
            if hit_ship != -1 {
                *(self.active.get()) = false;
                self.enemy_row2.clear_ship(hit_ship);
                return;
            }

            let hit_ship: i32 = self.enemy_row1.ship_hit(self);
            if hit_ship != -1 {
                *(self.active.get()) = false;
                self.enemy_row1.clear_ship(hit_ship);
                return;
            }
        }*/
    }
}

pub unsafe fn run_game() -> Result<(), core::convert::Infallible> {
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

    let beam1 = Beam {
        curr_x: UnsafeCell::new(100 as i32),
        curr_y: UnsafeCell::new(h - 30 as i32),
        width: UnsafeCell::new(10 as i32),
        height: UnsafeCell::new(20 as i32),
        player: UnsafeCell::new(1 as i32),
        active: UnsafeCell::new(false as bool),
        prev_dx: UnsafeCell::new(0 as i32),
        prev_dy: UnsafeCell::new(0 as i32),
        window_height: UnsafeCell::new(h as i32),
        available: UnsafeCell::new(true as bool),
        //enemy_row1: &row1,
        //enemy_row2: &row2,
    };

    let beam2 = Beam {
        curr_x: UnsafeCell::new(200 as i32),
        curr_y: UnsafeCell::new(75 as i32),
        width: UnsafeCell::new(10 as i32),
        height: UnsafeCell::new(20 as i32),
        player: UnsafeCell::new(-1 as i32),
        active: UnsafeCell::new(false as bool),
        prev_dx: UnsafeCell::new(0 as i32),
        prev_dy: UnsafeCell::new(0 as i32),
        window_height: UnsafeCell::new(h as i32),
        available: UnsafeCell::new(true as bool),
        //enemy_row1: &row1,
        //enemy_row2: &row2,
    };

    let beam3 = Beam {
        curr_x: UnsafeCell::new(200 as i32),
        curr_y: UnsafeCell::new(75 as i32),
        width: UnsafeCell::new(10 as i32),
        height: UnsafeCell::new(20 as i32),
        player: UnsafeCell::new(-1 as i32),
        active: UnsafeCell::new(false as bool),
        prev_dx: UnsafeCell::new(0 as i32),
        prev_dy: UnsafeCell::new(0 as i32),
        window_height: UnsafeCell::new(h as i32),
        available: UnsafeCell::new(true as bool),
        //enemy_row1: &row1,
        //enemy_row2: &row2,
    };

    let beam4 = Beam {
        curr_x: UnsafeCell::new(200 as i32),
        curr_y: UnsafeCell::new(75 as i32),
        width: UnsafeCell::new(10 as i32),
        height: UnsafeCell::new(20 as i32),
        player: UnsafeCell::new(-1 as i32),
        active: UnsafeCell::new(false as bool),
        prev_dx: UnsafeCell::new(0 as i32),
        prev_dy: UnsafeCell::new(0 as i32),
        window_height: UnsafeCell::new(h as i32),
        available: UnsafeCell::new(true as bool),
        //enemy_row1: &row1,
        //enemy_row2: &row2,
    };

    let beam5 = Beam {
        curr_x: UnsafeCell::new(200 as i32),
        curr_y: UnsafeCell::new(75 as i32),
        width: UnsafeCell::new(10 as i32),
        height: UnsafeCell::new(20 as i32),
        player: UnsafeCell::new(-1 as i32),
        active: UnsafeCell::new(false as bool),
        prev_dx: UnsafeCell::new(0 as i32),
        prev_dy: UnsafeCell::new(0 as i32),
        window_height: UnsafeCell::new(h as i32),
        available: UnsafeCell::new(true as bool),
        //enemy_row1: &row1,
        //enemy_row2: &row2,
    };

    let beam_arr: [Beam; 5] = [beam1, beam2, beam3, beam4, beam5];

    loop {
        row1.clear();
        row1.move_by(20);
        row1.draw();

        row2.clear();
        row2.move_by(20);
        row2.draw();

        //ship.clear();
        let black = PrimitiveStyle::with_fill(Bgr888::BLACK);
        let mut display = Display {};

        Rectangle::new(Point::new(0, h - 60), Point::new(w, h - 30))
            .into_styled(black)
            .draw(&mut display)?;

        if keyboard::read_next() == 'h' && ship.pos_x - 5 > 60 {
            //keyboard::PS2_KEY_ARROW_LEFT {
            ship.move_by(-20, 0);
        } else if keyboard::read_next() == 'l' && ship.pos_x + 5 < w - 60 {
            // keyboard::PS2_KEY_ARROW_RIGHT {
            ship.move_by(20, 0);
        } else if keyboard::read_next() == 'k' {
            // try to find a beam that is not active
            for beam in &beam_arr {
                if *(beam.available.get()) {
                    beam.clear();
                    *(beam.curr_x.get()) = ship.pos_x;
                    *(beam.curr_y.get()) = ship.pos_y - ship.size;
                    *(beam.player.get()) = 1;
                    *(beam.prev_dx.get()) = 0;
                    *(beam.prev_dy.get()) = 0;
                    *(beam.active.get()) = true;
                    *(beam.available.get()) = false;
                    break;
                }
            }
        } else if keyboard::read_next() == 'p' {
            while keyboard::read_next() != 'r' {}
        }

        ship.draw();

        for beam in &beam_arr {
            beam.clear();
            if *(beam.active.get()) {
                beam.move_by(20);
                beam.draw();

                if *(beam.player.get()) == 1 {
                    let mut hit_ship: i32 = row2.ship_hit(&beam);
                    if hit_ship != -1 {
                        *(beam.active.get()) = false;
                        row2.clear_ship(hit_ship);
                        continue;
                    }

                    hit_ship = row1.ship_hit(&beam);
                    if hit_ship != -1 {
                        *(beam.active.get()) = false;
                        row1.clear_ship(hit_ship);
                    }
                }
            } else {
                if *(beam.prev_dy.get()) == 0 {
                    *(beam.available.get()) = true;
                } else {
                    *(beam.prev_dy.get()) = 0;
                    *(beam.available.get()) = false;
                }
            }
        }

        if row2.start_y + row2.size >= ship.pos_y - ship.size {
            let mut display = Display {};
            egtext!(
                text = "Game over!",
                top_left = ((w / 2) - (5 * 24), (h / 2) - 16),
                style = text_style!(
                    font = Font24x32,
                    text_color = Bgr888::YELLOW,
                    background_color = Bgr888::BLACK
                )
            )
            .draw(&mut display)?;
            fb::fb_swap_buffer();

            return Ok(());
        }

        fb::fb_swap_buffer();
        cpu::sleep(500000);
        /*if (ship.pos_x + 30 > w - 30 && dx > 0) || (ship.pos_x - 30 < 30 && dx < 0) {
            dx *= -1;
        }*/
    }
}
