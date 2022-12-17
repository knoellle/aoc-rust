use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Display, Write},
    fs::read_to_string,
    sync::{Arc, Mutex},
};

use eyre::Result;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position([char; 2]);

impl Position {
    fn from_str(name: &str) -> Self {
        let mut chars = name.chars();
        Self([chars.next().unwrap(), chars.next().unwrap()])
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0[0])?;
        f.write_char(self.0[1])
    }
}

#[derive(Clone)]
struct Node {
    rate: u32,
    exits: HashMap<Position, u32>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, cost) in &self.exits {
            f.write_fmt(format_args!("  {name:?}: {cost}\n"))?
        }
        Ok(())
    }
}

struct Graph {
    nodes: HashMap<Position, Node>,
}

impl std::fmt::Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, node) in &self.nodes {
            f.write_fmt(format_args!("{name:?}: {node:?}\n"))?
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Step {
    Move(Position),
    Open,
}

impl Graph {
    fn parse(input: &str) -> Result<Self> {
        let mut nodes = HashMap::new();

        let exits_regex = Regex::new(r"[A-Z]{2}").unwrap();

        for line in input.lines() {
            let name = line.split_whitespace().nth(1).unwrap();
            let rate = line.split(['=', ';']).nth(1).unwrap().parse().unwrap();
            println!("{line}");
            let exits = exits_regex
                .find_iter(line)
                .map(|exit| (Position::from_str(exit.as_str()), 1))
                .collect();
            nodes.insert(Position::from_str(name), Node { rate, exits });
            println!("{name}");
        }

        Ok(Graph { nodes })
    }

    fn optimize(&self) -> Self {
        let mut nodes = HashMap::new();
        let relevant_nodes: HashMap<Position, Node> = self
            .nodes
            .iter()
            .filter(|(name, node)| node.rate > 0 || name.0 == ['A', 'A'])
            .map(|(name, node)| (*name, node.to_owned()))
            .collect();

        for (name, node) in relevant_nodes.iter() {
            let mut queue = VecDeque::new();
            queue.push_back(name);
            let mut costs = HashMap::new();
            costs.insert(name, 0);

            while let Some(other_name) = queue.pop_front() {
                let own_cost = costs[&other_name];
                for exit in self.nodes[other_name].exits.iter() {
                    let entry = costs.entry(exit.0).or_insert(u32::MAX);
                    if *entry > own_cost + exit.1 {
                        *entry = own_cost + exit.1;
                        queue.push_back(exit.0);
                    }
                }
            }

            let exits = relevant_nodes
                .iter()
                .filter(|(name2, _node)| name != *name2)
                .map(|(name, _node)| (*name, costs[name]))
                .collect();

            nodes.insert(
                *name,
                Node {
                    rate: node.rate,
                    exits,
                },
            );
        }

        Self { nodes }
    }

    #[cfg(test)]
    fn pressure_released_on_route(&self, route: &[Step]) -> u32 {
        let total_time = 30;
        assert!(route.len() < total_time);
        let mut position = Position::from_str("AA");
        let mut opened = HashSet::new();
        route
            .iter()
            .enumerate()
            .map(|(index, step)| match step {
                Step::Move(new_position) => {
                    if !self.nodes[&position].exits.contains_key(new_position) {
                        panic!("{new_position} not reachable from {position}. Route: {route:?}");
                    }
                    position = *new_position;
                    0
                }
                Step::Open => {
                    if !opened.insert(position) {
                        panic!("{position} opened twice. Route: {route:?}");
                    }
                    let time_remaining = total_time - (1 + index);
                    self.nodes[&position].rate * time_remaining as u32
                }
            })
            .sum()
    }

