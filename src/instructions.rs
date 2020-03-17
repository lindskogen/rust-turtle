use std::io::{self, BufRead};

#[derive(Debug)]
pub enum Instruction {
  Forwards(f64),
  Backwards(f64),
  TurnRight(f64),
  TurnLeft(f64),
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

fn parse_u32(string: &str) -> Option<f64> {
  let params: Vec<_> = string
    .split_whitespace()
    .skip(1)
    .filter_map(|num| num.parse::<f64>().ok())
    .collect();

  Some(params[0])
}

fn parse_instruction(string: &str) -> Option<Instruction> {
  match string.split_whitespace().nth(0)? {
    "forwards" => parse_u32(&string[..]).map(Instruction::Forwards),
    "backwards" => parse_u32(&string[..]).map(Instruction::Backwards),
    "turn_right" => parse_u32(&string[..]).map(Instruction::TurnRight),
    "turn_left" => parse_u32(&string[..]).map(Instruction::TurnLeft),
    "drop_pen" => Some(Instruction::DropPen),
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
