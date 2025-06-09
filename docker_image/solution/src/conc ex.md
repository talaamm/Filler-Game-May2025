# 🧪 **Example Input**

#### Anfield (20 * 15)

```txt
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 .........aa.........
004 ........a.a.........
005 ........aaa.........
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 .......ss$$.........
012 .........$$.........
013 ....................
014 ....................
```

#### Piece

```txt
Piece 2 2:
..
OO
```

---

### 🔍 Step-by-Step Tracking

Let’s simulate how the code finds the **best move**.

#### 1. **Gather Symbols**

```rust
let (my_syms, opp_syms) = (vec!['@', 'a'], vec!['$', 's']);
```

#### 2. **Find All Opponent Positions**

```rust
opponent_cells = vec![
    (11, 7), (11, 8), (11, 9), (11, 10),
    (12, 9), (12, 10)
]
```

#### 3. **Find All Valid Placements**

Let's imagine we test placing the `Piece` at position (4, 7). Here's the overlay:

```text
Piece:
..
OO

    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 .........aa.........
004 ........a.a.........
005 .......PPaa.........
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 .......ss$$.........
012 .........$$.........
013 ....................
014 ....................
```

#### SO, (4, 7) is a valid move

Try (4, 9):

```text
005[9]='a' , 005[10]='a'
overlap = 2 ❌
```

Keep collecting those that have `overlap == 1` and no opponent cells under the piece.

Suppose valid placements include:

```txt
(4,7), (4,10), (4,11), (3,8)...
```

---

### 🧮 Calculate Manhattan Distance

Now, for each valid move, calculate:

```rust
opponent_cells = [(11,7), (11,8), (11,9), (11,10), (12,9), (12,10)]
```

#### Example: Move = (4, 7)

Closest opponent:

```txt
Manhattan((4,7), (11,7)) = |4-11| + |7-7| = 7 + 0 = 7
```

Check all others:

* (4,7) to (11,8) → 8
* (4,7) to (11,9) → 9
* (4,7) to (11,10) → 10
* (4,7) to (12,9) → 10
* (4,7) to (12,10) → 11

**Minimum = 7 (to (11,7))**

Do this for each valid move and pick the one with the **smallest minimum Manhattan distance**.

For (4,10):
(4,10) to (11,7) → 10
(4,10) to (11,8) → 9
(4,10) to (11,9) → 8
(4,10) to (11,10) → 7
(4,10) to (12,9) → 9
(4,10) to (12,10) → 8

**Minimum = 7 (to (11,10))**

For (4,11):
(4,11) to (11,7) → 11
(4,11) to (11,8) → 10
(4,11) to (11,9) → 9
(4,11) to (11,10) → 8
(4,11) to (12,9) → 0
(4,11) to (12,10) → 9

**Minimum = 8 (to (11,10))**

(3,8) to (11,7) → 9
(3,8) to (11,8) → 8
(3,8) to (11,9) → 9
(3,8) to (11,10) → 10
(3,8) to (12,9) → 10
(3,8) to (12,10) → 11

**Minimum = 8 (to (11,8))**

---

### ✅ **Final Pick**

Suppose this gives:

```rust
Move (4, 7): min_dist = 7 ✅ best
Move (4, 10): min_dist = 7  
Move (4, 11): min_dist = 8
Move (3, 8): min_dist = 8
...
```

Then the AI picks **(4, 7)** and prints:

```txt
7 4
```

### 🔁 Summary of Flow

```text
1. Read input (Anfield + Piece)
2. Find all valid placements
3. Find opponent positions
4. For each placement:
   a. Measure Manhattan distance to each opponent cell
   b. Take the min
5. Pick the move with smallest min-distance
6. Print (x y)
```

---

## How does `min_by_key` work in this code?

### The mapping inside `min_by_key`

```rust
let best_move = valid_moves.iter().min_by_key(|&&(y, x)| {
    opponent_cells
        .iter()
        .map(|&(oy, ox)| manhattan_distance((y, x), (oy, ox)))
        .min()
        .unwrap_or(usize::MAX)
});
```

---

---

## 🌟 How Does `min_by_key` Track the Best Move?

The key insight is that the iterator is over references to the moves themselves (`&(y, x)`), and `min_by_key` only needs the key (the minimum distance) to compare the moves. But it always keeps track of **which move produced that key**.

### 🛠️ How does it work?

1. `valid_moves.iter()` produces an iterator over references to each move: `&(y, x)`.
2. For each move, you calculate a key (the minimum Manhattan distance to any opponent cell).
3. `min_by_key` compares these keys and remembers which move produced the smallest key.
4. When it’s done, it returns a reference to the move (i.e., `&(y, x)`) that had the smallest key.

**So:**

* The key is just used for comparison.
* The iterator always knows which `(y, x)` produced each key.
* The result is a reference to the move itself, not the key.

---

### 📋 Example

Suppose `valid_moves = vec![(4,7), (4,11), (5,7)]`:

* For `(4,7)`, key = 7
* For `(4,11)`, key = 7
* For `(5,7)`, key = 6

`min_by_key` will return a reference to `(5,7)` because its key (6) is the smallest.

---

### 🏷️ Type

The result is `Option<&(usize, usize)>` — a reference to the move with the smallest key.

---

### 📝 Summary

* The iterator keeps track of both the move and its key, so you always know which `(y, x)` is the best move.
* **No map or dictionary is needed!**

---
