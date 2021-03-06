#![warn(clippy::pedantic)]
#![allow(clippy::enum_glob_use)]

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// All the items in the game
pub enum Items {
    Illum,
    Res,
    Bshields,
    Haven,
    Nimble,
    Mriding,
    Mboost,
    Chronos,
    Rejuv,
    Veteran,
    Ktoheal,
    Lrip,
    Bulldozer,
    Caut,
    Dhands,
    Wrecker,
}

use Items::*;

impl Items {
    // Function to get the cost of the first level of the item
    fn get_base_cost(&self) -> isize {
        match self {
            Illum | Res | Nimble | Rejuv | Veteran | Bulldozer => 150,
            Bshields | Haven | Chronos => 300,
            Mriding | Mboost | Caut | Dhands | Wrecker => 250,
            Ktoheal | Lrip => 200,
        }
    }
}

// Helper function that generates a vector containing 1 of each item
fn get_all_items() -> Vec<Items> {
    vec![
        Illum, Res, Bshields, Haven, Nimble, Mriding, Mboost, Chronos, Rejuv, Veteran, Ktoheal,
        Lrip, Bulldozer, Caut, Dhands, Wrecker,
    ]
}

// Function which adds an item to a list of items. Up to 4 unique items can be selected, and
// each item can only go up to level 3
fn add_item(new_item: &Items, items: &Vec<Items>) -> Option<Vec<Items>> {
    let count = items.iter().filter(|&i| i == new_item).count();
    if count >= 3 {
        return None;
    }
    let mut copy = items.clone();
    copy.sort();
    copy.dedup();
    if count >= 1 || copy.len() < 4 {
        let mut result = items.clone();
        result.push(new_item.clone());
        return Some(result);
    }
    None
}

// Generates the next set of item sets by adding 1 of each item to the set
pub fn successor(items: &Vec<Items>) -> Vec<Vec<Items>> {
    let mut result = vec![];
    let all_items = get_all_items();
    for item in all_items {
        if let Some(s) = add_item(&item, items) {
            result.push(s);
        }
    }
    result
}

// Calculates the cost of a set of items
pub fn item_set_cost(items: &Vec<Items>) -> isize {
    let mut sum = 0isize;
    let mut copy = items.clone();
    copy.sort();
    copy.dedup();
    for i in copy {
        let count = items.iter().filter(|&j| *j == i).count() as isize;
        let mut mul = 0isize;
        for j in 1..=count {
            mul += j;
        }
        sum += i.get_base_cost() * mul;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_item_fail() {
        let set1 = vec![Illum, Illum, Illum];
        assert_eq!(None, add_item(&Illum, &set1));

        let set2 = vec![Illum, Nimble, Chronos, Caut];
        assert_eq!(None, add_item(&Wrecker, &set2));
    }

    #[test]
    fn test_add_item_pass() {
        let set1 = vec![Illum];
        let res1 = Some(vec![Illum, Illum]);
        assert_eq!(res1, add_item(&Illum, &set1));

        let res2 = Some(vec![Illum, Mboost]);
        assert_eq!(res2, add_item(&Mboost, &set1));

        let set2 = vec![];
        let res3 = Some(vec![Nimble]);
        assert_eq!(res3, add_item(&Nimble, &set2));

        let set3 = vec![Caut, Chronos, Caut, Nimble];
        let res4 = Some(vec![Caut, Chronos, Caut, Nimble, Illum]);
        assert_eq!(res4, add_item(&Illum, &set3));
    }

    #[test]
    fn test_successor() {
        let all_items = get_all_items();
        let set1 = vec![Illum];
        let mut res1 = vec![];
        for item in &all_items {
            let mut tmp = set1.clone();
            tmp.push(item.clone());
            res1.push(tmp);
        }
        assert_eq!(res1, successor(&set1));

        let set2 = vec![Illum, Illum, Illum];
        let mut res2 = vec![];
        for item in &all_items {
            if *item != Illum {
                let mut tmp = set2.clone();
                tmp.push(item.clone());
                res2.push(tmp);
            }
        }
        assert_eq!(res2, successor(&set2));

        let set3 = vec![Caut, Caut, Chronos, Caut];
        let mut res3 = vec![];
        for item in &all_items {
            if *item != Caut {
                let mut tmp = set3.clone();
                tmp.push(item.clone());
                res3.push(tmp);
            }
        }
        assert_eq!(res3, successor(&set3));
    }

    #[test]
    fn test_item_set_cost() {
        let set1 = vec![];
        assert_eq!(0, item_set_cost(&set1));

        let set2 = vec![Illum];
        assert_eq!(Illum.get_base_cost(), item_set_cost(&set2));

        let set3 = vec![Caut, Chronos, Caut, Illum];
        let caut_cost = 3 * Caut.get_base_cost();
        assert_eq!(
            caut_cost + Chronos.get_base_cost() + Illum.get_base_cost(),
            item_set_cost(&set3)
        );
    }
}
