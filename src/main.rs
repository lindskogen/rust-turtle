use graphics_buffer::{RenderBuffer, IDENTITY};
use piston_window::rectangle::*;
use piston_window::types::Scalar;
use piston_window::*;

#[derive(Eq, PartialEq)]
enum Instruction {
    Forwards,
    Backwards,
    TurnRight,
    TurnLeft,
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
        Instruction::Backwards => direction,
        Instruction::Forwards => direction,
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
    }
}

fn main() {
    let program = vec![
        Instruction::Forwards,
        Instruction::TurnLeft,
        Instruction::Forwards,
        Instruction::Forwards,
    ];

    let mut buffer = RenderBuffer::new(640, 480);
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            clear([1.0; 4], &mut buffer);
            let mut current_direction = Direction::South;
            let mut current_position = [0, 0];

            for next_instruction in program.iter() {
                let size = 50.0;
                let [x, y] = current_position;
                let dims = square(x as Scalar * size, y as Scalar * size, size);
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
                if *next_instruction == Instruction::Forwards
                    || *next_instruction == Instruction::Backwards
                {
                    match current_direction {
                        Direction::East => {
                            current_position[0] += 1;
                        }
                        Direction::West => {
                            current_position[0] -= 1;
                        }
                        Direction::South => {
                            current_position[1] += 1;
                        }
                        Direction::North => {
                            current_position[0] -= 1;
                        }
                    }
                }
                current_direction = rotate(current_direction, next_instruction);
            }
        });
    }
    buffer.save("resources/output.png").unwrap();
}
