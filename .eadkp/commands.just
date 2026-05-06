current_target := shell("rustc -vV | grep \"host:\" | awk '{print $2}'")
project_name := `cargo metadata --format-version 1 --no-deps | jq -r '.packages[0].name'`

out_dir := "build/"
simulator_dir := "epsilon_simulator/"

build:
    just target
    cargo build --release --example device --target=thumbv7em-none-eabihf

build_simulator:
    cargo build --release --example simulator

send:
    cargo run --release --example device --target=thumbv7em-none-eabihf

check:
    cargo build --release --example device --target=thumbv7em-none-eabihf

export:
    just build
    rm -rf {{out_dir}} 2>/dev/null
    mkdir -p {{out_dir}}
    if mv target/thumbv7em-none-eabihf/release/examples/device {{out_dir}}{{project_name}}.nwa; then \
        echo -e "\n\n\033[1;92m{{project_name}} build successfully!\n\n-> $(realpath {{out_dir}}{{project_name}}.nwa)\033[0m\n"; \
    else \
        echo -e "\n\n\033[1;31mError: Build failed. No .nwa file found.\033[0m\n"; \
    fi

run_nwb:
    echo -e "\033[1;95mRunning simulator... (if it freezes, kill it with 'pkill epsilon.bin')\033[0m"
    ./epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/release/examples/libsimulator.so & # Run in background to free up terminal. If simulator freezes, kill it with `pkill epsilon.bin`.

sim jobs="1":
    -git clone https://github.com/numworks/epsilon.git epsilon_simulator -b version-20 --depth 1
    just build_simulator
    if [ -d "{{simulator_dir}}" ]; then \
        cd {{simulator_dir}}; \
        sudo rm -rf .git; \
        sudo make PLATFORM=simulator -j {{jobs}}; \
        cd ..; \
    fi
    just run_nwb

[confirm("This will clean the built app AND the simulator. Do you want to continue ?")]
clean:
    if [ -d "{{simulator_dir}}" ]; then \
        cd {{simulator_dir}}; \
        make clean; \
        cd ..; \
    fi
    cargo clean
    rm -rf {{out_dir}} 2>/dev/null

[confirm("This will clean the built app AND DELETE the simulator. Do you want to continue ?")]
clear:
    rm -rf {{simulator_dir}} 2>/dev/null
    cargo clean
    rm -rf {{out_dir}} 2>/dev/null

[confirm("This will update all dependencies to their latest versions. Do you want to continue ?")]
update:
    cargo update
    
target:
    rustup target add thumbv7em-none-eabihf

a:
    @echo "[ ... ] echo 'a'"