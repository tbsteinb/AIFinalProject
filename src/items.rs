#![warn(clippy::pedantic)]
#![allow(clippy::enum_glob_use)]

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Items {
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
    fn get_base_cost(&self) -> usize {
        match self {
            Illum | Res | Nimble | Rejuv | Veteran | Bulldozer => 150,
            Bshields | Haven | Chronos => 300,
            Mriding | Mboost | Caut | Dhands | Wrecker => 250,
            Ktoheal | Lrip => 200,
        }
    }
}

fn get_all_items() -> Vec<Items> {
    vec![
        Illum, Res, Bshields, Haven, Nimble, Mriding, Mboost, Chronos, Rejuv, Veteran, Ktoheal,
        Lrip, Bulldozer, Caut, Dhands, Wrecker,
    ]
}

fn add_item(new_item: &Items, items: &HashSet<(Items, usize)>) -> Option<HashSet<(Items, usize)>> {
    let possible_item_tuple = vec![(new_item.clone(), 1), (new_item.clone(), 2)];
    if items.contains(&(new_item.clone(), 3)) {
        return None;
    }
    for item_tuple in &possible_item_tuple {
        if items.contains(&item_tuple) {
            let mut result = items.clone();
            result.remove(&item_tuple);
            result.insert((new_item.clone(), item_tuple.1 + 1));
            return Some(result);
        }
    }
    if items.len() > 3 {
        return None;
    }
    let mut result = items.clone();
    result.insert((new_item.clone(), 1));
    Some(result)
}

fn successor(items: &HashSet<(Items, usize)>) -> Vec<HashSet<(Items, usize)>> {
    let mut result = vec![];
    let all_items = get_all_items();
    for item in all_items {
        if let Some(s) = add_item(&item, items) {
            result.push(s);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_item_fail() {
        let set1 = vec![(Illum, 3)].into_iter().collect();
        assert_eq!(None, add_item(&Illum, &set1));

        let set2 = vec![(Illum, 1), (Nimble, 1), (Chronos, 1), (Caut, 1)]
            .into_iter()
            .collect();
        assert_eq!(None, add_item(&Wrecker, &set2));
    }

    #[test]
    fn test_add_item_pass() {
        let set1 = vec![(Illum, 1)].into_iter().collect();
        let res1 = Some(vec![(Illum, 2)].into_iter().collect());
        assert_eq!(res1, add_item(&Illum, &set1));

        let res2 = Some(vec![(Illum, 1), (Mboost, 1)].into_iter().collect());
        assert_eq!(res2, add_item(&Mboost, &set1));

        let set2 = HashSet::new();
        let res3 = Some(vec![(Nimble, 1)].into_iter().collect());
        assert_eq!(res3, add_item(&Nimble, &set2));
    }

    #[test]
    fn test_successor() {
        let all_items = get_all_items();
        let set1: HashSet<(Items, usize)> = vec![(Illum, 1)].into_iter().collect();
        let mut res1 = vec![];
        for item in &all_items {
            if *item == Illum {
                res1.push(vec![(Illum, 2)].into_iter().collect());
            } else {
                let mut tmp: HashSet<(Items, usize)> = set1.clone();
                tmp.insert((item.clone(), 1));
                res1.push(tmp);
            }
        }
        assert_eq!(res1, successor(&set1));

        let set2: HashSet<(Items, usize)> = vec![(Illum, 3)].into_iter().collect();
        let mut res2 = vec![];
        for item in &all_items {
            if *item != Illum {
                let mut tmp: HashSet<(Items, usize)> = set2.clone();
                tmp.insert((item.clone(), 1));
                res2.push(tmp);
            }
        }
        assert_eq!(res2, successor(&set2));
    }
}
