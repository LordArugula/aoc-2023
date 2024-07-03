fn main() {
  let input = include_str!("./input.txt");
  let output = part_one(input);
  println!("{}", output);
}

#[derive(Debug)]
enum Rule<'a> {
  CompareLess(&'a str, u32, &'a str),
  CompareGreater(&'a str, u32, &'a str),
  Continue(&'a str),
  Accept,
  Reject
}

fn part_one(input: &str) -> u32 {
  let (workflows, parts) = input.split_once("\n\n")
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
                  .parse::<u32>()
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

  parts.lines()
    .map(|line| {
      let mut ratings = line[1..line.len() - 1]
        .split(',');
      let x = ratings.next().expect("First should be 'x'")
        .split_once('=')
        .map(|(_, b)| b.parse::<u32>().expect("Should be a number"))
        .expect("Should have x value");
      let m = ratings.next().expect("First should be 'm'")
        .split_once('=')
        .map(|(_, b)| b.parse::<u32>().expect("Should be a number"))
        .expect("Should have m value");
      let a = ratings.next().expect("First should be 'a'")
        .split_once('=')
        .map(|(_, b)| b.parse::<u32>().expect("Should be a number"))
        .expect("Should have a value");
      let s = ratings.next().expect("First should be 's'")
        .split_once('=')
        .map(|(_, b)| b.parse::<u32>().expect("Should be a number"))
        .expect("Should have s value");
      (x, m, a, s)
    })
    .filter(|&(x, m, a, s)| {
      let mut workflow_name = "in";
      while let Some(rules) = workflows.get(&workflow_name) {
        for rule in rules.iter() {
          match rule {
            Rule::CompareLess(category, value, next_workflow) => {
              match *category {
                "x" => if x < *value {
                  workflow_name = next_workflow;
                  break;
                },
                "m" => if m < *value {
                  workflow_name = next_workflow;
                  break;
                },
                "a" => if a < *value {
                  workflow_name = next_workflow;
                  break;
                },
                "s" => if s < *value {
                  workflow_name = next_workflow;
                  break;
                },
                _ => unreachable!()
              }
            },
            Rule::CompareGreater(category, value, next_workflow) => {
              match *category {
                "x" => if x > *value {
                  workflow_name = next_workflow;
                  break;
                },
                "m" => if m > *value {
                  workflow_name = next_workflow;
                  break;
                },
                "a" => if a > *value {
                  workflow_name = next_workflow;
                  break;
                },
                "s" => if s > *value {
                  workflow_name = next_workflow;
                  break;
                },
                _ => unreachable!()
              }
            },
            Rule::Continue(next_workflow) => {
              workflow_name = next_workflow;
              break;
            },
            Rule::Accept => return true,
            Rule::Reject => return false,
          }
        }
        
        match workflow_name {
          "A" => return true,
          "R" => return false,
          _ => {}
        }
      }
      false
    })
    .map(|(x, m, a, s)| {
      x + m + a + s
    })
    .sum::<u32>()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part_one() {
    assert_eq!(part_one("px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}"), 19114);
  }
}
