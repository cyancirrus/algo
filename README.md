# Algorithm Studies Repository

A personal collection of Rust solutions to algorithmic problems, designed to build and document problem-solving skills across various domains.

Why: This repository is a work journal for personal mastery in algorithms and Rust. It is not polished for tutorials but represents raw, consistent practice.

## Overview

- **Goal:** Practice and learn algorithms, data structures, concurrency, and Rust idioms.
- **Scope:** Covers classic problems like dynamic programming, graphs, strings, concurrency, and more.
- **Focus Areas:**  
  - Algorithm patterns  
  - Parallelism & async with Tokio and Rayon  
  - Data structures and optimization

## Progress Snapshot (as of July 2025)

- ✅ ~100+ algorithm problems solved
- ✅ Core DSA patterns: arrays, sliding windows, recursion, backtracking
- ✅ Dynamic Programming: subset sum, coin change, triangle min path
- ✅ Graph algorithms: DFS, BFS, Dijkstra's, islands, enclaves
- ✅ Bitmasking and Sudoku solvers
- ✅ Concurrency with async/await and channels
- ✅ Built from-scratch data structures: linked lists, tries, ring buffers

## Repository Structure

All solutions are in the `solutions/` folder as standalone Rust files:
```
solutions/
├── add_two_numbers.rs
├── async_fizzbuzz.rs
├── climbing_stairs.rs
├── coin_change.rs
├── fizzbuzz_channels.rs
├── graph_edge_explore.rs
├── levenshtein.rs
├── parallel_courses.rs
├── sudoku_solve.rs
├── twosum.rs
├── unique_paths.rs
└── ... (many more)
```

## Example: Async FizzBuzz with Tokio

This async version uses Tokio tasks and channels to handle FizzBuzz concurrently:

```rust, norun
async fn fizzbuzz(n: usize) {
    // Channels for fizz, buzz, fizzbuzz, and numbers
    let (fizz_tx, mut fizz_rx) = channel::<usize>(10);
    let (buzz_tx, mut buzz_rx) = channel::<usize>(10);
    let (fzbz_tx, mut fzbz_rx) = channel::<usize>(10);
    let (num_tx, mut num_rx) = channel::<usize>(10);

    // Spawn tasks for each category
    let fizz_handle = tokio::spawn(async move {
        while let Some(i) = fizz_rx.recv().await {
            println!("fizz {i}");
        }
    });
    // ... similar for buzz, fizzbuzz, and numbers ...

    // Send numbers to appropriate channels based on divisibility
    for i in 1..=n {
        match (i % 3 == 0, i % 5 == 0) {
            (false, false) => num_tx.send(i).await.unwrap(),
            (true, true) => fzbz_tx.send(i).await.unwrap(),
            (true, false) => fizz_tx.send(i).await.unwrap(),
            (false, true) => buzz_tx.send(i).await.unwrap(),
        }
    }

    // Close channels and await task completion
    drop(fizz_tx); drop(buzz_tx); drop(fzbz_tx); drop(num_tx);
    let _ = join!(fizz_handle, buzz_handle, fzbz_handle, num_handle);
}
```

# Future Plans: Become a better programmer

    Add more problem solutions and comments
    Organize by category/difficulty
    Explore async and parallel algorithms further


Feel free to ask if you'd like help adding templates or restructuring!

Want me to help format your project files with README headers or something similar?


# Quick Functions Resource 
```bash
cloc . --exclude-dir=target
```
