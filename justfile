out_dir := "build/"
simulator_dir := "epsilon_simulator/"

build example="specs" variant="device":
    if [ "{{variant}}" = "device" ]; then \
        just target; \
        cargo build --release --example {{example}}-device --target=thumbv7em-none-eabihf; \
    elif [ "{{variant}}" = "simulator" ]; then \
        cargo build --release --example {{example}}-simulator; \
    else \
        echo "Unknown variant: {{variant}}"; \
        exit 1; \
    fi

send example="specs":
    cargo run --release --example {{example}}-device --target=thumbv7em-none-eabihf

check example="specs":
    cargo build --release --example {{example}}-device --target=thumbv7em-none-eabihf

export example="specs":
    just build {{example}} device
    rm -rf {{out_dir}} 2>/dev/null
    mkdir -p {{out_dir}}
    if mv target/thumbv7em-none-eabihf/release/examples/{{example}}-device {{out_dir}}{{example}}.nwa; then \
        echo -e "\n\n\033[1;92m{{example}} build successfully!\n\n-> $(realpath {{out_dir}}{{example}}.nwa)\033[0m\n"; \
    else \
        echo -e "\n\n\033[1;31mError: Build failed. No .nwa file found.\033[0m\n"; \
    fi

exports:
    for example in specs snake; do \
        just export "$example"; \
    done

[macos]
run_nwb example="specs":
    ./epsilon_simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/release/examples/lib{{example}}_simulator.dylib

[linux]
run_nwb example="specs":
    echo -e "\033[1;95mRunning simulator... (if it freezes, kill it with 'pkill epsilon.bin')\033[0m"
    ./epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/release/examples/lib{{example}}_simulator.so & # Run in background to free up terminal. If simulator freezes, kill it with `pkill epsilon.bin`.

sim example="specs" jobs="1":
    if [ ! -d "{{simulator_dir}}" ]; then \
        git clone https://github.com/numworks/epsilon.git {{simulator_dir}} -b version-20 --depth 1; \
    fi
    cargo build --release --example {{example}}-simulator
    if [ -d "{{simulator_dir}}" ]; then \
        cd {{simulator_dir}}; \
        rm -r .git 2>/dev/null;\
        make PLATFORM=simulator -j {{jobs}}; \
        cd ..; \
    fi
    just run_nwb {{example}}

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
