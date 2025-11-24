
use bitvec::prelude::*;

/// Vector of vertex indices corresponding to one group
type Group = Vec<usize>;

/// Create the maximized number of unique assignments given some minimum group
/// size.
pub fn make_assignments(conflicts: &mut Vec<BitVec>, min_group_size: usize) -> Vec<Vec<Vec<Group>>> {
    fn backtrack(
        conflicts: &mut Vec<BitVec>,
        sols: &mut Vec<Vec<Vec<Group>>>,
        curr: &mut Vec<Vec<Group>>,
        best: &mut usize,
        group_sizes: &[usize],
    ) {
        let options = single_assignment(conflicts, group_sizes);
        if options.is_empty() && curr.len() >= *best {
            if curr.len() > *best {
                sols.clear();
            }
            sols.push(curr.clone());
            *best = curr.len();
        } else {
            for opt in options {
                for g in &opt {
                    add_conflicts_between(conflicts, g);
                }
                curr.push(opt);
                backtrack(conflicts, sols, curr, best, group_sizes);
                if let Some(opt) = curr.pop() {
                    for g in &opt {
                        remove_conflicts_between(conflicts, g);
                    }
                }
            }
        }
    }
    let mut sols = vec![];
    let mut curr = vec![];
    let mut best = 0;
    let group_sizes = group_sizes(conflicts.len(), min_group_size);
    backtrack(conflicts, &mut sols, &mut curr, &mut best, &group_sizes);
    sols
}

pub fn group_sizes(n: usize, min_group_size: usize) -> Vec<usize> {
    let mut remaining = n % min_group_size;
    let mut sizes = vec![min_group_size; n / min_group_size]; 
    if sizes.is_empty() {
        return sizes;
    }
    // Evenly distribute leftover across vector
    'outer: loop {
        for i in 0..sizes.len() {
            if remaining == 0 {
                break 'outer sizes;
            }
            sizes[i] += 1;
            remaining -= 1;
        }
    }
}

/// Try all ways to make the current assignment
pub fn single_assignment(conflicts: &mut Vec<BitVec>, group_sizes: &[usize]) -> Vec<Vec<Group>> {
    fn backtrack(
        conflicts: &mut Vec<BitVec>,
        sols: &mut Vec<Vec<Group>>,
        curr: &mut Vec<Group>,
        group_sizes: &[usize],
        skip: &mut BitVec,
    ) {
        let k = group_sizes[curr.len()];
        for g in potential_groups(conflicts, k, skip) {
            if curr.len() == group_sizes.len() - 1 {
                curr.push(g);
                sols.push(curr.clone());
                curr.pop();
            } else {
                for e in &g {
                    skip.set(*e, true)
                }
                curr.push(g);
                backtrack(conflicts, sols, curr, group_sizes, skip);
                if let Some(g) = curr.pop() {
                    for e in g {
                        skip.set(e, false)
                    }
                }
            }
        }
    }

    let n = conflicts.len();
    let mut res: Vec<Vec<Group>> = vec![];
    let mut skip = bitvec![0; n];
    let mut curr = vec![];
    backtrack(conflicts, &mut res, &mut curr, &group_sizes, &mut skip);
    res
}

/// Get all possible ways to group a group of size k together.
pub fn potential_groups(conflicts: &mut Vec<BitVec>, k: usize, skip: &BitVec) -> Vec<Group> {
    fn backtrack(
        conflicts: &mut Vec<BitVec>,
        sols: &mut Vec<Vec<usize>>,
        curr: &mut Vec<usize>,
        row: usize,
        n: usize,
        k: usize,
        skip: &BitVec,
    ) {
        for col in (row + 1)..n {
            if skip[col] {
                continue;
            }
            let is_valid = curr.iter().all(|row| !conflicts[*row][col]);
            if is_valid {
                curr.push(col);
                if curr.len() == k {
                    sols.push(curr.clone());
                } else {
                    add_conflicts(conflicts, col, curr.iter());
                    backtrack(conflicts, sols, curr, col, n, k, skip);
                    remove_conflicts(conflicts, col, curr.iter());
                }
                curr.pop();
            }
        }
    }

    let mut res = vec![];
    let n = conflicts.len();
    for row in 0..n {
        if skip[row] {
            continue;
        }
        let mut curr = vec![row];
        backtrack(conflicts, &mut res, &mut curr, row, n, k, skip);
    }
    res
}

