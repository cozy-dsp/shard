use std::io::Read;

use shard::{Window, WindowHandler};

pub struct MyApp {
    window: Window,
}

impl WindowHandler for MyApp {
    fn window<'b>(&'b self) -> &'b Window {
        &self.window
    }

    fn window_mut<'b>(&'b mut self) -> &'b mut Window {
        &mut self.window
    }

    fn on_event(&mut self, event: shard::Event) {
        //todo!()
    }

    fn on_frame(&mut self) {
        //todo!()
    }
}

fn main() {
    shard::run_blocking(|window| MyApp { window });
}
