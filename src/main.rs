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

fn draw_position<G: Graphics>((x, y): (i32, i32), transform: Matrix2d, graphics: &mut G) {
    let size = 50.0;
    let dims = square(x as Scalar * size, y as Scalar * size, size);
    let rectangle = Rectangle::new([1.0, 0.0, 0.0, 1.0]).border(Border {
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
                    draw_position(current_position, transform, graphics);
                }
            }
            Instruction::TurnLeft | Instruction::TurnRight => {
                current_direction = rotate(current_direction, next_instruction);
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
        Instruction::Forwards,
        Instruction::TurnLeft,
        Instruction::Forwards,
        Instruction::DropPen,
        Instruction::Forwards,
        Instruction::Forwards,
        Instruction::LiftPen,
        Instruction::Forwards,
        Instruction::TurnRight,
        Instruction::Forwards,
        Instruction::DropPen,
        Instruction::Forwards,
        Instruction::Forwards,
    ];

    let mut buffer = RenderBuffer::new(640, 480);
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
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
