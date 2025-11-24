use std::collections::HashSet;

/// Vector of vertex indices corresponding to one group
type Group = Vec<usize>;

/// Create the maximized number of unique assignments given some minimum group
/// size.
/// FIXME: Don't repeat same groups with order rearranged
pub fn make_assignments(conflicts: &mut Vec<Vec<bool>>, min_group_size: usize) -> Vec<Vec<Vec<Group>>> {
    fn backtrack(
        conflicts: &mut Vec<Vec<bool>>,
        sols: &mut Vec<Vec<Vec<Group>>>,
        curr: &mut Vec<Vec<Group>>,
        best: &mut usize,
        min_group_size: usize,
    ) {
        let options = single_assignment(conflicts, min_group_size);
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
                backtrack(conflicts, sols, curr, best, min_group_size);
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
    backtrack(conflicts, &mut sols, &mut curr, &mut best, min_group_size);
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
/// FIXME: Repeats with groups rearranged in order
pub fn single_assignment(conflicts: &mut Vec<Vec<bool>>, min_group_size: usize) -> Vec<Vec<Group>> {
    fn backtrack(
        conflicts: &mut Vec<Vec<bool>>,
        sols: &mut Vec<Vec<Group>>,
        curr: &mut Vec<Group>,
        sizes: &[usize],
        skip: &mut HashSet<usize>,
    ) {
        let k = sizes[curr.len()];
        for g in potential_groups(conflicts, k, skip) {
            if curr.len() == sizes.len() - 1 {
                curr.push(g);
                sols.push(curr.clone());
                curr.pop();
            } else {
                skip.extend(&g);
                add_conflicts_between(conflicts, &g);
                curr.push(g);
                backtrack(conflicts, sols, curr, sizes, skip);
                if let Some(g) = curr.pop() {
                    remove_conflicts_between(conflicts, &g);
                    for e in g {
                        skip.remove(&e);
                    }
                }
            }
        }
    }

    let n = conflicts.len();
    let mut res: Vec<Vec<Group>> = vec![];
    let mut skip = HashSet::new();
    let mut curr = vec![];
    let sizes = group_sizes(n, min_group_size);
    backtrack(conflicts, &mut res, &mut curr, &sizes, &mut skip);

    res
}

/// Get all possible ways to group a group of size k together.
pub fn potential_groups(conflicts: &mut Vec<Vec<bool>>, k: usize, skip: &HashSet<usize>) -> Vec<Group> {
    fn backtrack(
        conflicts: &mut Vec<Vec<bool>>,
        sols: &mut Vec<Vec<usize>>,
        curr: &mut Vec<usize>,
        row: usize,
        n: usize,
        k: usize,
        skip: &HashSet<usize>,
    ) {
        for col in (row + 1)..n {
            if skip.contains(&col) {
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
        if skip.contains(&row) {
            continue;
        }
        let mut curr = vec![row];
        backtrack(conflicts, &mut res, &mut curr, row, n, k, skip);
    }
    res
}

fn add_conflicts_between(conflicts: &mut Vec<Vec<bool>>, between: &[usize]) {
    for i in between {
        for j in between {
            conflicts[*i][*j] = true;
        }
    }
}

fn remove_conflicts_between(conflicts: &mut Vec<Vec<bool>>, between: &[usize]) {
    for i in between {
        for j in between {
            conflicts[*i][*j] = false;
        }
    }
}

fn add_conflicts<'a>(conflicts: &mut Vec<Vec<bool>>, col: usize, rows: impl Iterator<Item = &'a usize>) {
    for row in rows {
        conflicts[*row][col] = true;
        conflicts[col][*row] = true;
    }
}

fn remove_conflicts<'a>(conflicts: &mut Vec<Vec<bool>>, col: usize, rows: impl Iterator<Item = &'a usize>) {
    for row in rows {
        conflicts[*row][col] = false;
        conflicts[col][*row] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn diagonal(n: usize) -> Vec<Vec<bool>> {
        (0..n).map(|i| {
                let mut vec = vec![false; n];
                vec[i] = true;
                vec
            }).collect()
    }

    #[test]
    fn no_conflicts() {
        let n = 5;
        let k = 3;
        let mut conflicts = diagonal(n);
        let skip = HashSet::new();
        let res = potential_groups(&mut conflicts, k, &skip);
        assert_eq!(res.len(), 10);
    }

    #[test]
    fn odd_complete_graph_single_assignment() {
        let tests = [
            (3, 2),
            (5, 2),
        ];
        for (n, k) in tests {
            let mut conflicts = diagonal(n);
            let res = single_assignment(&mut conflicts, k);
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
            let mut conflicts = diagonal(n);
            let res = single_assignment(&mut conflicts, k);
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
    }

    #[test]
    fn even_complete_all_assignments() {
        // TODO: Write tests
    }

    #[test]
    fn odd_complete_all_assignments() {
        let mut conflicts = diagonal(5);
        let res = make_assignments(&mut conflicts, 3);
        assert_eq!(res[0].len(), 1);
    }
}
