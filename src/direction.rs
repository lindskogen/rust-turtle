use super::instructions::Instruction;

#[derive(Eq, PartialEq)]
pub enum Direction {
  North,
  East,
  South,
  West,
}

pub fn rotate(direction: Direction, instruction: &Instruction) -> Direction {
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
