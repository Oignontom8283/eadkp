# Configuration

Once your project is created, it will be preconfigured to work right out of the box.

However, there are several things you need to know in order to customize your project.

## Name

To change the name of your project, it is very simple: you just need to change the `name` field under `[package]` in the `Cargo.toml` file:

```toml {3}
...
[package]
name = "my_apa"
version = "0.1.0"
edition = "2024"
...

```

The rest of the `Cargo.toml` file is preconfigured to work with your project's utilities. It is not recommended to modify it unless you know what you are doing.

## .eadkp/

The `.eadkp/` folder mainly contains script and configuration files.
**You must not modify, delete, or move these files.** The scripts are downloaded locally to work offline.
They are updated by the update script and will be restored in case of deletion or modification.

The `config.env` file basically contains the configuration for the update script of your project's utilities. It is not recommended to change it unless you know what you are doing.

## justfile

The provided `justfile` (an equivalent to a Makefile) only contains the import of the basic commands provided in `.eadkp/`: `export`, `build`, `sim`, etc.

It is here, following the import, that you can add your own custom commands, or even overwrite the basic commands if you wish.