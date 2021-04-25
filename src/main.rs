#![warn(clippy::pedantic)]
#![allow(dead_code)]
#![allow(unused_imports)]

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
// Lists the players in the server
struct GameInfo {
    player: Champions,
    team: Vec<Champions>,
    enemy: Vec<Champions>,
}

// Reads in game state from json file
fn get_game_state() -> GameInfo {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

// Adds a multiplier to the value of the item based on it's level
// Helps counteract the increase in cost per level.
fn get_item_mul(item: &Items, items: &Vec<Items>) -> isize {
    let count = items.iter().filter(|&j| j == item).count() as isize;
    (0..=count).sum::<isize>()
}

// Evaluation function. Returns a score for a set of items based
// on how useful those items are to the player.
fn eval(
    player: &Champions,
    items: &Vec<Items>,
    team: &Vec<Champions>,
    enemy: &Vec<Champions>,
) -> isize {
    let cost = item_set_cost(items);
    let mut sum = 0;
    let mut has_healer = false;
    let mut enemy_aoe = 0;
    let mut enemy_hitscan = 0;
    let mut enemy_deployables = 0;
    let mut enemy_cloak = 0;
    let mut enemy_cc = 0;
    let mut enemy_shields = 0;
    let mut enemy_torv_ruckus = false;
    let mut enemy_imani = false;
    for teammember in team {
        if teammember.get_class() == Class::Support {
            has_healer = true;
            break;
        }
    }
    for enemy_member in enemy {
        if enemy_member.has_aoe() {
            enemy_aoe += 1
        } else {
            enemy_hitscan += 1
        }
        if enemy_member.has_deployable() {
            enemy_deployables += 1
        }
        if enemy_member.has_cloak() {
            enemy_cloak += 1
        }
        if enemy_member.has_cc() {
            enemy_cc += 1
        }
        if enemy_member.has_normal_shield() {
            enemy_shields += 1
        }
        if enemy_member == &Champions::Torvald || enemy_member == &Champions::Ruckus {
            enemy_torv_ruckus = true
        } else if enemy_member == &Champions::Imani {
            enemy_imani = true
        }
    }
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
                sum += 300 * get_item_mul(i, items)
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
        if player.get_class() == Class::Support {
            if *i == Caut {
                sum -= 200 * get_item_mul(i, items)
            } else if *i == Nimble {
                sum += 25 * get_item_mul(i, items)
            }
        } else if player.get_class() == Class::FrontLine
            || player.get_class() == Class::Flank && *player != Champions::Yagorath
        {
            if *i == Mriding {
                sum += 50 * get_item_mul(i, items)
            }
        }
        if !has_healer && *i == Rejuv {
            sum = -10000;
        }
        if *i == Caut {
            sum += 100 * get_item_mul(i, items)
        } else if *i == Chronos {
            sum += 100 * get_item_mul(i, items)
        } else if *i == Haven {
            sum += 100 * get_item_mul(i, items) * enemy_hitscan
        } else if *i == Bshields {
            sum += 75 * get_item_mul(i, items) * enemy_aoe
        } else if *i == Wrecker {
            sum += 75 * get_item_mul(i, items) * enemy_shields;
            if enemy_torv_ruckus {
                sum += 50 * get_item_mul(i, items)
            }
        } else if *i == Bulldozer {
            if player.high_dps() {
                sum += 20 * get_item_mul(i, items) * enemy_deployables;
                if enemy_imani {
                    sum += 100 * get_item_mul(i, items)
                }
            }
        } else if *i == Illum {
            if enemy_cloak == 0 {
                sum = -10000
            }
            sum += 75 * get_item_mul(i, items) * enemy_cloak
        } else if *i == Res {
            sum += 25 * get_item_mul(i, items) * enemy_cc;
            if player == &Champions::Raum {
                sum += 100 * get_item_mul(i, items) * enemy_cc
            }
        }
    }
    sum - cost
}

fn main() {
    let game_state = get_game_state();
    let player = game_state.player;
    let team = game_state.team;
    let enemy = game_state.enemy;
    let root = vec![];
    let mut successors = successor(&root);
    let mut choice = vec![];
    while choice.len() < 12 {
        let mut max = std::isize::MIN;
        for s in &successors {
            let curr = eval(&player, &s, &team, &enemy);
            if curr > max {
                max = curr;
                choice = s.clone();
            }
        }
        successors = successor(&choice);
    }
    println!("The items you should get, in order, are: {:?}", choice);
}
