# [`embedded-graphics`](https://crates.io/crates/embedded-graphics) Driver for the VEX V5

This crate provides a [`DrawTarget`](https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/draw_target/trait.DrawTarget.html) implementation for the VEX V5 brain display, allowing you to draw to the display using the `embedded-graphics` ecosystem.

# Usage

To begin, turn your `display` peripheral into a `DisplayDriver`:

```rs
#![no_std]
#![no_main]

use vexide::prelude::*;
use vexide_embedded_graphics::DisplayDriver;

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let mut display = DisplayDriver::new(peripherals.display);
}
```

`DisplayDriver` is a [`DrawTarget`](https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/draw_target/trait.DrawTarget.html) that the `embedded-graphics` crate is
able to draw to.

```rs
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
```

Check out the [`embedded-graphics` docs](https://docs.rs/embedded-graphics/latest/embedded_graphics/examples/index.html) for more examples.