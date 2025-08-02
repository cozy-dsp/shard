use std::{io::Read, num::NonZero};

use shard::{Window, WindowHandler};
use softbuffer::{Context, Surface};

pub struct MyApp {
    window: Window,
    context: Context<Window>,
    surface: Surface<Window, Window>,
}

impl WindowHandler for MyApp {
    fn window<'b>(&'b self) -> &'b Window {
        &self.window
    }

    fn window_mut<'b>(&'b mut self) -> &'b mut Window {
        &mut self.window
    }

    fn on_event(&mut self, event: shard::Event) {
        println!("{event:?}");
    }

    fn on_frame(&mut self) {
        self.surface.buffer_mut().unwrap().fill(0xFF0000);
        self.surface.buffer_mut().unwrap().present().unwrap();
    }
}

fn main() {
    shard::run_blocking(|window| {
        let context = Context::new(window.clone()).unwrap();
        let mut surface = Surface::new(&context, window.clone()).unwrap();
        surface
            .resize(NonZero::new(512).unwrap(), NonZero::new(512).unwrap())
            .unwrap();

        MyApp {
            window,
            context,
            surface,
        }
    });
}
