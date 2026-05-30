
# Commands and Export

It is important to clearly distinguish between building and exporting your application:

* **Build:** Only compiles the source code to test your project on the calculator or the simulator. No distribution file is generated.
* **Export:** Compiles the project and generates a final file in `.nwa` format. This is the file you can install on your calculator or share with other users.

## Command Reference

### Build, Execution and Export

#### Build for the calculator

```bash
just build

```

#### Launch the simulator

```bash
just sim {number_of_cores}

```

This is the **recommended command** to test your application on a computer. Replace `{number_of_cores}` with the number of CPU cores you wish to allocate to the compilation.
*Note: If the simulator has not yet been compiled on your machine, this command will automatically compile it before launching.*

#### Export the application

```bash
just export

```

Generates the application file (`.nwa`) in the `build/` folder of your project.

> [!INFO]
> The default destination folder name might change in future updates (for example, to `dist/`, `out/`, etc.).

### Cleanup

#### Clean the project

```bash
just clean

```

Deletes the generated compilation files of your application (`target/` and `build/` folders).

#### Full reset

```bash
just clear

```

Performs the same operations as `just clean`, but also deletes the files related to the simulator compilation.

### Configuration and Maintenance

#### Download the compilation target

```bash
just target

```

Manually downloads the required compilation target for the calculator. (This operation is normally done automatically during your first build).

#### Update local tools

```bash
./update.sh

```

Updates your project's configuration and environment scripts (e.g., `docker.sh`, `update.sh`, `justfile`, etc.).

### Internal and Deprecated Commands

These commands are mainly used by the project's internal scripts or for very specific test cases. **It is generally not necessary to use them manually.**

#### Check the build (deprecated)

```bash
just check

```

This command does exactly the same thing as `just build`. It is recommended to use `just build` instead.

#### Build the simulator only

```bash
just build_simulator

```

This command compiles the application for the simulator without launching it.
It is considered useless for standard use and serves mainly for internal scripts to verify that the application successfully compiles in this environment.
Favor using `just sim` instead.