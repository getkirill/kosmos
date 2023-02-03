use core::time;
use std::thread;

use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    println!("Kosmos - branch {} - commit {:.6}", env!("VERGEN_GIT_BRANCH"), env!("VERGEN_GIT_SHA"));
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Kosmos").with_visible(true).build(&event_loop).expect("Aww man that shucks!");
    thread::sleep(time::Duration::from_secs(5));
    // PEACE!
}
