#[cfg(feature = "platform-sdl2")]
use sdl2_sys;
#[cfg(feature = "platform-sdl3")]
use sdl3_sys;

pub mod config;

pub fn init() {
    todo!()
    // unsafe { sdl2_sys::SDL_Init(flags); }
}
