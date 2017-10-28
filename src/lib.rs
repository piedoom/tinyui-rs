#[cfg(target_os = "macos")]
#[macro_use] extern crate objc;

#[cfg(target_os = "macos")]
extern crate cocoa;

mod platform;
pub use platform::app::*;
pub use platform::window::*;
pub use platform::label::*;
pub use platform::rect::*;
pub use platform::color::*;
pub use platform::button::*;
pub use platform::slider::*;
pub use platform::webview::*;

mod events;
pub use events::*;
