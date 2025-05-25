use glutin_window::{GlutinWindow, OpenGL};
use lasalgo::app::{Algorithm, App};
use opengl_graphics::GlGraphics;
use piston::{
    Button, EventSettings, Events, Key, PressEvent, RenderEvent, UpdateEvent, WindowSettings,
};
use rand::{Rng, distributions::Uniform, thread_rng};

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("lasalgo", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let values: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let low = 0;
    let high = values.len();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        values,
        search: usize::MAX,
        search_result: None,
        completed: false,
        i: 0,
        j: 0,
        low,
        high,
        algo_started: None,
        algo_duration: None,
        algorithm: Algorithm::BubbleSort,
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
                    app.set_algorithm(Algorithm::BubbleSort);
                    app.scramble_values();
                }
                Key::D2 => {
                    println!("Running linear search");
                    let mut rng = thread_rng();
                    app.search = rng.sample(Uniform::new(0usize, 100));
                    println!("Searching for {:?}", app.search);
                    app.set_algorithm(Algorithm::LinearSearch);
                }
                Key::D3 => {
                    println!("Running binary search");
                    let mut rng = thread_rng();
                    app.search = rng.sample(Uniform::new(0usize, 100));
                    println!("Searching for {:?}", app.search);
                    app.set_algorithm(Algorithm::BinarySearch);
                }
                _ => {}
            }
        }
    }
}
