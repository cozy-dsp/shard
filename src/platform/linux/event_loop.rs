use std::{
    os::fd::{AsFd, AsRawFd},
    time::Duration,
};

use nix::{
    poll::{PollFd, PollFlags, poll},
    sys::select::select,
};
use x11rb::connection::Connection;

use crate::WindowHandler;

pub(crate) struct EventLoop<W: WindowHandler> {
    handler: W,
    event_loop_running: bool,
}

impl<W: WindowHandler> EventLoop<W> {
    pub fn new(handler: W) -> Self {
        Self {
            handler,
            event_loop_running: true,
        }
    }

    pub fn run(mut self) {
        while self.event_loop_running {
            let xcb_fd = self.handler.window().inner.connection.as_fd();
            let mut poll_fd = [PollFd::new(xcb_fd, PollFlags::POLLIN)];
            poll(&mut poll_fd, 5 as u16).unwrap();
            poll_fd[0].revents();

            while let Some(event) = self
                .handler
                .window()
                .inner
                .connection
                .poll_for_event()
                .unwrap()
            {
                match event {
                    x11rb::protocol::Event::ClientMessage(client_message_event)
                        if client_message_event.format == 32
                            && client_message_event.data.as_data32()[0]
                                == self.handler.window().inner.atoms.WM_DELETE_WINDOW =>
                    {
                        self.event_loop_running = false;
                        println!("closing now!");
                    }
                    x11rb::protocol::Event::MotionNotify(motion_notify_event) => {
                        self.handler
                            .on_event(crate::Event::Mouse(crate::MouseEvent::Moved {
                                x: motion_notify_event.event_x.unsigned_abs(),
                                y: motion_notify_event.event_y.unsigned_abs(),
                            }));
                    }

                    event => println!("{event:?}"),
                }
            }

            self.handler.on_frame();
        }
    }
}
