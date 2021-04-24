#![warn(clippy::pedantic)]
#![allow(dead_code)]
#![allow(unused_imports)]

// use petgraph::graph::Graph;
use serde::Deserialize;
use serde_json::Result;
use std::env;
use std::fs::File;
use std::io::BufReader;

mod champions;
mod items;

use champions::*;
use items::Items::*;
use items::*;

#[derive(Debug, Deserialize)]
struct GameInfo {
    player: Champions,
    team: Vec<Champions>,
    enemy: Vec<Champions>,
}

fn get_game_state() -> GameInfo {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn eval(player: &Champions, items: &Vec<Items>) -> isize {
    let chronos_mul = items.iter().filter(|&j| *j == Chronos).count() as isize;
    let mboost_mul = items.iter().filter(|&j| *j == Mboost).count() as isize;
    let rejuv_mul = items.iter().filter(|&j| *j == Rejuv).count() as isize;
    let lrip_mul = items.iter().filter(|&j| *j == Lrip).count() as isize;
    let ktoheal_mul = items.iter().filter(|&j| *j == Ktoheal).count() as isize;
    let nimble_mul = items.iter().filter(|&j| *j == Nimble).count() as isize;
    let dhands_mul = items.iter().filter(|&j| *j == Dhands).count() as isize;
    let haven_mul = items.iter().filter(|&j| *j == Haven).count() as isize;
    let caut_mul = items.iter().filter(|&j| *j == Caut).count() as isize;
    let cost = item_set_cost(items);
    let mut sum = 0;
    for i in items {
        if player.uses_abilities_frequently() {
            if *i == Chronos {
                sum += 700 * chronos_mul;
            }
        }
        if player.has_major_ult() {
            if *i == Mboost {
                sum += 700 * mboost_mul
            }
        }
        if player.needs_healer() {
            if *i == Rejuv {
                sum += 800 * rejuv_mul
            }
        }
        if player.has_sustained_fire() {
            if *i == Lrip {
                sum += 600 * lrip_mul
            }
        }
        if player.high_dps() {
            if *i == Lrip {
                sum += 200 * lrip_mul
            } else if *i == Caut {
                sum += 700 * caut_mul
            }
        }
        if player.is_blaster() {
            if *i == Ktoheal {
                sum += 500 * ktoheal_mul
            }
        }
        if player.lacks_mobility() {
            if *i == Nimble {
                sum += 500 * nimble_mul
            }
        }
        if player.has_aoe() {
            if *i == Ktoheal {
                sum += 600 * ktoheal_mul
            }
        }
        if player.has_special_or_slow_reload() {
            if *i == Dhands {
                sum += 600 * dhands_mul
            }
        }
        if player.is_squishy() {
            if *i == Haven {
                sum += 600 * haven_mul
            }
        }
        if player.is_fast() {
            if *i == Nimble {
                sum += 500 * nimble_mul
            }
        }
        if *i == Caut {
            sum += 200 * caut_mul
        } else if *i == Chronos {
            sum += 200 * chronos_mul
        } else if *i == Haven {
            sum += 200 * haven_mul
        } else if *i == Bshields {
            sum += 150 * haven_mul
        }
    }
    sum - cost
}

fn main() {
    let game_state = get_game_state();
    let player = game_state.player;
    let root = vec![];
    let mut successors = successor(&root);
    let mut choice = vec![];
    while choice.len() < 12 {
        let mut max = std::isize::MIN;
        for s in &successors {
            let curr = eval(&player, &s);
            if curr > max {
                max = curr;
                choice = s.clone();
            }
        }
        successors = successor(&choice);
    }
    println!("The items you should get, in order, are: {:?}", choice);
}
