# For release
cargo build --release
// first run on mac is slower has to be validated

# For godbolt
[check compilation](godbolt.org);
compile with `-C opt-level=2`

# Closures
fn add_n(x:u8, y:u8) -> impl Fn(u8){
    move |y| x + y 
}

# algo
A personal collection of algorithm practice problems, data structures, and experimental Rust implementations.

## ✨ What's in here

- **Solutions** to various algorithmic problems — primarily from LeetCode — with first-pass implementations written solo.
- Follow-up iterations with optimizations guided by research or discussions (e.g., GPT, documentation, articles), but always understood and re-implemented, not copied.
- Custom-built data structures in `src/` like:
  - `list.rs`, `stack.rs`, `deque.rs`: exploring classic structures
  - `tree.rs`: basic tree from scratch (stopped before diving into `unsafe`)
  - `memory.rs`: initial experiments with manual layout understanding

## 📁 Structure
