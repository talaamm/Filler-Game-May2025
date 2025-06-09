To consistently win (or almost always win) at **Filler**, your bot needs to:

### ✅ Use a well-structured strategy with a strong evaluation function and efficient move generation

Let’s break down a **winning strategy** for your Filler AI bot:

---

## 🔁 1. Game Loop Strategy

### A. **Parse Input Efficiently**

- Read the Anfield and incoming piece from stdin.
- Convert them into internal data structures (e.g., `Vec<Vec<Cell>>`).

### B. **Generate All Valid Moves**

- Loop through every cell on the map.
- For each cell, check if placing the piece there:

  - Has **exactly one overlap** with your existing cells.
  - **Does not collide** with the enemy.
  - **Fully fits inside** the bounds.

### C. **Score Each Valid Move**

Use a heuristic function to **evaluate the quality** of each valid placement.

---

## 🧠 2. Heuristic Evaluation Function

Here are components of a strong scoring function:

### A. **Aggressive Expansion**

- Prefer placements that move **toward the enemy territory**.
- Use **Manhattan distance** between your placement and enemy cells to reward moves that get closer.

```rust
let enemy_proximity = closest_enemy_distance(x, y, &enemy_cells);
score -= enemy_proximity; // The closer to enemy, the better
```

### B. **Cutoff Strategy**

- Try to **cut off the enemy** from open regions of the map.
- You can simulate **flood fill** from enemy and your territory to see **who has more future potential area**.

```rust
let my_reach = flood_fill(&map, my_cells);
let enemy_reach = flood_fill(&map, enemy_cells);
score += my_reach.len() - enemy_reach.len();
```

### C. **Compactness / Shape Efficiency**

- Avoid long, thin pieces unless they help you reach a goal.
- Reward pieces that **maximize coverage** and **don't leave holes**.

---

## 🚫 3. Bad Move Filters

Before evaluating a move, filter out:

- Placements that **don’t touch** your existing area.
- Placements that **touch more than 1 cell** of your existing area.
- Placements that **collide** with the opponent.

---

## 🏃 4. Optimization: Early Game vs Late Game

### Early Game

- Expand fast.
- Take control of **central** and **high-access** areas.

### Mid Game

- Start **closing off regions**.
- Cut off the enemy from large spaces.

### Late Game

- Focus on **high coverage**.
- Block the enemy.
- Fill enclosed regions.

---

## 🧠 5. Advanced: Minimax (Bonus)

If you want to beat the **terminator bot**, you might need a simplified **Minimax search** with depth 1 or 2:

```rust
fn minimax(board, depth, is_my_turn) -> i32 {
    if depth == 0 { return evaluate(board); }
    let moves = generate_valid_moves(board, is_my_turn);
    if is_my_turn {
        moves.iter().map(|m| minimax(apply(m), depth-1, false)).max()
    } else {
        moves.iter().map(|m| minimax(apply(m), depth-1, true)).min()
    }
}
```

Use pruning or restrict number of evaluated moves for performance.

---

## 🛠️ 6. Technical Tips

- Use **Rust structs** for the map, pieces, and bot state.
- Precompute **enemy positions**, valid move offsets, and scoring zones.
- Use a **priority queue or sorted list** to track the best scoring placements.

---

## 🚀 TL;DR: Strategy Summary

| Phase                 | Strategy                                                                                |
| --------------------- | --------------------------------------------------------------------------------------- |
| **Early Game**        | Expand rapidly, aim for center, cover open areas                                        |
| **Mid Game**          | Start blocking enemy access, split the map                                              |
| **Late Game**         | Maximize placement, cut enemy off from remaining areas                                  |
| **Placement Scoring** | Closer to enemy (+), more open area reachable (+), isolated spot (–), blocked moves (–) |

---

Would you like a full working AI skeleton in Rust that follows this strategy?
