use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Context;
use num::Integer;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    let modules = parse_input(input);
    let mut modules: HashMap<String, Module> =
        modules.into_iter().map(|m| (m.name.clone(), m)).collect();
    let mut pulse_counter = PulseCounter::default();
    for _ in 0..1000 {
        let mut queue = VecDeque::new();

        queue.push_back(("broadcaster".to_string(), Pulse::Low, "button".to_string()));

        while let Some((name, pulse, from)) = queue.pop_front() {
            pulse_counter.add(&pulse);

            let module = modules
                .get_mut(&name)
                .with_context(|| format!("module '{}' not found", name))
                .unwrap();
            match &mut module.typ {
                ModuleType::Broadcaster => {
                    for output in &module.outputs {
                        queue.push_back((output.clone(), pulse, name.clone()));
                    }
                }
                ModuleType::FlipFlop(state) => {
                    if pulse == Pulse::High {
                        // ignore
                        continue;
                    }

                    let next_state = state.flip();
                    module.typ = ModuleType::FlipFlop(next_state);

                    let next_pulse = match next_state {
                        OnOff::On => Pulse::High,
                        OnOff::Off => Pulse::Low,
                    };
                    for output in &module.outputs {
                        queue.push_back((output.clone(), next_pulse, name.clone()));
                    }
                }
                ModuleType::Conjunction(state) => {
                    let memory = state.iter_mut().find(|(n, _)| n == &from).unwrap();
                    memory.1 = pulse;

                    let next_pulse = if state.iter().all(|(_, p)| *p == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for output in &module.outputs {
                        queue.push_back((output.clone(), next_pulse, name.clone()));
                    }
                }
                ModuleType::Blackhole => continue,
            }
        }
    }
    Ok(pulse_counter.0 * pulse_counter.1)
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let modules = parse_input(input);
    let mut modules: HashMap<String, Module> =
        modules.into_iter().map(|m| (m.name.clone(), m)).collect();

    let blackhole = modules
        .iter()
        .find(|(_, m)| matches!(m.typ, ModuleType::Blackhole))
        .unwrap()
        .0
        .clone();

    let mut blackhole_input = "".to_string();
    for (name, module) in &mut modules {
        if module.outputs.contains(&blackhole) {
            blackhole_input = name.to_string();
        }
    }

    let mut conjunctions = vec![];
    for (name, module) in &mut modules {
        if module.outputs.contains(&blackhole_input) {
            conjunctions.push(name.to_string());
        }
    }

    let mut lcm = 1;
    for conj in conjunctions {
        let mut modules = modules.clone();
        let mut found = false;
        let mut i = 0;
        while !found {
            i += 1;
            simulate_button_press(&mut modules, |name, pulse, _| {
                if name == conj && pulse == &Pulse::Low {
                    found = true;
                }
            });
        }
        lcm = lcm.lcm(&i);
    }

    Ok(lcm)
}

fn simulate_button_press<F>(modules: &mut HashMap<String, Module>, mut inspect: F)
where
    F: FnMut(&str, &Pulse, &str),
{
    let mut queue = VecDeque::new();

    queue.push_back(("broadcaster".to_string(), Pulse::Low, "button".to_string()));

    while let Some((name, pulse, from)) = queue.pop_front() {
        let module = modules
            .get_mut(&name)
            .with_context(|| format!("module '{}' not found", name))
            .unwrap();

        inspect(&name, &pulse, &from);

        match &mut module.typ {
            ModuleType::Broadcaster => {
                for output in &module.outputs {
                    queue.push_back((output.clone(), pulse, name.clone()));
                }
            }
            ModuleType::FlipFlop(state) => {
                if pulse == Pulse::High {
                    // ignore
                    continue;
                }

                let next_state = state.flip();
                module.typ = ModuleType::FlipFlop(next_state);

                let next_pulse = match next_state {
                    OnOff::On => Pulse::High,
                    OnOff::Off => Pulse::Low,
                };
                for output in &module.outputs {
                    queue.push_back((output.clone(), next_pulse, name.clone()));
                }
            }
            ModuleType::Conjunction(state) => {
                let memory = state.iter_mut().find(|(n, _)| n == &from).unwrap();
                memory.1 = pulse;

                let next_pulse = if state.iter().all(|(_, p)| *p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                for output in &module.outputs {
                    queue.push_back((output.clone(), next_pulse, name.clone()));
                }
            }
            ModuleType::Blackhole => continue,
        }
    }
}

#[derive(Debug, Default)]
struct PulseCounter(u64, u64);

impl PulseCounter {
    fn add(&mut self, pulse: &Pulse) {
        match pulse {
            Pulse::Low => self.0 += 1,
            Pulse::High => self.1 += 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    outputs: Vec<String>,
    typ: ModuleType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType {
    FlipFlop(OnOff),
    Conjunction(Vec<(String, Pulse)>),
    Broadcaster,
    Blackhole,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum OnOff {
    On,
    Off,
}

impl OnOff {
    fn flip(&self) -> Self {
        match self {
            OnOff::On => OnOff::Off,
            OnOff::Off => OnOff::On,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

fn parse_input(input: &str) -> Vec<Module> {
    let mut modules = Vec::new();
    let mut known_modules = HashSet::new();
    let mut known_outputs = HashSet::new();
    for line in input.lines() {
        let (mut name, outputs) = line.split_once(" -> ").unwrap();
        let outputs: Vec<_> = outputs.split(", ").map(|s| s.to_string()).collect();
        let mut typ = ModuleType::Broadcaster;
        match name.chars().next().unwrap() {
            '%' => {
                name = name.trim_start_matches('%');
                typ = ModuleType::FlipFlop(OnOff::Off);
            }
            '&' => {
                name = name.trim_start_matches('&');
                // we fill in the inputs in a second pass below
                typ = ModuleType::Conjunction(Vec::new());
            }
            _ => (),
        }
        if name == "broadcaster" {
            typ = ModuleType::Broadcaster;
        }
        if name != "broadcaster" && typ == ModuleType::Broadcaster {
            panic!("invalid module type");
        }
        for output in &outputs {
            known_outputs.insert(output.clone());
        }
        known_modules.insert(name.to_string());
        modules.push(Module {
            name: name.to_string(),
            outputs,
            typ,
        });
    }

    // fill in the conjunction inputs
    for i in 0..modules.len() {
        if let ModuleType::Conjunction(_) = modules[i].typ {
            let mut inputs = Vec::new();
            for j in 0..modules.len() {
                for output in &modules[j].outputs {
                    if output == &modules[i].name {
                        inputs.push((modules[j].name.clone(), Pulse::Low));
                    }
                }
            }
            modules[i].typ = ModuleType::Conjunction(inputs);
        }
    }

    known_outputs.difference(&known_modules).for_each(|m| {
        modules.push(Module {
            name: m.clone(),
            outputs: Vec::new(),
            typ: ModuleType::Blackhole,
        })
    });

    modules
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day20.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(684125385, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(225872806380073, ans);
    }
}
