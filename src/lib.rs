mod platform;

use std::marker::PhantomData;

use platform::EventLoop;

pub trait WindowHandler {
    fn window<'b>(&'b self) -> &'b Window;
    fn window_mut<'b>(&'b mut self) -> &'b mut Window;
    fn on_event(&mut self, event: Event);
    fn on_frame(&mut self);
}

pub struct Window {
    inner: platform::Window,
}

impl Window {
    pub fn new() -> Self {
        Self {
            inner: platform::Window::new(),
        }
    }
}

#[non_exhaustive]
pub enum Event {
    Mouse,
    Keyboard,
    Window,
}

pub fn run_blocking<B, W>(build: B)
where
    B: FnOnce(Window) -> W,
    W: WindowHandler,
{
    let window = Window::new();
    let handler = (build)(window);
    let event_loop = EventLoop::new(handler);
    event_loop.run();
}
