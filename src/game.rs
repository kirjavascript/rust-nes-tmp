use crate::{ppu, sprites, apu};

// statically allocated memory
static mut STATE: Option<Game> = None;
static mut SEED: u16 = 0x8988;

/// do not call this more than once in the same scope (!)
fn state() -> &'static mut Game {
    unsafe { STATE.as_mut().unwrap() }
}

pub fn init() {
    unsafe { STATE = Some(Game::new()); }

    // palettes and border
    ppu::write_bytes(ppu::PAL_BG_0, &[0x0D, 0x36, 0x16, 0x23]);
    ppu::write_bytes(ppu::PAL_SPRITE_0 + 3, &[0x30]);
    ppu::draw_box(1, 1, 30, 28);

    // bricks
    let mut addr = 0x2082;
    state().bricks.chunks(BRICKS_WIDE).for_each(|bricks| {
        ppu::write_addr(addr);
        bricks.iter().for_each(|brick| {
            let tile = brick.as_tile();
            ppu::write_data(tile);
            ppu::write_data(tile + 1);
        });
        addr += 0x20;
    });
}

pub fn frame() {
    let game = state();

    game.step();

    sprites::add(TOP_MARGIN + game.ball.x, LEFT_MARGIN + game.ball.y -1, 0x80, 0);
}

pub fn render() {
    let game = state();

    game.destroyed.iter_mut().for_each(|des| {
        if des.is_some() {
            let index = des.unwrap();
            *des = None;

            let y_addr = (index / BRICKS_WIDE as u8) as u16 * 0x20;
            let x_addr = (index % BRICKS_WIDE as u8) as u16 * 2;
            let addr = 0x2082 + y_addr + x_addr;

            ppu::write_addr(addr);
            ppu::write_data(0);
            ppu::write_data(0);
        }
    });
}

// game logic

fn cycle_rng() {
    unsafe {
        let new_bit = ((SEED >> 9) ^ (SEED >> 1)) & 1;
        SEED = (new_bit << 15) | (SEED >> 1);
    }
}

fn get_rng() -> u8 {
    unsafe { (SEED >> 8) as u8 }
}

const WIDTH: u8 = 224;
const HEIGHT: u8 = 208;
const BRICKS_WIDE: usize = 14;
const TOP_BRICK_MARGIN: usize = 2;
const BALL_DIAMETER: u8 = 6;
const BRICK_WIDTH: u8 = 16;
const BRICK_HEIGHT: u8 = 8;
const LEFT_MARGIN: u8 = 16;
const TOP_MARGIN: u8 = 16;

struct Ball { x: u8, y: u8, dx: i8, dy: i8 }
struct Paddle { x: u8, y: u8, width: u8, height: u8 }

#[derive(Copy, Clone, PartialEq)]
enum Brick { Empty, A, B, C }

impl Brick {
    fn as_tile(&self) -> u8 {
        match self {
            Brick::A => 0x81,
            Brick::B => 0x83,
            Brick::C => 0x85,
            Brick::Empty => 0,
        }
    }
}

struct Game {
    paddle: Paddle,
    ball: Ball,
    bricks: [Brick; 140],
    destroyed: [Option<u8>; 4],
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            ball: Ball { x: 0, y: HEIGHT / 2, dx: 2, dy: -1 },
            paddle: Paddle { x: WIDTH / 2, y: HEIGHT - 10, width: 8, height: 6 },
            bricks: [Brick::Empty; 140],
            destroyed: [None; 4],
        };

        for i in 0..140 {
            cycle_rng();
            game.bricks[i] = match get_rng() % 3 {
                0 => Brick::A,
                1 => Brick::B,
                2 => Brick::C,
                _ => unreachable!(),
            };
        }

        game
    }

    fn step(&mut self) {
        self.ball.x = (self.ball.x as i8 + self.ball.dx) as u8;
        self.ball.y = (self.ball.y as i8 + self.ball.dy) as u8;

        // brick collision
        for (i, brick) in self.bricks.iter_mut().enumerate() {
            if *brick != Brick::Empty {
                let i = i as u8;
                let brick_y = i / BRICKS_WIDE as u8;
                let brick_x = i % BRICKS_WIDE as u8;
                let brick_y = (brick_y * BRICK_HEIGHT) + (TOP_BRICK_MARGIN as u8 * BRICK_HEIGHT);
                let brick_x = (brick_y * BRICK_WIDTH);

                if self.ball.y > brick_y && self.ball.y < brick_y + BRICK_HEIGHT &&
                self.ball.x >= brick_x && self.ball.x <= brick_x + BRICK_WIDTH {
                    *brick = Brick::Empty;

                    let pos = self.destroyed.iter()
                        .position(|&item| item == None)
                        .unwrap_or(0);

                    self.destroyed[pos] = Some(i);
                    self.ball.dy = -self.ball.dy;
                    apu::play_sfx(apu::Sfx::MenuBoop);
                }
            }
        }

        // Screen collision
        if self.ball.x == 0 || self.ball.x + BALL_DIAMETER >= WIDTH {
            self.ball.dx = -self.ball.dx;
            apu::play_sfx(apu::Sfx::Lock);
        }
        if self.ball.y == 0 || self.ball.y + BALL_DIAMETER >= HEIGHT {
            self.ball.dy = -self.ball.dy;
            apu::play_sfx(apu::Sfx::Lock);
        }
    }
}

// struct Brick { x: u8, y: u8, width: u8, height: u8, is_destroyed: bool }
// struct Game { ball: Ball, paddle: Paddle, bricks: Vec<Brick> }

// impl Game {
//     fn new() -> Game {
//         let mut bricks = Vec::new();
//         for i in 0..50 {
//             for j in 0..30 {
//                 bricks.push(Brick { x: i * 8 + 1, y: j * 3 + 1, width: 6, height: 2, is_destroyed: false });
//             }
//         }
//         Game {
//             ball: ,
//             paddle: Paddle { x: 36, y: 48, width: 8, height: 1 },
//             bricks: bricks
//         }
//     }

//     fn update(&mut self) {
//         self.ball.x = (self.ball.x as i8 + self.ball.dx) as u8;
//         self.ball.y = (self.ball.y as i8 + self.ball.dy) as u8;

//         // Paddle collision
//         if self.ball.y >= self.paddle.y && self.ball.y <= self.paddle.y + self.paddle.height &&
//             self.ball.x >= self.paddle.x && self.ball.x <= self.paddle.x + self.paddle.width {
//                 self.ball.dy = -self.ball.dy;
//         }

//         // Brick collision
//         let mut destroyed = HashSet::new();
//         for (i, brick) in self.bricks.iter().enumerate() {
//             if !brick.is_destroyed &&
//                 self.ball.y >= brick.y && self.ball.y <= brick.y + brick.height &&
//                 self.ball.x >= brick.x && self.ball.x <= brick.x + brick.width {
//                     destroyed.insert(i);
//                     self.ball.dy = -self.ball.dy;
//             }
//         }

//         // Remove destroyed bricks
//         for i in destroyed {
//             self.bricks[i].is_destroyed = true;
//         }

//         // Screen collision
//         if self.ball.x == 0 || self.ball.x == 79 {
//             self.ball.dx = -self.ball.dx;
//         }
//         if self.ball.y == 0 {
//             self.ball.dy = -self.ball.dy;
//         }
//     }
// }
