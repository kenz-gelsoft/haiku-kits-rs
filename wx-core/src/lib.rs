use std::os::raw::{c_int, c_long};
use std::ptr;

mod generated;
pub use generated::*;

// re-export wx_base
pub use wx_base::*;

#[doc(hidden)]
pub mod methods {
    pub use super::generated::methods::*;

    // re-export wx_base::methods
    pub use wx_base::methods::*;

    pub trait Builder<T: ?Sized> {
        fn build(&self) -> T;
    }
    pub trait Buildable<B: Builder<Self>> {
        fn builder() -> B
        where
            Self: Sized;
    }
}
use methods::*;

mod ffi {
    use std::os::raw::{c_int, c_void};
    extern "C" {
        pub fn wxObject_delete(self_: *mut c_void);
        pub fn wxRustMessageBox(
            message: *const c_void,
            caption: *const c_void,
            style: c_int,
            parent: *mut c_void,
            x: c_int,
            y: c_int,
        );
    }
}

pub struct FrameBuilder {
    title: String,
}
impl FrameBuilder {
    fn new() -> Self {
        Self {
            title: "".to_string(),
        }
    }
    pub fn title(&mut self, s: &str) -> &mut Self {
        self.title = s.to_string();
        self
    }
}
impl Builder<Frame> for FrameBuilder {
    fn build(&self) -> Frame {
        Frame::new(
            Window::none(),
            ID_ANY,
            &self.title,
            &Point::default(),
            &Size::default(),
            DEFAULT_FRAME_STYLE,
            "",
        )
    }
}
impl Buildable<FrameBuilder> for Frame {
    fn builder() -> FrameBuilder {
        FrameBuilder::new()
    }
}

// wxDefaultPosition
impl<const OWNED: bool> Default for PointIsOwned<OWNED> {
    fn default() -> Self {
        PointIsOwned::new_with_int(-1, -1)
    }
}
// wxDefaultSize
impl<const OWNED: bool> Default for SizeIsOwned<OWNED> {
    fn default() -> Self {
        SizeIsOwned::new_with_int(-1, -1)
    }
}
// wxDefaultValidator
impl<const OWNED: bool> Default for ValidatorIsOwned<OWNED> {
    fn default() -> Self {
        ValidatorIsOwned::new()
    }
}

pub fn message_box<T: WindowMethods>(
    message: &str,
    caption: &str,
    style: c_int,
    parent: Option<&T>,
) {
    unsafe {
        let message = wx_base::wx_string_from(message);
        let caption = wx_base::wx_string_from(caption);
        let parent = match parent {
            Some(r) => r.as_ptr(),
            None => ptr::null_mut(),
        };
        ffi::wxRustMessageBox(message, caption, style, parent, -1, -1)
    }
}
