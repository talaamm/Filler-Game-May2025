To **pass the audit and complete the Filler project successfully**, you need to **meet both functional and qualitative requirements**. Below is a **detailed checklist and explanation** of everything you need to prepare, implement, and demonstrate to the auditor:

---

## ✅ **1. Functional Requirements**

### 📌 Docker Setup

* **Build the Docker image**:

  ```bash
  docker build -t filler .
  ```
* **Run the Docker container**:

  ```bash
  docker run -v "$(pwd)/solution":/filler/solution -it filler
  ```

  * Your AI player code must be in the `solution` directory.
  * You’ll compile and run your bot **inside the container**.

> 🔍 **Audit Question:** Can you confirm the student was able to create the image and container correctly?

### 📌 Game Execution Check

Run:

```bash
./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator
```

* This confirms your environment is functional.

> 🔍 **Audit Question:** Can you confirm the project runs correctly?

---

## ✅ **2. Implementing Your Player (Robot AI)**

Your program must:

* Read the **Anfield and piece** from `stdin`
* Analyze possible placements based on:

  * Overlap with **exactly one** of your territory cells (`@` or `$`)
  * No overlap with enemy pieces
* Output coordinates in this format:

  ```
  X Y\n
  ```

### 📌 Placement Logic

* Ensure your player always places pieces with **only one overlapping cell** from its territory.
* Ensure all pieces stay **within the Anfield** and don’t overlap the opponent.

> 🔍 **Audit Question:** Can you confirm that the student player is placing the pieces correctly with the overlapping of just one cell?

---

## ✅ **3. Testing Against Robots (Win Rate Validation)**

Run these tests **inside the container**, changing positions (p1/p2):

```bash
# Example for map00
./game_engine -f maps/map00 -p1 solution/my_player -p2 robots/wall_e
./game_engine -f maps/map00 -p1 robots/wall_e -p2 solution/my_player
... (5 total runs)

# Repeat similarly with:
# - robots/h2_d2 on map01
# - robots/bender on map02
```

You **must win at least 4 out of 5** times for each robot.

> 🔍 **Audit Questions:**

* Can you confirm that the student player won at least 4/5 against wall\_e?
* Can you confirm that the student player won at least 4/5 against h2\_d2?
* Can you confirm that the student player won at least 4/5 against bender?

---

## ✅ **4. Code Quality**

### 📌 Best Practices

* Clean structure and modular code
* Clear variable names
* No memory leaks (use Valgrind if writing in C)

> 🔍 **Audit Question:** Does the code obey good practices?

### 📌 Test File

* Provide a separate file with **unit tests** for key components like:

  * Valid placement detection
  * Parsing the input
  * Piece fitting logic

> 🔍 **Audit Question:** Is there a test file checking each possible case?

---

## ⭐ **BONUS (Optional but Valuable)**

### 📌 1. Graphic Visualizer

Create a visualizer (in Python, C++, JS, etc.) to show the game progression visually.

> 🔍 **Audit Question:** Did the student create a visualizer?

### 📌 2. Beat the Terminator

This is the toughest bot.

* You’ll run your bot 5 times against `terminator`:

```bash
./game_engine -f maps/mapX -p1 solution/my_player -p2 robots/terminator
./game_engine -f maps/mapX -p1 robots/terminator -p2 solution/my_player
```

> 🔍 **Audit Question:** Can you confirm the student beat the terminator 4/5 times?

This is **not required to pass** the project — it's bonus.

---

## 🎯 Summary of What You Must Have:

| Requirement                                | Status     |
| ------------------------------------------ | ---------- |
| Docker image built successfully            | ✅          |
| Docker container runs and compiles your AI | ✅          |
| AI places pieces with correct logic        | ✅          |
| AI wins 4/5 vs wall\_e                     | ✅          |
| AI wins 4/5 vs h2\_d2                      | ✅          |
| AI wins 4/5 vs bender                      | ✅          |
| Clean code + test file with good coverage  | ✅          |
| (Bonus) Visualizer made                    | ⭐ Optional |
| (Bonus) Beat terminator 4/5                | ⭐ Optional |

---

If you'd like help structuring your AI logic or optimizing it for better wins, let me know what language you're using (e.g., C, Rust), and I can help you step-by-step.
