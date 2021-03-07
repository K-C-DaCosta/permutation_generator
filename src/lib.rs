use std::{collections::HashSet, usize};

pub fn print_permutations(list: &Vec<String>, sub_size: usize) {
    let mut item_table = HashSet::new();
    let mut item_list = Vec::new();
    print_perm_helper(sub_size, list, &mut item_table, &mut item_list);
}
fn print_perm_helper(
    sub_size: usize,
    list: &Vec<String>,
    items: &mut HashSet<usize>,
    item_list: &mut Vec<usize>,
) {
    if item_list.len() >= list.len() || item_list.len() >= sub_size {
        let permutation = item_list
            .iter()
            .map(|&i| list[i].clone())
            .collect::<Vec<_>>();
        println!("{:?}", permutation);
        return;
    }
    for k in 0..list.len() {
        let query = items.contains(&k);
        if query == false {
            items.insert(k);
            item_list.push(k);
            print_perm_helper(sub_size, list, items, item_list);
            items.remove(&k);
            item_list.pop();
        }
    }
}

/// # Description
/// returns list of reorderings of length `sub_size`
pub fn permutations(list: &Vec<String>, sub_size: usize) -> Vec<Vec<String>> {
    let mut results = Vec::new();
    let mut item_table = HashSet::new();
    let mut item_list = Vec::new();
    perm_helper(
        sub_size,
        list,
        &mut item_table,
        &mut item_list,
        &mut results,
    );
    results
}

fn perm_helper(
    sub_size: usize,
    list: &Vec<String>,
    items: &mut HashSet<usize>,
    item_list: &mut Vec<usize>,
    results: &mut Vec<Vec<String>>,
) {
    if item_list.len() >= list.len() || item_list.len() >= sub_size {
        let permutation = item_list
            .iter()
            .map(|&i| list[i].clone())
            .collect::<Vec<_>>();
        results.push(permutation);
        return;
    }
    for k in 0..list.len() {
        let query = items.contains(&k);
        if query == false {
            items.insert(k);
            item_list.push(k);
            perm_helper(sub_size, list, items, item_list, results);
            items.remove(&k);
            item_list.pop();
        }
    }
}
