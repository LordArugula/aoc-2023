use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum Pulse {
  Low,
  High
}

#[derive(Debug)]
pub enum Module<'a> {
  Broadcaster,
  FlipFlop(bool),
  Conjunction(HashMap<&'a str, Pulse>)
}

impl<'a> Module<'a> {
  pub fn emit(&mut self, source: &'a str, input: Pulse) -> Option<Pulse> {
    match self {
      Module::Broadcaster => Some(input),
      Module::FlipFlop(on) => {
        match input {
          Pulse::Low => {
            *on = !*on;
            match on {
              true => Some(Pulse::High),
              false => Some(Pulse::Low)
            }
          },
          Pulse::High => None
        }
      },
      Module::Conjunction(memory) => {
        memory.entry(&source)
          .and_modify(|value| *value = input)
          .or_insert(input);

        match memory.values().all(|&x| if let Pulse::High = x { true } else { false }) {
          true => Some(Pulse::Low),
          false => Some(Pulse::High),
        }
      }
    }
  }
}
