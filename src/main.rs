use graphics_buffer::{RenderBuffer, IDENTITY};
use piston_window::*;

mod direction;
mod instructions;
use instructions::parse_file;
mod gfx;
use gfx::draw_frame;

fn main() {
    let program = parse_file().expect("No input provided");

    let mut buffer = RenderBuffer::new(640, 640);
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 640])
        .exit_on_esc(true)
        .build()
        .unwrap();

    draw_frame(&program, IDENTITY, &mut buffer);
    buffer.save("resources/output.png").unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            draw_frame(&program, context.transform, graphics);
        });
    }
}
