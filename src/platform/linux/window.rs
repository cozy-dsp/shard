use std::{
    num::{NonZero, NonZeroU32},
    os::raw::c_int,
    ptr::NonNull,
};

use raw_window_handle::{
    DisplayHandle, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    WindowHandle, XcbDisplayHandle, XcbWindowHandle,
};
use x11rb::{
    COPY_DEPTH_FROM_PARENT, COPY_FROM_PARENT,
    connection::Connection,
    protocol::xproto::{
        AtomEnum, ConnectionExt, CreateWindowAux, EventMask, PropMode, WindowClass,
    },
    resource_manager,
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
    xcb_ffi::XCBConnection,
};

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
    }
}

pub struct Window {
    pub(super) connection: XCBConnection,
    pub(super) atoms: Atoms,
    screen_number: usize,
    window_id: NonZero<u32>,
}

impl HasWindowHandle for Window {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let handle = XcbWindowHandle::new(self.window_id);
        Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Xcb(handle)) })
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Ok(unsafe {
            DisplayHandle::borrow_raw(RawDisplayHandle::Xcb(XcbDisplayHandle::new(
                NonNull::new(self.connection.get_raw_xcb_connection()),
                self.screen_number as c_int,
            )))
        })
    }
}

impl Window {
    pub fn new() -> Self {
        let (connection, screen_number) = XCBConnection::connect(None).unwrap();
        let atoms = Atoms::new(&connection).unwrap().reply().unwrap();
        let screen = &connection.setup().roots[screen_number];
        let root_id = screen.root;
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
        println!("i'm being cleaned up! yay!");
        self.connection
            .destroy_window(self.window_id.get())
            .unwrap();
        self.connection.flush().unwrap();
    }
}
