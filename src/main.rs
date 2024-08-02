use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::Stylize,
    terminal::{self, ClearType},
};
use rand::Rng;

struct Snake {
    blocks: Vec<(u16, u16)>,
    direction: Direction,
    dead: bool,
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const FRAME_DURATON: Duration = Duration::from_millis(200);
const HEIGHT: u16 = 21;
const WIDTH: u16 = 35;

impl Snake {
    fn new() -> Self {
        Self {
            blocks: vec![(10, 10), (10, 9), (10, 8)],
            direction: Direction::Up,
            dead: false,
        }
    }

    fn move_forward(&mut self) {
        let head = self.blocks.first().clone().unwrap();
        let new_head = match self.direction {
            Direction::Down => (head.0, head.1 + 1),
            Direction::Up => (head.0, head.1 - 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };

        if (new_head.0 >= WIDTH) | (new_head.0 == 0) | (new_head.1 >= HEIGHT) | (new_head.1 == 0) {
            self.dead = true
        }

        self.blocks.insert(0, new_head);
        self.blocks.pop();
    }

    fn change_direction(&mut self, new_direction: Direction) {
        match (self.direction.clone(), new_direction.clone()) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => (),
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => (),
            _ => self.direction = new_direction,
        }
    }

    fn handle_keydown(&mut self, event: KeyEvent) {
        if event.kind == KeyEventKind::Press {
            match event.code {
                KeyCode::Left => self.change_direction(Direction::Left),
                KeyCode::Right => self.change_direction(Direction::Right),
                KeyCode::Up => self.change_direction(Direction::Up),
                KeyCode::Down => self.change_direction(Direction::Down),
                _ => (),
            }
        }
    }

    fn grow(&mut self) {
        let x = self.blocks.last().unwrap().0;
        let y = self.blocks.last().unwrap().1;

        let cords = match self.direction {
            Direction::Down => (x, y - 1),
            Direction::Up => (x, y + 1),
            Direction::Left => (x + 1, y),
            Direction::Right => (x - 1, y),
        };
        self.blocks.push(cords)
    }

    fn detect_apple(&mut self, mut apples: Vec<(u16, u16)>) -> Vec<(u16, u16)> {
        for (i, apple) in apples.clone().iter().enumerate() {
            if (self.blocks.first().unwrap().0 == apple.0)
                && (self.blocks.first().unwrap().1 == apple.1)
            {
                self.grow();
                apples.remove(i);
                apples.push(draw_apple());
            }
        }

        apples
    }
}

fn game_loop() {
    let mut snake = Snake::new();
    let mut stdout = stdout();

    let mut game_over: bool = false;
    execute!(std::io::stdout()).unwrap();

    let mut apples: Vec<(u16, u16)> = vec![draw_apple()];
    while !game_over {
        execute!(stdout, terminal::Clear(ClearType::All)).unwrap();
        draw_board();
        draw_prev_apples(&apples);
        draw_snake(&snake);
        snake.move_forward();
        apples = snake.detect_apple(apples);

        if snake.dead {
            execute!(stdout, terminal::Clear(ClearType::All)).unwrap();
            game_over = true;
        }

        if poll(FRAME_DURATON).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    if event.code == KeyCode::Esc {
                        execute!(stdout, terminal::Clear(ClearType::All)).unwrap();
                        game_over = true;
                        // break;
                    } else {
                        snake.handle_keydown(event)
                    }
                }
                _ => (),
            }
        }
    }
}

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, cursor::Hide).unwrap();
    loop {
        println!("");
        println!("{}", "Press any key to play again".green());
        println!("{}", "Press ESC to quit".blue());

        let mut play = false;
        match read().unwrap() {
            Event::Key(event) => {
                if event.code == KeyCode::Esc {
                    break;
                } else {
                    play = true
                }
            }
            _ => play = false,
        }
        if play {
            game_loop();
        }
    }

    execute!(stdout, cursor::Show).unwrap();
    stdout.flush().unwrap();
}

fn draw_snake(snake: &Snake) {
    let mut stdout = stdout();

    for block in &snake.blocks {
        execute!(stdout, cursor::MoveTo(block.0, block.1)).unwrap();
        print!("*");
    }

    stdout.flush().unwrap();
}

fn draw_apple() -> (u16, u16) {
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();

    let rand_x = rng.gen_range(1..WIDTH - 2);
    let rand_y = rng.gen_range(1..HEIGHT - 2);

    execute!(stdout, cursor::MoveTo(rand_x, rand_y)).unwrap();
    print!("{}", "*".red());

    stdout.flush().unwrap();

    (rand_x, rand_y)
}

fn draw_prev_apples(apples: &Vec<(u16, u16)>) {
    let mut stdout = stdout();

    for apple in apples {
        execute!(stdout, cursor::MoveTo(apple.0, apple.1)).unwrap();
        print!("{}", "*".red());
    }

    stdout.flush().unwrap();
}

fn draw_board() {
    let mut stdout = stdout();

    for i in 0..WIDTH {
        execute!(stdout, cursor::MoveTo(i, 0)).unwrap();
        print!("{}", "_".white())
    }

    for i in 0..WIDTH {
        execute!(stdout, cursor::MoveTo(i, HEIGHT)).unwrap();
        print!("{}", "_".white())
    }

    for i in 0..HEIGHT {
        execute!(stdout, cursor::MoveTo(0, i)).unwrap();
        print!("{}", "|".white())
    }

    for i in 0..HEIGHT {
        execute!(stdout, cursor::MoveTo(WIDTH, i)).unwrap();
        print!("{}", "|".white())
    }

    stdout.flush().unwrap();
}
