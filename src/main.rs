use graphics_buffer::{RenderBuffer, IDENTITY};
use piston_window::math::Matrix2d;
use piston_window::rectangle::*;
use piston_window::types::Scalar;
use piston_window::Graphics;
use piston_window::*;

#[derive(Eq, PartialEq)]
enum Instruction {
    Forwards,
    Backwards,
    TurnRight,
    TurnLeft,
    DropPen,
    LiftPen,
    SetColor(u8, u8, u8),
}

#[derive(Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn rotate(direction: Direction, instruction: &Instruction) -> Direction {
    match instruction {
        Instruction::TurnLeft => match direction {
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::North => Direction::West,
        },
        Instruction::TurnRight => match direction {
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::North => Direction::East,
        },
        _ => direction,
    }
}

fn draw_position<G: Graphics>(
    (x, y): (i32, i32),
    (r, g, b): (u8, u8, u8),
    transform: Matrix2d,
    graphics: &mut G,
) {
    let size = 32.0;
    let dims = square(x as Scalar * size, y as Scalar * size, size);
    let rectangle = Rectangle::new([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0])
        .border(Border {
            color: [0.0, 0.0, 0.0, 1.0],
            radius: 1.0,
        });

    rectangle.draw(dims, &draw_state::DrawState::default(), transform, graphics);
}

fn draw_frame<G: Graphics>(program: &Vec<Instruction>, transform: Matrix2d, graphics: &mut G) {
    clear([1.0; 4], graphics);
    let mut current_direction = Direction::South;
    let mut current_pen = false;
    let mut current_position = (0, 0);
    let mut color = (255, 255, 255);

    for next_instruction in program.iter() {
        match next_instruction {
            Instruction::Forwards | Instruction::Backwards => {
                match current_direction {
                    Direction::East => {
                        current_position.0 += 1;
                    }
                    Direction::West => {
                        current_position.0 -= 1;
                    }
                    Direction::South => {
                        current_position.1 += 1;
                    }
                    Direction::North => {
                        current_position.0 -= 1;
                    }
                }

                if current_pen {
                    draw_position(current_position, color, transform, graphics);
                }
            }
            Instruction::TurnLeft | Instruction::TurnRight => {
                current_direction = rotate(current_direction, next_instruction);
            }
            Instruction::SetColor(r, g, b) => {
                color = (*r, *g, *b);
            }
            Instruction::DropPen => {
                current_pen = true;
            }
            Instruction::LiftPen => {
                current_pen = false;
            }
        }
    }
}

fn main() {
    let program = vec![
        Instruction::SetColor(226, 92, 51),
        Instruction::Forwards,
        Instruction::TurnLeft,
        Instruction::Forwards,
        Instruction::DropPen,
        Instruction::Forwards,
        Instruction::SetColor(237, 147, 11),
        Instruction::Forwards,
        Instruction::Forwards,
        Instruction::SetColor(234, 200, 81),
        Instruction::Forwards,
        Instruction::Forwards,
        Instruction::SetColor(84, 158, 105),
        Instruction::Forwards,
        Instruction::LiftPen,
        Instruction::Forwards,
        Instruction::TurnRight,
        Instruction::Forwards,
        Instruction::SetColor(47, 81, 81),
        Instruction::DropPen,
        Instruction::Forwards,
        Instruction::Forwards,
    ];

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
