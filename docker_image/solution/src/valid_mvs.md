# 🧠 **Function Purpose**

This function finds all the **valid positions** on the game board (`anfield`) where a given `piece` can be placed according to specific rules:

* The piece must **overlap exactly one cell** that belongs to the player.
* The piece must **not touch any opponent cells**.
* The rest of the piece must go on empty or neutral cells (denoted by `.`).

---

## 📦 **Function Signature**

```rust
fn find_valid_placements(
    anfield: &Vec<Vec<char>>,  // 2D grid representing the game board
    piece: &Vec<Vec<char>>,    // 2D grid representing the piece shape
    player_num: usize,         // 1 or 2, identifying the player
) -> Vec<(usize, usize)>       // Returns list of top-left coordinates (y, x)
```

---

## 🔢 **Variable Initialization**

```rust
let height = anfield.len();
let width = anfield[0].len();
let piece_height = piece.len();
let piece_width = piece[0].len();
```

This sets up dimensions:

* `height` and `width`: dimensions of the board.
* `piece_height` and `piece_width`: dimensions of the piece.

---

## 👥 **Player Symbol Mapping**

```rust
let (my_syms, opp_syms) = if player_num == 1 {
    (vec!['@', 'a'], vec!['$', 's'])
} else {
    (vec!['$', 's'], vec!['@', 'a'])
};
```

This maps the player's and opponent's cells:

* Player 1: owns `@`, `a`; opponent has `$`, `s`.
* Player 2: reversed.
  These are likely symbols used in a strategy game for main and supporting units.

---

## 🔍 **Main Loop — Scanning the Board**

```rust
for y in 0..=height.saturating_sub(piece_height) {
    for x in 0..=width.saturating_sub(piece_width) {
```

This iterates over all possible **top-left positions** `(y, x)` where the piece can *fully fit* inside the board.

* `saturating_sub` prevents underflow if the piece is larger than the board (gracefully results in 0).

---

## 🧮 **Inner Loop — Validating a Placement**

```rust
let mut overlap = 0;
let mut valid_pos = true;
```

We track:

* `overlap`: number of cells the piece overlaps with the player's own symbols.
* `valid_pos`: if the piece touches opponent cells (becomes `false`).

---

### 🔁 Nested Loops — Over the Piece

```rust
for dy in 0..piece_height {
    for dx in 0..piece_width {
        if piece[dy][dx] != '.' {
```

This loop scans each cell in the piece:

* `dy`, `dx`: relative offsets in the piece.
* Only non-`.` (i.e., active) piece cells are considered.

---

### 📍 Checking Board Cell

```rust
let cell = anfield[y + dy][x + dx];
if my_syms.contains(&cell) {
    overlap += 1;
} else if opp_syms.contains(&cell) {
    valid_pos = false;
}
```

This checks:

* If the piece cell overlaps the **player’s own cell**, `overlap` is incremented.
* If it touches **opponent’s cell**, the whole position is invalid.

---

## ✅ **Adding Valid Position**

```rust
if valid_pos && overlap == 1 {
    valid.push((y, x));
}
```

* The position is valid **only if**:

  * It does not overlap opponent cells.
  * It overlaps **exactly one** of the player’s own cells.

This exact-1 rule enforces a connection to the player’s territory while preventing capturing or illegal contact.

---

## 🔚 **Return Result**

```rust
valid
```

Returns a list of all top-left positions where the piece can legally be placed.

---

## 📌 Summary

* This function implements a **game rule engine** for piece placement.
* It’s similar in spirit to games like **Go**, **Blokus**, or **Filler** in AI tournaments.
* Each placement must:

  * Fit inside the board
  * Overlap exactly 1 friendly cell
  * Avoid opponent cells
  * Respect the shape of the piece

---

Would you like to walk through this function with a specific `anfield` and `piece` example for a visual/step-by-step case?

---

Great question.

### ✅ Understanding `width.saturating_sub(piece_width)`

```rust
width.saturating_sub(piece_width)
```

This computes:

```txt
20.saturating_sub(4) → 16
```

### 🤔 Why use `saturating_sub`?

Because `saturating_sub` ensures there's **no panic** or **underflow** if the subtraction would go negative.

* Example: `3 - 5 = -2`, which panics in Rust if done with `usize - usize`.
* But `3.saturating_sub(5)` = `0`

### ✅ Loop Range

```rust
for x in 0..=width.saturating_sub(piece_width)
```

With:

* `width = 20`
* `piece_width = 4`

You get:

```rust
for x in 0..=16  // x goes from 0 to 16 inclusive
```

### 🔢 Why up to 16?

So that the **right edge** of the piece (x + 4) doesn't go beyond the board width (20).

```txt
x in 0..=16
→ x + 0..3 for the piece
→ max x + 3 = 19, still within board
```

This ensures that the piece **fully fits** within the board horizontally.

Let me know if you want a visual layout or to see it applied to an example board!
