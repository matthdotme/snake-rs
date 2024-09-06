// src/game.rs
use rand::Rng;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, PartialEq)]
pub struct Snake {
    pub body: VecDeque<Point>,
    pub direction: Direction,
    pub boundary_x: usize,
    pub boundary_y: usize,
}

impl Snake {
    pub fn new(starting_point: Point, width: usize, height: usize) -> Self {
        let mut body = VecDeque::new();
        body.push_back(starting_point);
        Snake {
            body,
            direction: Direction::Right,
            boundary_x: height,
            boundary_y: width,
        }
    }

    pub fn move_forward(&mut self) {
        let head = self.body.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Point { x: head.x, y: {
                if head.y == 0 {
                    self.boundary_y
                } else {
                    head.y.saturating_sub(1)
                }
                }, },
            Direction::Down => Point { x: head.x, y: (head.y + 1) % self.boundary_y },
            Direction::Left => Point { x: {
                if head.x == 0 {
                    self.boundary_x
                } else {
                    head.x.saturating_sub(1)
                }
            }, y: head.y },
            Direction::Right => Point { x: (head.x + 1) % self.boundary_x, y: head.y },
        };

        self.body.push_front(new_head);
        self.body.pop_back();
    }

    pub fn grow(&mut self) {
        let tail = self.body.back().unwrap().clone();
        self.body.push_back(tail);
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if self.direction == Direction::Up && new_direction == Direction::Down
            || self.direction == Direction::Down && new_direction == Direction::Up
            || self.direction == Direction::Left && new_direction == Direction::Right
            || self.direction == Direction::Right && new_direction == Direction::Left {
                return;
            }
        self.direction = new_direction;
    }

    pub fn check_collision(&self) -> bool {
        let head = self.body.front().unwrap();
        self.body.iter().skip(1).any(|&point| point == *head)
    }
}

#[derive(Clone, PartialEq)]
pub struct Game {
    pub snake: Snake,
    pub food: Point,
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let starting_point = Point { x: width / 2, y: height / 2 };
        let snake = Snake::new(starting_point, width, height);
        let food = Game::generate_food(&snake, width, height);
        Game { snake, food, width, height }
    }

    pub fn generate_food(snake: &Snake, width: usize, height: usize) -> Point {
        let mut rng = rand::thread_rng();
        loop {
            let food = Point {
                x: rng.gen_range(0..width),
                y: rng.gen_range(0..height),
            };
            if !snake.body.contains(&food) {
                return food;
            }
        }
    }

    pub fn update(&mut self) {
        self.snake.move_forward();
        if self.snake.body.front().unwrap() == &self.food {
            self.snake.grow();
            self.food = Game::generate_food(&self.snake, self.width, self.height);
        }
    }

    pub fn is_game_over(&self) -> bool {
        // let head = self.snake.body.front().unwrap();
        self.snake.check_collision()
        // head.x >= self.width || head.y >= self.height || self.snake.check_collision()
    }
}
