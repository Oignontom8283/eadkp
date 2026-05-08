out_dir := "build/"
simulator_dir := "epsilon_simulator/"

# List all example files (absolute path) excluding _lib files
list_examples:
    @find {{`realpath examples`}} -name "*.rs" ! -name "*_lib.rs"

# Get the name of an example from its path
example_name path:
    @basename -s .rs {{path}}

send example="specs":
    just check {{example}}
    cargo run --release --example {{example}}-device --target=thumbv7em-none-eabihf

check example="specs":
    cargo build --release --example {{example}}-device --target=thumbv7em-none-eabihf

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

export example="specs" remove="true":
    just build {{example}} device
    if {{remove}}; then \
        rm -rf {{out_dir}} 2>/dev/null; \
    fi
    mkdir -p {{out_dir}}
    if mv target/thumbv7em-none-eabihf/release/examples/{{example}}-device {{out_dir}}{{example}}.nwa; then \
        echo -e "\n\n\033[1;92m{{example}} build successfully!\n\n-> $(realpath {{out_dir}}{{example}}.nwa)\033[0m\n"; \
    else \
        echo -e "\n\n\033[1;31mError: Build failed. No .nwa file found.\033[0m\n"; \
    fi

exports:
    rm -rf {{out_dir}} 2>/dev/null
    for example_path in `just list_examples`; do \
        just export `just example_name "$example_path"` "false"; \
    done

run_nwb example="specs":
    @echo -e "\033[1;95mRunning simulator... (if it freezes, kill it with 'pkill epsilon.bin')\033[0m"
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

list_build_dir:
    @echo "Build directory:"
    @ls -lah build/ || true

list_nwa_symbols:
    @echo "NWA symbols:"
    @for nwa in `find build -name "*.nwa"`; do \
        echo "Symbols for $nwa:"; \
        arm-none-eabi-nm -C $nwa; \
    done

list_binaries:
    @echo "Device examples binaries:"
    @ls -lah target/thumbv7em-none-eabihf/release/examples/ || true

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
