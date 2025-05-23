use std::time::{Duration, Instant};

use glutin_window::{GlutinWindow, OpenGL};
use opengl_graphics::GlGraphics;
use piston::{
    Button, EventSettings, Events, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs,
    UpdateEvent, WindowSettings,
};

use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("lasalgo", [1920, 1080])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let values: Vec<f64> = (0..1000).map(|x| x as f64).collect();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        values,
        sorted: false,
        i: 0,
        j: 0,
        sort_started: None,
        sort_duration: None,
        algorithm: SortAlgorithm::BubbleSort,
    };

    app.scramble_values();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::D1 => {
                    println!("Running bubble sort");
                    app.set_algorithm(SortAlgorithm::BubbleSort);
                }
                Key::D2 => {
                    println!("Running quick sort");
                    app.set_algorithm(SortAlgorithm::QuickSort);
                }
                _ => {}
            }
        }
    }
}

struct App {
    gl: GlGraphics,
    values: Vec<f64>,
    sorted: bool,
    i: usize,
    j: usize,
    sort_started: Option<Instant>,
    sort_duration: Option<Duration>,
    algorithm: SortAlgorithm,
}

enum SortAlgorithm {
    BubbleSort,
    QuickSort,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let transform = c.transform;
            let screen_height = args.window_size[1];

            for (x, &length) in self.values.iter().enumerate() {
                let color = if x == self.j {
                    color::RED
                } else {
                    color::WHITE
                };

                let x: f64 = x as f64;
                let y_start = screen_height;
                let y_end = screen_height - length;

                graphics::line(color, 1.0, [x, y_start, x, y_end], transform, gl);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.sort_step(100);
    }

    fn set_algorithm(&mut self, algorithm: SortAlgorithm) {
        self.algorithm = algorithm;
        self.sorted = false;
        self.i = 0;
        self.j = 0;
        self.sort_started = None;
        self.sort_duration = None;
        self.values = (0..1000).map(|x| x as f64).collect();
        self.scramble_values();
    }

    fn sort_step(&mut self, steps: usize) {
        if self.sorted {
            return;
        }

        match self.algorithm {
            SortAlgorithm::BubbleSort => self.bubble_sort(steps),
            SortAlgorithm::QuickSort => self.quick_sort(steps),
        }
    }

    fn scramble_values(&mut self) {
        self.values.shuffle(&mut thread_rng());
    }

    fn bubble_sort(&mut self, steps: usize) {
        if self.sorted {
            return;
        }

        if self.sort_started.is_none() {
            self.sort_started = Some(Instant::now());
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
                self.sorted = true;
                self.sort_duration = self.sort_started.map(|start| start.elapsed());
                if let Some(duration) = self.sort_duration {
                    println!("Bubble Sort completed in {:.2?}", duration);
                }
                break;
            }
        }
    }

    fn quick_sort(&mut self, steps: usize) {
        if self.sorted {
            return;
        }

        if self.sort_started.is_none() {
            self.sort_started = Some(Instant::now());
        }

        for _ in 0..steps {}
    }
}