#[inline]
fn add_conflicts_between(conflicts: &mut Vec<BitVec>, between: &[usize]) {
    for i in between {
        for j in between {
            conflicts[*i].set(*j, true);
        }
    }
}

#[inline]
fn remove_conflicts_between(conflicts: &mut Vec<BitVec>, between: &[usize]) {
    for i in between {
        for j in between {
            conflicts[*i].set(*j, false);
        }
    }
}

#[inline]
fn add_conflicts<'a>(conflicts: &mut Vec<BitVec>, col: usize, rows: impl Iterator<Item = &'a usize>) {
    for row in rows {
        conflicts[*row].set(col, true);
        conflicts[col].set(*row, true);
    }
}

#[inline]
fn remove_conflicts<'a>(conflicts: &mut Vec<BitVec>, col: usize, rows: impl Iterator<Item = &'a usize>) {
    for row in rows {
        conflicts[*row].set(col, false);
        conflicts[col].set(*row, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn diagonal(n: usize) -> Vec<BitVec> {
        (0..n).map(|i| {
                let mut vec = bitvec![0; n];
                vec.set(i, true);
                vec
            }).collect()
    }

    #[test]
    fn no_conflicts() {
        let n = 5;
        let k = 3;
        let mut conflicts = diagonal(n);
        let skip = bitvec![0; n];
        let res = potential_groups(&mut conflicts, k, &skip);
        assert_eq!(res.len(), 10);
    }

    fn test_single_assignment(n: usize, k: usize) {
        let mut conflicts = diagonal(n);
        let group_sizes = group_sizes(n, k);
        let res = single_assignment(&mut conflicts, &group_sizes);
        for groups in res {
            let mut seen: HashSet<usize> = HashSet::new();
            let mut count = 0;
            for g in groups {
                count += g.len();
                seen.extend(&g);
            }
            assert_eq!(count, n);
            assert_eq!(seen.len(), count);
        }
    }

    #[test]
    fn odd_complete_graph_single_assignment() {
        let tests = [
            (3, 2),
            (5, 2),
        ];
        for (n, k) in tests {
            test_single_assignment(n, k);
        }
    }

    #[test]
    fn even_complete_graph_single_assignment() {
        let tests = [
            (2, 2),
            (4, 2),
            (6, 2),
            (6, 3),
            (6, 6),
        ];
        for (n, k) in tests {
            test_single_assignment(n, k);
        }
    }

    fn test_all_assignment(n: usize, k: usize, exp_rounds: usize, exp_sizes: &[usize]) {
        let mut conflicts = diagonal(n);
        let res = make_assignments(&mut conflicts, k);
        let nrounds = res[0].len();
        assert_eq!(nrounds, exp_rounds, "Expected {exp_rounds} rounds but found {nrounds}");
        for possibility in res {
            for i in 0..exp_rounds {
                let group_sizes = possibility[i].iter().map(|v| v.len());
                assert!(group_sizes.eq(exp_sizes.iter().copied()));
            }
        }
    }

    #[test]
    fn even_complete_all_assignments() {
        let tests = [
            (4, 2, 3, vec![2, 2]),
            (4, 3, 1, vec![4]),
            (6, 3, 1, vec![3, 3]),
        ];
        for (n, k, exp_rounds, exp_sizes) in tests {
            test_all_assignment(n, k, exp_rounds, &exp_sizes);
        }
    }

    #[test]
    fn odd_complete_all_assignments() {
        let tests = [
            (3, 2, 1, vec![3]),
            (5, 2, 1, vec![3, 2]),
            (5, 3, 1, vec![5]),
            (7, 2, 3, vec![3, 2, 2]),
        ];
        for (n, k, exp_rounds, exp_sizes) in tests {
            test_all_assignment(n, k, exp_rounds, &exp_sizes);
        }
    }
}
