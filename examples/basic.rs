#![no_std]
#![no_main]

use vexide::prelude::*;
use vexide_embedded_graphics::DisplayDriver;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    text::Text,
};

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let mut display = DisplayDriver::new(peripherals.display);
    let style = MonoTextStyle::new(&FONT_6X10, Rgb888::GREEN);
    
    Text::new("Hello,\nRust!", Point::new(2, 28), style)
        .draw(&mut display)
        .unwrap();
}
