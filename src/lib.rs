use std::collections::HashSet;

type Groups = Vec<Vec<usize>>;

/// Create the maximized number of unique assignments given some minimum group
/// size.
pub fn make_assignments(conflicts: Vec<Vec<bool>>, min_group_size: usize) -> Vec<Groups> {
    todo!()
}

/// Try all ways to make the current assignment
pub fn single_assignment(conflicts: &mut Vec<Vec<bool>>, min_group_size: usize) -> Option<Groups> {
    todo!()
}

/// Get all possible ways to group a group of size k together.
pub fn potential_groups(conflicts: &mut Vec<Vec<bool>>, k: usize, skip: &HashSet<usize>) -> Vec<Group> {
    fn backtrack_row(
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
                    backtrack_row(conflicts, sols, curr, col, n, k, skip);
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
        backtrack_row(conflicts, &mut res, &mut curr, row, n, k, skip);
    }
    res
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
}
