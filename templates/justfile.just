current_target := shell("rustc -vV | grep \"host:\" | awk '{print $2}'")
project_name := `cargo metadata --format-version 1 --no-deps | jq -r '.packages[0].name'`

build:
    cargo build --release --example device --target=thumbv7em-none-eabihf

send:
    cargo run --release --example device --target=thumbv7em-none-eabihf

check:
    cargo build --release --example device --target=thumbv7em-none-eabihf

export:
    just build
    rm -rf build
    mkdir -p build
    mv target/thumbv7em-none-eabihf/release/examples/device build/example.nwa
    echo -e "\n\n\033[1;92mEadkp example app build successfully!\n\n-> $(realpath build/example.nwa)\033[0m\n"

[macos]
run_nwb:
    ./epsilon_simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/release/examples/libsimulator.dylib

[linux]
run_nwb:
    ./epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/release/examples/libsimulator.so

sim jobs="1":
    -git clone https://github.com/numworks/epsilon.git epsilon_simulator -b version-20 # Broken with version 21. Nice!
    cargo build --release --example simulator
    if [ ! -f "target/simulator_patched" ]; then \
        cd epsilon_simulator; \
        rm -r .git; \
        make PLATFORM=simulator -j {{jobs}}; \
        cd ..; \
        echo "yes it is" >> target/simulator_patched; \
    fi
    just run_nwb

[confirm("This will clean the built app AND the simulator. Do you want to continue ?")]
clean:
    cd ./epsilon_simulator && make clean
    cargo clean
    rm -rf ./build

[confirm("This will clean the built app AND DELETE the simulator. Do you want to continue ?")]
clear:
    rm -rf ./epsilon_simulator
    cargo clean
    rm -rf ./build
