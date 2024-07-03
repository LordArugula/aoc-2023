fn main() {
  let input = include_str!("./input.txt");
  let output = part_two(input);
  println!("{}", output);
}

#[derive(Debug)]
enum Rule<'a> {
  CompareLess(&'a str, u64, &'a str),
  CompareGreater(&'a str, u64, &'a str),
  Continue(&'a str),
  Accept,
  Reject
}

fn count_combinations(x: (u64, u64), m: (u64, u64), a: (u64, u64), s: (u64, u64)) -> u64 {
  (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1)
}

fn part_two(input: &str) -> u64 {
  let (workflows, _) = input.split_once("\n\n")
    .expect("Should have an empty line between workflows and parts");

  let workflows = workflows.lines()
    .map(|line| {
      let end_of_name_idx = line.find('{')
        .expect("Should have a '{'");

      let name = &line[0..end_of_name_idx];
      let workflow = &line[end_of_name_idx + 1..line.len() - 1];
      (name, workflow)
    })
    .map(|(name, workflow)| {
      let rules = workflow.split(',')
        .map(|rule| {
          match rule {
            "A" => Rule::Accept,
            "R" => Rule::Reject,
            _ => {
              if let Some(operator_idx) = rule.find(|c| c == '<' || c == '>') {
                let category = &rule[0..operator_idx];
                let operator = &rule[operator_idx..operator_idx + 1];
                let condition_end_idx = rule.find(':')
                  .expect("Should have a ':'");
                let value = rule[operator_idx + 1..condition_end_idx]
                  .parse::<u64>()
                  .expect("Should be a number");
                let next_workflow = &rule[condition_end_idx + 1..];
                match operator {
                  "<" => Rule::CompareLess(category, value, next_workflow),
                  ">" => Rule::CompareGreater(category, value, next_workflow),
                  _ => unreachable!()
                }
              }
              else {
                Rule::Continue(rule)
              }
            }
          }
        })
        .collect::<Vec<_>>();
      (name, rules)
    })
    .collect::<std::collections::HashMap<_,_>>();

  let start_workflow = "in";
  let mut next_workflows = vec![(start_workflow, (1,4000), (1,4000), (1,4000), (1,4000))];

  let mut num_combinations = 0;
  while let Some((workflow, mut x, mut m, mut a, mut s)) = next_workflows.pop() {
    println!("Start {}: x: {:?} m: {:?} a: {:?} s: {:?}", workflow, x, m, a, s);
    
    match workflow {
      "A" => {
        println!("Accepted: {}: x: {:?} m: {:?} a: {:?} s: {:?}", workflow, x, m, a, s);
        num_combinations += count_combinations(x, m, a, s);
        continue;
      },
      "R" => {
        continue;
      },
      _ => {}
    }

    if let Some(rules) = workflows.get(workflow) {
      for rule in rules.iter() {
        match rule {
          Rule::CompareLess(category, value, next_workflow) => {
            match *category {
              "x" => {
                let x1 = (x.0, *value - 1);
                next_workflows.push((next_workflow, x1, m, a, s));
                x = (*value, x.1);
              },
              "m" => {
                let m1 = (m.0, *value - 1);
                next_workflows.push((next_workflow, x, m1, a, s));
                m = (*value, m.1);
              },
              "a" => {
                let a1 = (a.0, *value - 1);
                next_workflows.push((next_workflow, x, m, a1, s));
                a = (*value, a.1);
              },
              "s" => {
                let s1 = (s.0, *value - 1);
                next_workflows.push((next_workflow, x, m, a, s1));
                s = (*value, s.1);
              },
              _ => unreachable!()
            }
          },
          Rule::CompareGreater(category, value, next_workflow) => {
            match *category {
              "x" => {
                let x1 = (*value + 1, x.1);
                next_workflows.push((next_workflow, x1, m, a, s));
                x = (x.0, *value);
              },
              "m" => {
                let m1 = (*value + 1, m.1);
                next_workflows.push((next_workflow, x, m1, a, s));
                m = (m.0, *value);
              },
              "a" => {
                let a1 = (*value + 1, a.1);
                next_workflows.push((next_workflow, x, m, a1, s));
                a = (a.0, *value);
              },
              "s" => {
                let s1 = (*value + 1, s.1);
                next_workflows.push((next_workflow, x, m, a, s1));
                s = (s.0, *value);
              },
              _ => unreachable!()
            }
          },
          Rule::Continue(next_workflow) => {
            next_workflows.push((next_workflow, x, m, a, s));
            break;
          },
          Rule::Accept => {
            num_combinations += count_combinations(x, m, a, s);
            break;
          },
          Rule::Reject => {
            break;
          },
        }
      }
    }
    else {
      unreachable!();
    }
  }
  num_combinations
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_two() {
    assert_eq!(part_two("px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"), 167409079868000);
  }
}
