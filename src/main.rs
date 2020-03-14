use graphics_buffer::{RenderBuffer, IDENTITY};
use piston_window::rectangle::*;
use piston_window::types::Scalar;
use piston_window::*;

fn main() {
    let mut buffer = RenderBuffer::new(640, 480);
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            clear([1.0; 4], &mut buffer);
            for i in 0..9 {
                let dims = square(i as Scalar * 50.0, 0.0, 50.0);
                let rectangle = Rectangle::new([1.0, 0.0, 0.0, 1.0]).border(Border {
                    color: [0.0, 0.0, 0.0, 1.0],
                    radius: 1.0,
                });

                rectangle.draw(
                    dims,
                    &draw_state::DrawState::default(),
                    context.transform,
                    graphics,
                );
                rectangle.draw(
                    dims,
                    &draw_state::DrawState::default(),
                    IDENTITY,
                    &mut buffer,
                );
            }
        });
    }
    buffer.save("resources/output.png").unwrap();
}
