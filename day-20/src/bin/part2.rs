mod modules;

use modules::Module;
use modules::Pulse;
use std::collections::HashMap;

fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

fn part_two(input: &str) -> u64 {
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

  let start_module_name = "broadcaster";
  let module_cycles = module_output_map[start_module_name].iter()
    .map(|&module| {
      let mut module_cycle = HashMap::new();
      let mut open_modules = vec![module];

      module_cycle.insert(start_module_name, vec![module]);
      
      while let Some(module) = open_modules.pop() {
        if module_cycle.contains_key(&module) || module == "rx" {
          continue;
        }
        module_cycle.insert(module, module_output_map[&module].clone());
        open_modules.extend(module_output_map[&module].iter());
      }
      module_cycle
    })
    .collect::<Vec<_>>();

  let end_module = "rx";
  module_cycles.iter()
    .map(|module_cycle| {
      let (&end_module_name, _) = module_cycle.iter()
        .filter(|(_, targets)| {
          targets.contains(&end_module)
        })
        .next()
        .expect("Should contain a module that outputs to \"rx\"");
      
      let mut iteration_count: u64 = 0;
      loop {
        iteration_count += 1;
        let input_pulse = Pulse::Low;
        let mut modules = std::collections::VecDeque::from([(start_module_name, ("button", input_pulse))]);
        while let Some((module_name, (source, input_pulse))) = modules.pop_front() {   
          if module_name == end_module_name {
            if let Pulse::High = input_pulse {
              // println!("End {:?} -> {}", input_pulse, module_name);
              return iteration_count;
            }
          }

          if let Some(module) = module_map.get_mut(module_name) {
            if let Some(output_pulse) = (*module).emit(source, input_pulse) {
              let outputs = &module_cycle[module_name];
  
              for &output in outputs.iter() {
                // println!("{} -{:?}-> {}", module_name, output_pulse, output);
                modules.push_back((output, (module_name, output_pulse)));
              }
            }
          }
        }
      }
    })
    .fold(1, |acc, result| {
      acc * result
    })
}
