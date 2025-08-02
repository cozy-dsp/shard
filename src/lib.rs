mod platform;

use std::{
    marker::PhantomData,
    rc::{Rc, Weak},
};

use platform::EventLoop;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub trait WindowHandler {
    fn window<'b>(&'b self) -> &'b Window;
    fn window_mut<'b>(&'b mut self) -> &'b mut Window;
    fn on_event(&mut self, event: Event);
    fn on_frame(&mut self);
}

#[derive(Clone)]
pub struct Window {
    inner: Rc<platform::Window>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(platform::Window::new()),
        }
    }
}

impl HasWindowHandle for Window {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.inner.window_handle()
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.inner.display_handle()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard,
    Window,
}

#[derive(Debug)]
pub enum MouseEvent {
    Moved { x: u16, y: u16 },
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
