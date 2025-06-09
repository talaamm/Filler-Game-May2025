FROM rust:1.77

COPY ./maps                 /filler/maps
COPY ./linux_robots         /filler/linux_robots
COPY ./m1_robots            /filler/m1_robots
COPY ./linux_game_engine    /filler/linux_game_engine
COPY ./m1_game_engine       /filler/m1_game_engine
COPY ./solution             /filler/solution

WORKDIR /filler/solution

# Build the solution for release (Linux x86_64)
RUN cargo build --release
# RUN ls -l target/release
RUN cp target/release/filler /filler/linux_robots/filler && chmod +x /filler/linux_robots/filler
# Copy the built binary to the robots folder and make it executable

WORKDIR /filler

ENTRYPOINT /bin/bash