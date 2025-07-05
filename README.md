# Filler Game

A strategic algorithmic game where two robots compete to fill a grid with pieces, implemented in Rust.

Try out to play the game! [Game Demo](https://cosmic-grid-filler.lovable.app/)

## 🎮 Game Overview

Filler is an algorithmic game where two robots compete on a grid called the "Anfield". Players take turns placing randomly generated pieces on the grid, with the goal of occupying the largest surface area. The game ends when neither player can place any more pieces.

### Key Game Rules

- **Grid**: The Anfield is a 2D grid of arbitrary size
- **Pieces**: Randomly generated pieces of varying shapes and sizes
- **Placement**: Each piece must overlap exactly one cell with your previous territory
- **Scoring**: Points are earned for each successfully placed piece
- **Victory**: The player with the largest occupied area wins

### Game Symbols

- **Player 1**: `@` (territory) and `a` (last placed piece)
- **Player 2**: `$` (territory) and `s` (last placed piece)
- **Empty cells**: `.`

## 📁 Project Structure

```text
Filler-Game-May2025/
├── docker_image/          # Docker container and game engine
│   ├── Dockerfile         # Container configuration
│   ├── linux_game_engine  # Game engine for Linux
│   ├── m1_game_engine     # Game engine for M1 Macs
│   ├── maps/              # Game maps
│   │   ├── map00          # Small test map (20x15)
│   │   ├── map01          # Medium map (30x14)
│   │   └── map02          # Large map (100x100)
│   ├── linux_robots/      # Pre-built AI opponents
│   │   ├── bender         # Aggressive AI
│   │   ├── h2_d2          # Strategic AI
│   │   ├── terminator     # Advanced AI (challenge)
│   │   └── wall_e         # Basic AI
│   ├── m1_robots/         # M1 Mac compatible robots
│   └── solution/          # Your AI implementation
│       ├── Cargo.toml     # Rust project configuration
│       ├── src/
│       │   └── main.rs    # Main AI logic
│       └── game_log.txt   # Game execution logs
├── solution_test/         # Unit tests for AI logic
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs         # Test implementations
└── audit.txt              # Performance test results
```

## 🚀 Quick Start

### Prerequisites

- Docker installed on your system
- Rust toolchain (optional, for local development)

### Setup Instructions

1. **Navigate to the docker_image directory:**

   ```bash
   cd docker_image
   ```

2. **Build the Docker image:**

   ```bash
   docker build -t filler .
   ```

3. **Run the container:**

   ```bash
   docker run -v "$(pwd)/solution":/filler/solution -it filler
   ```

### Running Games

Inside the container, you can run games with different configurations:

```bash
# Basic game: Your AI vs Wall-E
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/filler

# Challenge game: Your AI vs Terminator
./linux_game_engine -f maps/map01 -p1 linux_robots/terminator -p2 linux_robots/filler

# Save game output to file
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/filler > /filler/solution/game_log.txt 2>&1
```

### Game Engine Options

- `-f, -file string`: Path to map file
- `-p1, -player1 string`: Path to first AI
- `-p2, -player2 string`: Path to second AI
- `-q, -quiet`: Quiet mode
- `-r, -refresh`: Throttling mode
- `-s, -seed int`: Use specific random seed
- `-t, -time int`: Set timeout in seconds (default 10)

## 🤖 AI Implementation

The current AI implementation (`docker_image/solution/src/main.rs`) uses a strategic approach:

1. **Valid Move Detection**: Finds all valid placement positions for the current piece
2. **Opponent Analysis**: Identifies opponent's territory positions
3. **Strategic Placement**: Chooses moves that minimize distance to opponent territory
4. **Fallback**: Returns `0 0` when no valid moves are available

### Key Functions

- `find_valid_placements()`: Identifies all valid piece placement positions
- `find_opponent_cells()`: Locates opponent's territory on the board
- `manhattan_distance()`: Calculates distance between positions

## 🧪 Testing

Run the test suite to verify AI logic:

```bash
cd solution_test
cargo test
```

The test suite covers:

- Manhattan distance calculations
- Valid move detection
- Opponent cell identification
- Edge cases and boundary conditions

## 🎯 Game Strategy

### Current Strategy

The AI implements a "territory expansion" strategy:

1. Always place pieces adjacent to existing territory
2. Prioritize moves that bring the AI closer to opponent territory
3. Maintain connectivity of territory
4. Adapt to different map sizes and piece shapes

## 📝 Game Protocol

### Input Format

The game engine sends data in this format:

```text
$$$ exec p1 : [robot_path]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
...
Piece 4 1:
.OO.
```

### Output Format

Respond with coordinates in format: `X Y\n`

- X: Column coordinate (0-based)
- Y: Row coordinate (0-based)
- Example: `7 2\n`

### Error Handling

- Return `0 0\n` when no valid moves are available
- Handle timeouts gracefully
- Validate all input data

---
