# Commands and Export

## Building and Exporting

Running `just build` or `just export` both compile your project. The difference is that `just export` also packages the output as a `.nwa` file — which is simply the compiled binary renamed and placed in a dedicated folder for easy distribution. Use `just export` when you want a file to install on a calculator or share with others.


## Command Reference

### Build, Run and Export

#### Build
```bash
just build
```

#### Run the simulator
```bash
just sim {number_of_cores}
```
See the [Simulator](./simulator.md) page for details.


#### Export
```bash
just export
```
Generates the `.nwa` file in the `build/` folder.

> [!INFO]
> The output folder name may change in future updates.


### Cleanup

#### Clean project files
```bash
just clean
```
Deletes the compiled output of your application (`target/` and `build/` folders).


#### Full reset
```bash
just clear
```
Same as `just clean`, but also removes the compiled simulator files.


### Configuration and Maintenance

#### Download the compilation target
```bash
just target
```
Manually downloads the required compilation target. Normally done automatically on first build.


#### Update local tools
```bash
./update.sh
```
Updates project scripts and tooling (`docker.sh`, `update.sh`, `justfile`, etc.).


### Internal and Deprecated Commands

These are used by internal scripts or for specific test cases. **No need to run them manually.**


#### `just check` *(deprecated)*

Identical to `just build`. Use `just build` instead.


#### `just build_simulator`

Compiles the app for the simulator without launching it. Used internally to verify simulator compatibility. Use `just sim` instead.
