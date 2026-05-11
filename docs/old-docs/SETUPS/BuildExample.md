# Test Example Compilation Guide

This guide shows you how to compile and export the test/example project for the NumWorks hardware.

## Prerequisites
- [Setup](./Setup.md)

## Compilation

Only compiles, not executable.

```
just build
```

## Exportation

Export the project as a `.nwa` archive (archive format for NumWorks applications).

```
just export
```

The `.nwa` is generated in the `build/` folder. To install it on your calculator, [follow this guide](https://yaya-cout.github.io/Nwagyu/guide/help/how-to-install.html)