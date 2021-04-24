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

fn get_item_mul(item: &Items, items: &Vec<Items>) -> isize {
    let count = items.iter().filter(|&j| j == item).count() as isize;
    (0..=count).sum::<isize>()
}

fn eval(player: &Champions, items: &Vec<Items>) -> isize {
    let cost = item_set_cost(items);
    let mut sum = 0;
    for i in items {
        if player.uses_abilities_frequently() {
            if *i == Chronos {
                sum += 300 * get_item_mul(i, items)
            }
        }
        if player.has_major_ult() {
            if *i == Mboost {
                sum += 400 * get_item_mul(i, items)
            }
        }
        if player.needs_healer() {
            if *i == Rejuv {
                sum += 200 * get_item_mul(i, items)
            }
        }
        if player.has_sustained_fire() || player.high_dps() {
            if *i == Lrip {
                sum += 200 * get_item_mul(i, items)
            }
        }
        if player.high_dps() {
            if *i == Caut {
                sum += 400 * get_item_mul(i, items)
            }
        }
        if player.is_blaster() {
            if *i == Ktoheal {
                sum += 150 * get_item_mul(i, items)
            }
        }
        if player.lacks_mobility() {
            if *i == Nimble {
                sum += 200 * get_item_mul(i, items)
            }
        }
        if player.has_aoe() {
            if *i == Ktoheal {
                sum += 150 * get_item_mul(i, items)
            }
        }
        if player.has_special_or_slow_reload() {
            if *i == Dhands {
                sum += 300 * get_item_mul(i, items)
            }
        }
        if player.is_squishy() {
            if *i == Haven {
                sum += 400 * get_item_mul(i, items)
            }
        }
        if player.is_fast() {
            if *i == Nimble {
                sum += 200 * get_item_mul(i, items)
            }
        }
        if *i == Caut {
            sum += 100 * get_item_mul(i, items)
        } else if *i == Chronos {
            sum += 100 * get_item_mul(i, items)
        } else if *i == Haven {
            sum += 100 * get_item_mul(i, items)
        } else if *i == Bshields {
            sum += 50 * get_item_mul(i, items)
        }
        if player.get_class() == Class::Support {
            if *i == Caut {
                sum -= 200 * get_item_mul(i, items)
            } else if *i == Nimble {
                sum += 25 * get_item_mul(i, items)
            }
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
