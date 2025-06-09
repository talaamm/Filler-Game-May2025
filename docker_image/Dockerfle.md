# Dockerfile Explanation

Below is a line-by-line explanation of your Dockerfile:

---

```dockerfile
FROM rust:1.77
```

**Purpose**: Sets the base image for the build.

**Explanation**: Uses the official Rust 1.77 image, which includes Rust tooling and a Linux environment.

---

```dockerfile
COPY ./maps                 /filler/maps
COPY ./linux_robots         /filler/linux_robots
COPY ./m1_robots            /filler/m1_robots
COPY ./linux_game_engine    /filler/linux_game_engine
COPY ./m1_game_engine       /filler/m1_game_engine
COPY ./solution             /filler/solution
```

**Purpose**: Copies files and directories from your project into the Docker image.

**Explanation**:

* `./maps` → `/filler/maps`: Game maps.
* `./linux_robots` → `/filler/linux_robots`: Robot binaries/scripts for Linux.
* `./m1_robots` → `/filler/m1_robots`: Robot binaries/scripts for M1/Mac.
* `./linux_game_engine` → `/filler/linux_game_engine`: Game engine for Linux.
* `./m1_game_engine` → `/filler/m1_game_engine`: Game engine for M1/Mac.
* `./solution` → `/filler/solution`: Your Rust solution source code.

---

```dockerfile
WORKDIR /filler/solution
```

**Purpose**: Sets the working directory for subsequent commands.

**Explanation**: All following commands will run inside `/filler/solution`.

---

```dockerfile
RUN cargo build --release
```

**Purpose**: Builds your Rust project in release mode.

**Explanation**: Produces an optimized binary in target/release.

---

```dockerfile
RUN cp target/release/filler /filler/linux_robots/filler && chmod +x /filler/linux_robots/filler
```

**Purpose**: Copies the built binary to the robots folder and makes it executable.

**Explanation**:

* `cp`: Copies the compiled filler binary.
* `chmod +x`: Ensures the binary is executable.

---

```dockerfile
WORKDIR /filler
```

**Purpose**: Changes the working directory for the container's default shell.

**Explanation**: Sets /filler as the default directory when the container starts.

---

```dockerfile
ENTRYPOINT /bin/bash
```

**Purpose**: Sets the default command to run when the container starts.

**Explanation**: Opens a Bash shell, so you can interact with the container.

---

Release mode in Rust refers to building your project with optimizations enabled, making the resulting binary faster and smaller.

By default, cargo build creates a debug build (with extra info for debugging, but slower and larger).

cargo build --release creates a release build (optimized for speed and efficiency, but harder to debug).

tells Cargo to compile your Rust code with optimizations, producing the binary in target/release/.
