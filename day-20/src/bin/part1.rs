mod modules;

use modules::Module;
use modules::Pulse;
use std::collections::HashMap;

fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

fn part_one(input: &str) -> u64 {
  let (mut module_map, module_output_map) = input.lines()
    .map(|line| {
      let (module, outputs) = line.split_once(" -> ")
        .expect("Should have an module and outputs separated by \" -> \"");

      let (name, module) = match module {
        "broadcaster" => {
          (module, Module::Broadcaster)
        },
        _ => {
          let module_type = &module[0..1];
          let module_name = &module[1..];
          match module_type {
            "%" => {
              (module_name, Module::FlipFlop(false))
            },
            "&" => {
              (module_name, Module::Conjunction(HashMap::new()))
            },
            _ => panic!("Should be either '%' or '&'") 
          }
        }
      };
      let outputs = outputs.split(", ")
        .collect::<Vec<_>>();

      (name, module, outputs)
    })
    .fold((HashMap::new(), HashMap::new()), |(mut module_map, mut module_output_map), (name, module, outputs)| {
      module_map.insert(name, module);
      module_output_map.insert(name, outputs);
      (module_map, module_output_map)
    });

  module_map.iter_mut()
    .for_each(|(conjunction_name, conjunction_module)| {
      match conjunction_module {
        Module::Conjunction(memory) => {
          module_output_map.iter()
            .filter(|(_, outputs)| {
              outputs.contains(&conjunction_name)
            })
            .for_each(|(module_name, _)| {
              memory.insert(module_name, Pulse::Low);
            });
        },
        _ => {},
      }
    });

  let mut iterations = Vec::new();
  let num_iterations = 1000;
  let mut iteration_index = 0;
  while iteration_index < num_iterations {
    let start_module = "broadcaster";
    let input_pulse = Pulse::Low;
    let mut modules = std::collections::VecDeque::from([(start_module, ("button", input_pulse))]);

    let mut num_low_pulses = 1;
    let mut num_high_pulses = 0;
    while let Some((module_name, (source, input_pulse))) = modules.pop_front() {
      if let Some(module) = module_map.get_mut(module_name) {
        if let Some(output_pulse) = (*module).emit(source, input_pulse) {
          let outputs = &module_output_map[module_name];
          match output_pulse {
            Pulse::Low => num_low_pulses += outputs.len() as u64,
            Pulse::High => num_high_pulses += outputs.len() as u64,
          }

          for &output in outputs.iter() {
            // println!("{} -{:?}-> {}", module_name, output_pulse, output);
            modules.push_back((output, (module_name, output_pulse)));
          }
        }
      }
    }

    // println!("{} * {} = {}", num_low_pulses, num_high_pulses, num_low_pulses * num_high_pulses);
    iterations.push((num_low_pulses, num_high_pulses));
    iteration_index += 1;
  }
  
  let (num_low_pulses, num_high_pulses) = iterations.iter()
    .fold((0, 0), |(acc_low, acc_high), (num_low, num_high)| {
      (acc_low + num_low, acc_high + num_high)
    });
  num_low_pulses * num_high_pulses
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one_simple() {
    assert_eq!(part_one("broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"), 32000000);
  }

  #[test]
  fn test_part_one_complex() {
    assert_eq!(part_one("broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"), 11687500);
  }
}
