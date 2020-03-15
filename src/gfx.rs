use piston_window::math::Matrix2d;
use piston_window::rectangle::*;
use piston_window::types::Scalar;
use piston_window::Graphics;
use piston_window::*;

use super::direction::{rotate, Direction};
use super::instructions::Instruction;

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

pub fn draw_frame<G: Graphics>(program: &Vec<Instruction>, transform: Matrix2d, graphics: &mut G) {
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
