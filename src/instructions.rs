use std::io::{self, BufRead};

#[derive(Eq, PartialEq)]
pub enum Instruction {
  Forwards,
  Backwards,
  TurnRight,
  TurnLeft,
  DropPen,
  LiftPen,
  SetColor(u8, u8, u8),
}

fn parse_color(color: &str) -> Option<Instruction> {
  let colors: Vec<_> = color
    .split_whitespace()
    .skip(1)
    .filter_map(|num| num.parse::<u8>().ok())
    .collect();

  Some(Instruction::SetColor(
    *colors.get(0)?,
    *colors.get(1)?,
    *colors.get(2)?,
  ))
}

fn parse_instruction(string: &str) -> Option<Instruction> {
  match string.split_whitespace().nth(0)? {
    "forwards" => Some(Instruction::Forwards),
    "drop_pen" => Some(Instruction::DropPen),
    "backwards" => Some(Instruction::Backwards),
    "turn_right" => Some(Instruction::TurnRight),
    "turn_left" => Some(Instruction::TurnLeft),
    "lift_pen" => Some(Instruction::LiftPen),
    "set_color" => parse_color(&string[..]),
    _ => None,
  }
}

pub fn parse_file() -> Option<Vec<Instruction>> {
  let reader = io::BufReader::new(io::stdin());

  let instructions: Vec<_> = reader
    .lines()
    .filter_map(|line| line.ok().and_then(|l| parse_instruction(&l[..])))
    .collect();

  Some(instructions)
}
