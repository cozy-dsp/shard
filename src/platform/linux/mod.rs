pub mod event_loop;

use std::num::{NonZero, NonZeroU32};

use raw_window_handle::{HasWindowHandle, RawWindowHandle, XcbWindowHandle};
use x11rb::{
    COPY_DEPTH_FROM_PARENT, COPY_FROM_PARENT,
    connection::Connection,
    protocol::xproto::{
        AtomEnum, ConnectionExt, CreateWindowAux, EventMask, PropMode, WindowClass,
    },
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

pub(crate) use event_loop::EventLoop;

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
    }
}

pub struct Window {
    connection: RustConnection,
    screen_number: usize,
    atoms: Atoms,
    window_id: NonZero<u32>,
}

impl Window {
    pub fn new() -> Self {
        let (connection, screen_number) = x11rb::connect(None).unwrap();
        let atoms = Atoms::new(&connection).unwrap().reply().unwrap();
        let root_id = connection.setup().roots[screen_number].root;
        let window_id = NonZeroU32::new(connection.generate_id().unwrap()).unwrap();
        let _ = connection
            .create_window(
                COPY_DEPTH_FROM_PARENT,
                window_id.get(),
                root_id,
                0,
                0,
                512,
                512,
                0,
                WindowClass::INPUT_OUTPUT,
                COPY_FROM_PARENT,
                &CreateWindowAux::new().event_mask(
                    EventMask::EXPOSURE
                        | EventMask::POINTER_MOTION
                        | EventMask::BUTTON_PRESS
                        | EventMask::BUTTON_RELEASE
                        | EventMask::KEY_PRESS
                        | EventMask::KEY_RELEASE
                        | EventMask::STRUCTURE_NOTIFY
                        | EventMask::ENTER_WINDOW
                        | EventMask::LEAVE_WINDOW,
                ),
            )
            .unwrap();

        connection.map_window(window_id.get()).unwrap();

        connection
            .change_property32(
                PropMode::REPLACE,
                window_id.get(),
                atoms.WM_PROTOCOLS,
                AtomEnum::ATOM,
                &[atoms.WM_DELETE_WINDOW],
            )
            .unwrap();

        connection.flush().unwrap();

        Self {
            connection,
            atoms,
            screen_number,
            window_id,
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.connection
            .destroy_window(self.window_id.get())
            .unwrap();
        self.connection.flush().unwrap();
    }
}
