use std::time::{Duration, Instant};

use num_traits::ToPrimitive;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use rand::{seq::SliceRandom, thread_rng};

pub struct App {
    pub gl: GlGraphics,
    pub values: Vec<f64>,
    pub completed: bool,
    pub search: usize,
    pub search_result: Option<usize>,
    pub i: usize,
    pub j: usize,
    pub low: usize,
    pub high: usize,
    pub algo_started: Option<Instant>,
    pub algo_duration: Option<Duration>,
    pub algorithm: Algorithm,
}

pub enum Algorithm {
    BubbleSort,
    BinarySearch,
    LinearSearch,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let transform = c.transform;
            let screen_height = args.window_size[1];

            for (x, &length) in self.values.iter().enumerate() {
                let color = if Some(x) == self.search_result {
                    color::GREEN
                } else if x == self.j {
                    color::RED
                } else {
                    color::WHITE
                };

                let scale = 6.0;
                let x: f64 = x as f64 * scale;
                let length = length * scale;
                let y_start = screen_height;
                let y_end = screen_height - length;

                graphics::line(color, 1.0, [x, y_start, x, y_end], transform, gl);
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        self.step(1);
    }

    pub fn set_algorithm(&mut self, algorithm: Algorithm) {
        self.algorithm = algorithm;
        self.completed = false;
        self.values = (0..100).map(|x| x as f64).collect();
        self.algo_started = None;
        self.algo_duration = None;
        self.search_result = None;
        self.i = 0;
        self.j = 0;
        self.low = 0;
        self.high = self.values.len();
    }

    fn step(&mut self, steps: usize) {
        if self.completed {
            return;
        }

        match self.algorithm {
            Algorithm::BubbleSort => self.bubble_sort(steps),
            Algorithm::BinarySearch => self.binary_search(steps),
            Algorithm::LinearSearch => self.linear_search(steps),
        }
    }

    pub fn scramble_values(&mut self) {
        self.values.shuffle(&mut thread_rng());
    }

    fn bubble_sort(&mut self, steps: usize) {
        if self.completed {
            return;
        }

        if self.algo_started.is_none() {
            self.algo_started = Some(Instant::now());
        }

        for _ in 0..steps {
            if self.i < self.values.len() {
                if self.j < self.values.len() - self.i - 1 {
                    if self.values[self.j] > self.values[self.j + 1] {
                        self.values.swap(self.j, self.j + 1);
                    }
                    self.j += 1;
                } else {
                    self.j = 0;
                    self.i += 1;
                }
            } else {
                self.completed = true;
                self.algo_duration = self.algo_started.map(|start| start.elapsed());
                if let Some(duration) = self.algo_duration {
                    println!("Bubble Sort completed in {:.2?}", duration);
                }
                break;
            }
        }
    }

    fn linear_search(&mut self, steps: usize) {
        if self.completed {
            return;
        }

        if self.algo_started.is_none() {
            self.algo_started = Some(Instant::now());
        }

        let needle = self.search.to_f64().unwrap();

        for _ in 0..steps {
            if self.j < self.values.len() {
                if self.values[self.j] == needle {
                    self.completed = true;
                    self.search_result = Some(self.j);
                    self.algo_duration = self.algo_started.map(|start| start.elapsed());
                    if let Some(duration) = self.algo_duration {
                        println!(
                            "Linear Search found {} at index {} in {:.2?}",
                            needle, self.j, duration
                        );
                    }
                    break;
                } else {
                    self.j += 1;
                }
            } else {
                self.completed = true;
                self.search_result = Some(self.j);
                self.algo_duration = self.algo_started.map(|start| start.elapsed());
                if let Some(duration) = self.algo_duration {
                    println!("Linear Search completed in {:.2?}", duration);
                }
                break;
            }
        }
    }

    fn binary_search(&mut self, steps: usize) {
        if self.completed {
            return;
        }

        if self.algo_started.is_none() {
            self.algo_started = Some(Instant::now());
        }

        let needle = self.search.to_f64().unwrap();

        for _ in 0..steps {
            if self.low >= self.high {
                self.completed = true;
                self.search_result = Some(self.j);
                self.algo_duration = self.algo_started.map(|start| start.elapsed());
                if let Some(duration) = self.algo_duration {
                    println!("Binary Search completed in {:.2?}", duration);
                }
                break;
            }
            let middle = self.low + (self.high - self.low) / 2;
            self.j = middle;
            let value = self.values[middle];

            if value == needle {
                self.completed = true;
                self.search_result = Some(self.j);
                self.algo_duration = self.algo_started.map(|start| start.elapsed());
                if let Some(duration) = self.algo_duration {
                    println!("Binary Search completed in {:.2?}", duration);
                }
                return;
            } else if value < needle {
                self.low = middle + 1;
            } else {
                self.high = middle + 1;
            }
        }
    }
}