    fn find_best_path_recurse(
        &self,
        position: Position,
        time: u32,
        score_so_far: u32,
        best_score: Arc<Mutex<u32>>,
        already_opened: &mut HashSet<Position>,
    ) -> Option<(u32, Vec<Step>)> {
        // println!("{time} {score_so_far}");
        if time == 0 || already_opened.len() == self.nodes.len() - 1 {
            return None;
        }
        let remaining_valves = self
            .nodes
            .iter()
            .filter(|(name, _node)| !already_opened.contains(name));
        let best_possible: u32 = remaining_valves
            .clone()
            .map(|(_name, node)| node.rate * (time - 1))
            .sum();
        {
            if *best_score.lock().unwrap() > score_so_far + best_possible {
                // println!("Culling");
                return None;
            }
        }
        let open_step = (self.nodes[&position].rate > 0 && !already_opened.contains(&position))
            .then(|| {
                assert!(already_opened.insert(position));

                let additional_release = (time - 1) * self.nodes[&position].rate;
                let pressure = score_so_far + additional_release;
                {
                    let mut best_score = best_score.lock().unwrap();
                    // println!("{best_score} < {pressure}");
                    if *best_score < pressure {
                        **best_score.borrow_mut() = pressure;
                        println!("new best: {pressure}");
                    }
                }
                let (pressure, steps) = self
                    .find_best_path_recurse(
                        position,
                        time - 1,
                        score_so_far + additional_release,
                        best_score.clone(),
                        already_opened,
                    )
                    .unwrap_or_default();
                assert!(already_opened.remove(&position));
                (pressure + additional_release, Step::Open, steps)
            });

        let move_steps = self.nodes[&position].exits.par_iter().filter_map(|exit| {
            if time < *exit.1 {
                return None;
            }
            let mut already_opened = already_opened.clone();
            let (pressure, steps) = self
                .find_best_path_recurse(
                    *exit.0,
                    time - exit.1,
                    score_so_far,
                    best_score.clone(),
                    &mut already_opened,
                )
                .unwrap_or_default();
            Some((pressure, Step::Move(*exit.0), steps))
        });

        let (pressure, step, mut steps) = open_step
            .into_par_iter()
            .chain(move_steps)
            .max_by_key(|(pressure, _step, _steps)| *pressure)?;

        steps.insert(0, step);
        Some((pressure, steps))
    }

    fn find_best_path(&self, position: Position, time: u32) -> Option<(u32, Vec<Step>)> {
        let best_score = Arc::new(Mutex::new(0));
        self.find_best_path_recurse(
            position,
            time,
            0,
            best_score,
            &mut HashSet::from_iter(Some(position)),
        )
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let graph = Graph::parse(&input).unwrap();
    let graph = graph.optimize();
    println!("{graph:?}");
    let (pressure, steps) = graph.find_best_path(Position::from_str("AA"), 30).unwrap();

    println!("{steps:?}");
    println!("{pressure:?}");
}

#[cfg(test)]
mod test {

    use super::*;

    fn move_to(position: &str) -> Step {
        Step::Move(Position::from_str(position))
    }

    fn open() -> Step {
        Step::Open
    }

    #[test]
    fn example() {
        let input = read_to_string("example").unwrap();
        let graph = Graph::parse(&input).unwrap();
        let total_pressure_released = graph.pressure_released_on_route(&[
            move_to("DD"),
            open(),
            move_to("CC"),
            move_to("BB"),
            open(),
            move_to("AA"),
            move_to("II"),
            move_to("JJ"),
            open(),
            move_to("II"),
            move_to("AA"),
            move_to("DD"),
            move_to("EE"),
            move_to("FF"),
            move_to("GG"),
            move_to("HH"),
            open(),
            move_to("GG"),
            move_to("FF"),
            move_to("EE"),
            open(),
            move_to("DD"),
            move_to("CC"),
            open(),
        ]);
        assert_eq!(total_pressure_released, 1651);
        assert_eq!(
            graph
                .find_best_path(Position::from_str("AA"), 30)
                .unwrap()
                .0,
            1651
        );
    }
}
