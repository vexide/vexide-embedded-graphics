//! [`embedded-graphics`] driver for the VEX V5
//! 
//! [`embedded-graphics`]: https://crates.io/crates/embedded-graphics
//! 
//! This crate provides a [`DrawTarget`] implementation for the VEX V5 brain display,
//! allowing you to draw to the display using the `embedded-graphics` ecosystem.
//! 
//! # Usage
//! 
//! To begin, turn your `display` peripheral into a [`DisplayDriver`]:
//! 
//! ```
//! #![no_std]
//! #![no_main]
//! 
//! use vexide::prelude::*;
//! use vexide_embedded_graphics::DisplayDriver;
//! 
//! #[vexide::main]
//! async fn main(peripherals: Peripherals) {
//!     let mut display = DisplayDriver::new(peripherals.display);
//! }
//! ```
//! 
//! [`DisplayDriver`] is a [`DrawTarget`] that the `embedded-graphics` crate is
//! able to draw to.
//! 
//! ```
//! #![no_std]
//! #![no_main]
//! 
//! use vexide::prelude::*;
//! use vexide_embedded_graphics::DisplayDriver;
//! 
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X10, MonoTextStyle},
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     text::Text,
//! };
//! 
//! #[vexide::main]
//! async fn main(peripherals: Peripherals) {
//!     let mut display = DisplayDriver::new(peripherals.display);
//!     let style = MonoTextStyle::new(&FONT_6X10, Rgb888::GREEN);
//!     
//!     Text::new("Hello,\nRust!", Point::new(2, 28), style)
//!         .draw(&mut display)
//!         .unwrap();
//! }
//! ```
//! 
//! Check out the [`embedded-graphics` docs] for more examples.
//! 
//! [`embedded-graphics` examples]: https://docs.rs/embedded-graphics/latest/embedded_graphics/examples/index.html

#![no_std]

use core::convert::Infallible;
use embedded_graphics_core::{pixelcolor::Rgb888, prelude::*};
use vexide::devices::{
    display::{Display, TouchEvent},
    rgb::Rgb,
};

fn rgb_into_raw(rgb: Rgb<u8>) -> u32 {
    (u32::from(rgb.r) << 16) + (u32::from(rgb.g) << 8) + u32::from(rgb.b)
}

/// An embedded-graphics draw target for the V5 Brain display
/// Currently, this does not support touch detection like the regular [`Display`] API.
pub struct DisplayDriver {
    display: Display,
    triple_buffer:
        [u32; Display::HORIZONTAL_RESOLUTION as usize * Display::VERTICAL_RESOLUTION as usize],
}

impl DisplayDriver {
    /// Create a new [`DisplayDriver`] from a [`Display`].
    /// 
    /// The display peripheral must be moved into this struct,
    /// as it is used to render the display and having multiple
    /// mutable references to it is unsafe.
    #[must_use]
    pub fn new(mut display: Display) -> Self {
        display.set_render_mode(vexide::devices::display::RenderMode::DoubleBuffered);
        Self {
            display,
            #[allow(clippy::large_stack_arrays)] // we got plenty
            triple_buffer: [0; Display::HORIZONTAL_RESOLUTION as usize
                * Display::VERTICAL_RESOLUTION as usize],
        }
    }

    /// Returns the current touch status of the display.
    #[must_use]
    pub fn touch_status(&self) -> TouchEvent {
        self.display.touch_status()
    }
}

impl OriginDimensions for DisplayDriver {
    fn size(&self) -> Size {
        Size {
            width: Display::HORIZONTAL_RESOLUTION as _,
            height: Display::VERTICAL_RESOLUTION as _,
        }
    }
}

impl DrawTarget for DisplayDriver {
    type Color = Rgb888;

    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        pixels
            .into_iter()
            .map(|p| (p.0, rgb_into_raw(Rgb::new(p.1.r(), p.1.g(), p.1.b()))))
            .for_each(|(pos, col)| {
                self.triple_buffer
                    [pos.y as usize * Display::HORIZONTAL_RESOLUTION as usize + pos.x as usize] =
                    col;
            });

        unsafe {
            vex_sdk::vexDisplayCopyRect(
                0,
                0x20,
                Display::HORIZONTAL_RESOLUTION.into(),
                Display::VERTICAL_RESOLUTION.into(),
                self.triple_buffer.as_mut_ptr(),
                Display::HORIZONTAL_RESOLUTION.into(),
            );
        };
        self.display.render();

        Ok(())
    }
}
