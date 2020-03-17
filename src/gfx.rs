use piston_window::line_from_to;
use piston_window::math::Matrix2d;
use piston_window::types::Scalar;
use piston_window::Graphics;
use piston_window::*;

use super::instructions::Instruction;

type Coord = (Scalar, Scalar);

fn draw_line<G: Graphics>(
  (x1, y1): Coord,
  (x2, y2): Coord,
  (r, g, b): (u8, u8, u8),
  transform: Matrix2d,
  graphics: &mut G,
) {
  let color = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0];
  line_from_to(color, 1.0, [x1, y1], [x2, y2], transform, graphics);
}

pub fn draw_frame<G: Graphics>(program: &Vec<Instruction>, transform: Matrix2d, graphics: &mut G) {
  clear([1.0; 4], graphics);
  let mut current_direction: Scalar = 0.0;
  let mut current_pen = false;
  let mut current_position: Coord = (0.0, 0.0);
  let mut color = (255, 255, 255);

  for next_instruction in program.iter() {
    match next_instruction {
      Instruction::Forwards(steps) | Instruction::Backwards(steps) => {
        let next_position = move_steps_in_direction(current_position, current_direction, *steps);
        if current_pen {
          draw_line(current_position, next_position, color, transform, graphics);
        }
        current_position = next_position;
      }
      Instruction::TurnLeft(deg) => {
        current_direction -= *deg as Scalar;
      }
      Instruction::TurnRight(deg) => {
        current_direction += *deg as Scalar;
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

fn move_steps_in_direction((x, y): Coord, direction: Scalar, steps: u32) -> Coord {
  (
    x + direction.to_radians().cos() * steps as Scalar,
    y + direction.to_radians().sin() * steps as Scalar,
  )
}
