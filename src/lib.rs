//! # Group-Assignment Generator
//!
//! This module provides a backtracking engine for generating **all possible
//! multi-round groupings** of a fixed set of vertices such that:
//!
//! - Every round partitions all vertices into groups.
//! - No group is smaller than a specified minimum size.
//! - No pair of vertices may ever be grouped together more than once
//!   (a *conflict constraint*).
//! - The generator returns **only those assignments that achieve the maximum
//!   number of possible rounds**.
//!
//! The design was motivated by scheduling problems such as:
//! - round-robin small-group assignments,
//! - classroom partner rotation,
//! - avoiding repeated team composition,
//! - tournament or workshop session planning.
//!
//! ## Core Concepts
//!
//! ### Vertices
//! Each vertex is an integer index `0..n-1`.  
//! The system never stores custom data—just relationships (conflicts).
//!
//! ### Conflict Matrix
//! The `conflicts: Vec<BitVec>` parameter is a square boolean adjacency
//! matrix where `conflicts[i][j] == true` means vertices `i` and `j` may **not**
//! appear in the same group.  
//!
//! Conflicts are updated *dynamically* while exploring group combinations:
//! - When building a group, temporary conflicts are injected to prune search.
//! - When completing a round, the chosen groups’ members are permanently
//!   marked as conflicting so they cannot reappear together in later rounds.
//!
//! This mutability is what allows multi-round generation without storing
//! explicit history.
//!
//! ### Groups and Rounds
//! A **group** is a `Vec<usize>`.  
//! A **round** is a `Vec<Group>`: a full partition of all vertices.  
//! An **assignment** is a `Vec<Vec<Group>>`: one valid sequence of rounds.  
//!
//! The solver returns a `Vec<Vec<Vec<Group>>>`, meaning:
//!
//! ```text
//! // Many possible assignments
//! Vec<
//!     // Single assignment of groups with multiple rounds
//!     Vec<
//!         // Single round, containing groups with predetermined group size
//!         Vec<
//!             // Single group, containing a vector of vertex indices
//!             Group
//!         >
//!     >
//! >
//! ```
//!
//! ### Group Size Planning
//!
//! A predetermined vector of group sizes for each round is produced by
//! [`group_sizes()`]. Groups must meet or exceed the minimum size, and any
//! leftover vertices are distributed as evenly as possible across groups.
//!
//! This ensures each round has a fixed shape (e.g., `[3,2,2]` for `7`
//! vertices with minimum group size 2).
//!
//! ## Algorithm Summary
//!
//! 1. **Determine per-round group sizes** (`group_sizes()`).
//! 2. **Enumerate all conflict-valid rounds** given the current conflict state
//!    (`single_assignment()`).
//! 3. For each round found:
//!    - Commit the round, marking all pairs within each group as conflicting.
//!    - Recursively attempt to build additional rounds.
//! 4. Once no more rounds are possible, compare the number reached to the
//!    best found so far.
//! 5. Return **all** assignments that have the maximal number of rounds.
//!
//! The search is fully exhaustive but implements several mechanisms to make it
//! faster than naïve combinatorial enumeration:
//!
//! - Efficient adjacency matrix/skiplist storage using bit vectors (providing
//!   a modest, constant factor speedup over less compact approaches.
//! - Pruning recursion tree by not including invalid groups (as determined via
//!   conflicts list) during recursive calls.
use bitvec::prelude::*;

/// Vector of vertex indices corresponding to one group
type Group = Vec<usize>;

/// Generate all possible *maximum-round* group assignments such that:
///
/// - No two vertices that have previously been in a group together may be
///   grouped again.
/// - All groups satisfy a minimum size constraint.
/// - All rounds must partition all vertices.
/// - Only assignments achieving the **maximum possible number of rounds**
///   are returned.
///
/// # Arguments
///
/// - `conflicts`:  
///   A square adjacency matrix (bit-matrix) where `conflicts[i][j] == true`
///   means that vertices `i` and `j` may **not** be placed in the same group.
///   This matrix gets *updated* as groups are tentatively formed during
///   backtracking, but is always returned to its previous state.
///
/// - `min_group_size`:  
///   The minimum allowed group size. Groups may be larger if needed for an
///   even partition.
///
/// # Returns
///
/// A vector of assignment possibilities. All returned assignments achieve the 
/// same maximal number of rounds.
///
/// # Panics
///
/// Panics if the `conflicts` matrix is empty, not square, or has fewer vertices
/// than required by `min_group_size`.
pub fn make_assignments(conflicts: &mut Vec<BitVec>, min_group_size: usize) -> Vec<Vec<Vec<Group>>> {
    assert!(conflicts.len() > 0, "Cannot make groups from 0 vertices.");
    assert!(conflicts.iter().all(|v| v.len() == conflicts.len()), "Conflicts matrix must have matching dimensions (N x N)");
    assert!(min_group_size <= conflicts.len(), "Cannot require groups larger than the number of potential vertices.");

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

/// Compute the group sizes for a single round, given
/// `n` total vertices and a minimum group size `min_group_size`.
///
/// Produces a vector of sizes that:
///
/// - All sum to exactly `n`.
/// - Are each at least `min_group_size`.
/// - Distribute any leftover vertices as evenly as possible.
///
/// # Example
///
/// ```
/// assert_eq!(group_sizes(7, 2), vec![3, 2, 2]);
/// ```
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

/// Generate all possible **single-round** assignments respecting current
/// conflict constraints.
///
/// A *single assignment* is one full round of grouping using the provided
/// `group_sizes` (which must sum to the total vertex count).
///
/// # Behavior
///
/// - Each group in the round is conflict-free according to `conflicts`.
/// - Once a vertex is placed in a group, it is marked as “skipped” for the
///   remainder of this round.
/// - All assignments are returned; no maximality filtering occurs here.
///
/// # Returns
///
/// A list of all valid ways to construct one round:
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

/// Enumerate all possible **groups of size `k`** that are valid given the
/// conflict matrix and current "skip" mask.
///
/// A group is valid if:
///
/// - It contains exactly `k` vertices.
/// - None of the vertices are marked in `skip` (already chosen).
/// - No pair inside the group has a conflict (`conflicts[i][j] == true`).
///
/// The function **temporarily** marks conflict edges while exploring deeper
/// combinations to prune invalid partial groups.
///
/// # Arguments
///
/// - `conflicts`: mutable matrix tracking current conflict constraints.
/// - `k`: required group size.
/// - `skip`: bitmask of vertices that are already taken in this round.
///
/// # Returns
///
/// Eery valid `k`-set of vertex indices. 
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

/// Mark all pairs inside `between` as mutually conflicting.
#[inline]
fn add_conflicts_between(conflicts: &mut Vec<BitVec>, between: &[usize]) {
    for i in between {
        for j in between {
            conflicts[*i].set(*j, true);
        }
    }
}

/// Remove all conflicts previously added by `add_conflicts_between`.
#[inline]
fn remove_conflicts_between(conflicts: &mut Vec<BitVec>, between: &[usize]) {
    for i in between {
        for j in between {
            conflicts[*i].set(*j, false);
        }
    }
}

/// Add conflicts between one vertex `col` and all vertices from an iterator.
#[inline]
fn add_conflicts<'a>(conflicts: &mut Vec<BitVec>, col: usize, rows: impl Iterator<Item = &'a usize>) {
    for row in rows {
        conflicts[*row].set(col, true);
        conflicts[col].set(*row, true);
    }
}

/// Remove conflicts previously added by `add_conflicts`.
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
