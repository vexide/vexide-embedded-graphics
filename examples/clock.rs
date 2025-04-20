//! # Example: Analog Clock
//!
//! This example shows some more advanced usage of Embedded Graphics. It draws a round clock face
//! with hour, minute and second hands. A digital clock is drawn in the middle of the clock. The
//! whole thing is updated with your computer's local time every 50ms.
//!
//! This example is based on the `embedded-graphics` example `clock.rs`.

#![no_std]
#![no_main]

use alloc::format;
use vexide::{
    devices::display::{RenderMode, TouchState},
    prelude::*,
    time::Instant,
};
use vexide_embedded_graphics::DisplayDriver;

use core::f32::consts::PI;
use embedded_graphics::{
    mono_font::{ascii::FONT_9X15, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::Text,
};

extern crate alloc;

/// The margin between the clock face and the display border.
const MARGIN: u32 = 10;

/// Converts a polar coordinate (angle/distance) into an (X, Y) coordinate centered around the
/// center of the circle.
///
/// The angle is relative to the 12 o'clock position and the radius is relative to the edge of the
/// clock face.
fn polar(circle: &Circle, angle: f32, radius_delta: i32) -> Point {
    let radius = circle.diameter as f32 / 2.0 + radius_delta as f32;

    circle.center()
        + Point::new(
            (angle.sin() * radius) as i32,
            -(angle.cos() * radius) as i32,
        )
}

/// Converts an hour into an angle in radians.
fn hour_to_angle(hour: u32) -> f32 {
    // Convert from 24 to 12 hour time.
    let hour = hour % 12;

    (hour as f32 / 12.0) * 2.0 * PI
}

/// Converts a sexagesimal (base 60) value into an angle in radians.
fn sexagesimal_to_angle(value: u32) -> f32 {
    (value as f32 / 60.0) * 2.0 * PI
}

/// Creates a centered circle for the clock face.
fn create_face(target: &impl DrawTarget) -> Circle {
    // The draw target bounding box can be used to determine the size of the display.
    let bounding_box = target.bounding_box();

    let diameter = bounding_box.size.width.min(bounding_box.size.height) - 2 * MARGIN;

    Circle::with_center(bounding_box.center(), diameter)
}

/// Draws a circle and 12 graduations as a simple clock face.
fn draw_face<D>(target: &mut D, clock_face: &Circle) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    // Draw the outer face.
    (*clock_face)
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_BLUE, 2))
        .draw(target)?;

    // Draw 12 graduations.
    for angle in (0..12).map(hour_to_angle) {
        // Start point on circumference.
        let start = polar(clock_face, angle, 0);

        // End point offset by 10 pixels from the edge.
        let end = polar(clock_face, angle, -10);

        Line::new(start, end)
            .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_AZURE, 1))
            .draw(target)?;
    }

    Ok(())
}

/// Draws a clock hand.
fn draw_hand<D>(
    target: &mut D,
    clock_face: &Circle,
    angle: f32,
    length_delta: i32,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    let end = polar(clock_face, angle, length_delta);

    Line::new(clock_face.center(), end)
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_RED, 1))
        .draw(target)
}

/// Draws a decorative circle on the second hand.
fn draw_second_decoration<D>(
    target: &mut D,
    clock_face: &Circle,
    angle: f32,
    length_delta: i32,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    let decoration_position = polar(clock_face, angle, length_delta);

    let decoration_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb888::CSS_YELLOW)
        .stroke_color(Rgb888::CSS_RED)
        .stroke_width(1)
        .build();

    // Draw a fancy circle near the end of the second hand.
    Circle::with_center(decoration_position, 11)
        .into_styled(decoration_style)
        .draw(target)
}

/// Draw digital clock just above center with black text on a white background
fn draw_digital_clock<D>(
    target: &mut D,
    clock_face: &Circle,
    time_str: &str,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    // Create a styled text object for the time text.
    let mut text = Text::new(
        time_str,
        Point::zero(),
        MonoTextStyle::new(&FONT_9X15, Rgb888::CSS_BLACK),
    );

    // Move text to be centered between the 12 o'clock point and the center of the clock face.
    text.translate_mut(
        clock_face.center()
            - text.bounding_box().center()
            - clock_face.bounding_box().size.y_axis() / 4,
    );

    // Add a background around the time digits.
    // Note that there is no bottom-right padding as this is added by the font renderer itself.
    let text_dimensions = text.bounding_box();
    Rectangle::new(
        text_dimensions.top_left - Point::new(3, 3),
        text_dimensions.size + Size::new(4, 4),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb888::CSS_WHITE))
    .draw(target)?;

    // Draw the text after the background is drawn.
    text.draw(target)?;

    Ok(())
}

#[vexide::main]
async fn main(peripherals: Peripherals) -> Result<(), core::convert::Infallible> {
    let mut display = DisplayDriver::new(peripherals.display);
    display.set_render_mode(RenderMode::DoubleBuffered);

    let clock_face = create_face(&display);

    let start = Instant::now();
    'running: loop {
        let time = start.elapsed();

        // Calculate the position of the three clock hands in radians.
        let hours_radians = hour_to_angle(time.as_secs() as u32 / 3600 % 12);
        let minutes_radians = sexagesimal_to_angle((time.as_secs() as u32 % 3600) / 60);
        let seconds_radians = sexagesimal_to_angle(time.as_secs() as u32 % 60);

        let digital_clock_text = format!(
            "{:02}:{:02}:{:02}.{:03}",
            time.as_secs() / 3600 % 12,
            (time.as_secs() % 3600) / 60,
            time.as_secs() % 60,
            time.as_millis() % 1000
        );

        display.clear(Rgb888::CSS_LIGHT_CORAL)?;

        draw_face(&mut display, &clock_face)?;
        draw_hand(&mut display, &clock_face, hours_radians, -60)?;
        draw_hand(&mut display, &clock_face, minutes_radians, -30)?;
        draw_hand(&mut display, &clock_face, seconds_radians, 0)?;
        draw_second_decoration(&mut display, &clock_face, seconds_radians, -20)?;

        // Draw digital clock just above center.
        draw_digital_clock(&mut display, &clock_face, &digital_clock_text)?;

        // Draw a small circle over the hands in the center of the clock face.
        // This has to happen after the hands are drawn so they're covered up.
        Circle::with_center(clock_face.center(), 9)
            .into_styled(PrimitiveStyle::with_fill(Rgb888::CSS_RED))
            .draw(&mut display)?;

        if matches!(display.touch_status().state, TouchState::Pressed) {
            // If the screen is touched, exit the loop.
            break 'running Ok(());
        }

        display.render();

        sleep(core::time::Duration::from_millis(50)).await;
    }
}
