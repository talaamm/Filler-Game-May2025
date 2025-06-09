Great question. Here's a **clear explanation** of what the Filler project is all about and **what you need to do to complete it successfully**:

---

## 🧠 **Project Overview: What is Filler?**

Filler is a **two-player strategy game** where your program (a "robot player") competes against another robot to **fill as much space as possible** on a 2D grid (the "Anfield").

Each turn:

* The game engine gives your program a **randomly shaped piece**.
* You must place that piece on the board so that:

  * It **touches your existing territory by exactly one cell**.
  * It **does not overlap the enemy’s territory** or go outside the board.

Your objective is to write an **AI program** that:

* Reads the game state and the new piece.
* Calculates the **best possible placement**.
* Outputs the **coordinates** to place the piece.

The winner is the player who covers the **most territory** when neither can play anymore.

---

## 🛠️ **Your Tasks**

### ✅ **1. Write the AI (your robot)**

This is your main job!

You will write a **C (or Rust, if permitted)** program that:

* Reads from standard input:

  * Player number (`$$$ exec p1 : ...`)
  * The current state of the Anfield (grid)
  * The piece to be placed
* Computes the best (valid) place to put the piece.
* Outputs to standard output:

  * The coordinates where to place the piece (e.g., `7 2\n`)

---

### ✅ **2. Dockerize your solution**

You need to:

* Use the provided Dockerfile and folder structure.
* Place your AI inside the `/solution` folder.
* Build the docker image using `docker build -t filler .`
* Run the container with your solution using:

  ```bash
  docker run -v "$(pwd)/solution":/filler/solution -it filler
  ```

This simulates the game in an isolated environment (like the evaluators will).

---

### ✅ **3. Test your player**

You must run your player against the provided robots:

```bash
./game_engine -f maps/map01 -p1 solution/your_player -p2 robots/bender
```

Swap player positions and run **5 times** to prove your AI wins consistently.

---

### ✅ **4. Handle input correctly**

* Read and parse the board and piece.
* Store their dimensions and contents.
* Make sure you detect:

  * Which positions are yours
  * Where the piece could legally be placed (touching your area by **only one cell**)
* Output the correct position.

---

### ✅ **5. Make your AI smart**

At first, your AI can be simple—just find **any valid** move.

Then, improve it to:

* Prefer moves that block the enemy.
* Expand aggressively.
* Prioritize corners or center depending on your strategy.

---

### ✅ **6. (Optional but Recommended) Create a visualizer**

Bonus points if you make a visualizer to show the game in action (e.g., in the terminal or using a graphical tool).

---

### ✅ **7. (Bonus+) Beat the Terminator**

This is the strongest robot. If your bot can beat it **4/5 times**, you’ll ace the bonus audit.

---

## ✅ To Pass the Audit:

You must:

* ✅ Build and run your Docker image and container.
* ✅ Write a player that parses input, outputs correct moves.
* ✅ Beat `wall_e`, `h2_d2`, and `bender` at least 4 out of 5 times each.
* ✅ Follow clean coding practices and (optionally) write test files.
* ✅ Handle edge cases, errors, and invalid inputs gracefully.
* ✅ (Bonus) Beat the `terminator` robot.
* ✅ (Bonus) Build a visualizer.

---

## ✅ Summary of Deliverables

| Item                                  | Required? |
| ------------------------------------- | --------- |
| Dockerized project                    | ✅ Yes     |
| Functional AI that plays the game     | ✅ Yes     |
| Ability to beat bender/h2\_d2/wall\_e | ✅ Yes     |
| Clean code and (if possible) tests    | ✅ Yes     |
| Visualizer                            | 🔁 Bonus  |
| Beats terminator 4/5 times            | 🔁 Bonus  |

---

If you'd like, I can also help you design a **step-by-step plan to implement the AI logic**, parse the input, and test it—just let me know your preferred language (C or Rust), and I’ll walk you through it.
