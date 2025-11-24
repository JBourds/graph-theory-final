# Graph Theory Final Project

UVM Fall 2025

Jordan Bourdeau

Writeup and implementation for MATH5678 final project.

- Code is located in the `src` directory.
- Project report is in the `manuscript.typ` file.

Looking into the problem of generating as many unique groups of at least size
`k` as possible given `n` students with some pre-existing conflicts.

What I've realized from working on this problem is that it is super-duper NP-hard.
A single iteration with `k = 3` and `n` is some factor of `k` is exactly the
3-dimensional matching problem which is NP-hard. This problem has the added
complexities of parity (not having a clean increment of `k` for `n`) and
repeating multiple rounds, with edges between the groups in each round being
deleted in following rounds.
