🍏 **On Mac (or Linux Terminal)**
<br>

---

# 🚀 Running the Docker Image

## 🍏 **On Mac (or Linux Terminal)**

### **Quick Commands**

```sh
cd docker_image
docker build --platform linux/amd64 -t filler .
docker run --platform linux/amd64 -v "$(pwd)/solution":/filler/solution -it filler
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator > /filler/solution/game_log.txt 2>&1
```

---

### **Explanation**

- `cd docker_image`  
    Change directory to where your Dockerfile and related files are.

- `docker build --platform linux/amd64 -t filler .`  
    Build the Docker image for the Linux amd64 architecture and tag it as `filler`.

- `docker run --platform linux/amd64 -v "$(pwd)/solution":/filler/solution -it filler`  
    Run the container, mounting your local `solution` folder into the container at `/filler/solution`.  
    The `-it` flag provides an interactive terminal.

- `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator > /filler/solution/game_log.txt 2>&1`  
    Run the game engine inside the container, saving all output (including errors) to `game_log.txt` in your mounted solution folder.

---

## 📝 **Step-by-step Instructions**

### 1. Change Directory

```sh
cd docker_image
```

_Navigate to the folder containing your Dockerfile and related files._

### 2. Build the Docker Image

```sh
docker build --platform linux/amd64 -t filler .
```

_Builds the Docker image for the Linux amd64 architecture and tags it as `filler`._

### 3. Run the Docker Container

```sh
docker run --platform linux/amd64 -v "$(pwd)/solution":/filler/solution -it filler
```

_Starts the container, mounting your local `solution` folder into the container at `/filler/solution`.<br>
The `-it` flag provides an interactive terminal._

### 4. Run the Game Engine

```sh
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator > /filler/solution/game_log.txt 2>&1
```

_Executes the game engine inside the container, saving all output (including errors) to `game_log.txt` in your mounted solution folder._

---

## 🪟 **On Windows PowerShell**

### **Quick Commands**

```powershell
cd docker_image
docker build --platform linux/amd64 -t filler .
docker run --platform linux/amd64 -v "C:\Users\lenovo\Desktop\filler-2\docker_image\solution:/filler/solution" -it filler
apt-get update && apt-get install -y dos2unix
dos2unix maps/map00
dos2unix maps/map01
dos2unix maps/map02
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/filler > /filler/solution/1_ben_fi.txt 2>&1
```

---

### **Explanation**

- `cd docker_image`  
    Change directory to your Docker context.

- `docker build --platform linux/amd64 -t filler .`  
    Build the Docker image for Linux amd64.

- `docker run --platform linux/amd64 -v "C:\Users\lenovo\Desktop\filler-2\docker_image\solution:/filler/solution" -it filler`  
    Run the container, mounting your Windows solution folder into the container at `/filler/solution`.

- `apt-get update && apt-get install -y dos2unix`  
    Update package lists and install `dos2unix` (needed to convert Windows line endings to Unix).

- `dos2unix maps/map00`, `dos2unix maps/map01`, `dos2unix maps/map02`  
    Convert map files to Unix line endings to avoid "invalid map" errors.

- `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/filler > /filler/solution/1_ben_fi.txt 2>&1`  
    Run the game engine, saving all output to `1_ben_fi.txt` in your solution folder.
