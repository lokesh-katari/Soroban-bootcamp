use std::fmt::{Display, Error, Formatter};

enum VertDir {
    Up,
    Down,
}
enum HorizDir {
    Left,
    Right,
}
struct Ball {
    x: i32,
    y: i32,
    vert_dir: VertDir,
    hor_dir: HorizDir,
}

struct Frame {
    width: i32,
    height: i32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        Game {
            frame: Frame {
                width: 60,
                height: 30,
            },
            ball: Ball {
                x: 0,
                y: 0,
                vert_dir: VertDir::Down,
                hor_dir: HorizDir::Right,
            },
        }
    }
    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x <= 0 {
            self.hor_dir = HorizDir::Right;
        } else if frame.width <= self.x {
            self.hor_dir = HorizDir::Left;
        } else if self.y <= 0 {
            self.vert_dir = VertDir::Down;
        } else if frame.height <= self.y {
            self.vert_dir = VertDir::Up;
        } else {
        }
    }
    fn mv(&mut self) {
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
        match self.hor_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "x")?;

        // Print top border
        for _ in 0..64 {
            write!(fmt, "-")?;
        }
        write!(fmt, "\n")?;

        // Print game board
        for y in 0..32 {
            for x in 0..64 {
                if self.ball.x == x as i32 && self.ball.y == y as i32 {
                    write!(fmt, "0")?;
                } else if x == 0 {
                    write!(fmt, "|")?;
                } else {
                    write!(fmt, " ")?;
                }
            }
            // Print right border and move to the next line
            write!(fmt, "|\n")?;
        }

        // Print bottom border
        for _ in 0..64 {
            write!(fmt, "-")?;
        }
        write!(fmt, "\n")?;

        Ok(())
    }
}

fn main() {
    let mut game = Game::new();
    let sleep_time = std::time::Duration::from_millis(100);
    loop {
        println!("{}", game);
        game.step();
        std::thread::sleep(sleep_time);
        println!("{} {} ", game.ball.x, game.ball.y)
    }
}
