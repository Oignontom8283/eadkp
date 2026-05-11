# Simulator Usage Guide

This guide explains how to use the official NumWorks simulator to test the execution of the test example.

## Prerequisites
- [Setup](./Setup.md)

## Launching the simulator

> [!IMPORTANT]
> You must use your OS terminal (not an IDE terminal)

Make sure you are inside the Docker container.

```bash
chmod +x ./shell.sh
```

```bash
./shell.sh
```

Then start the simulator with the following command:

```bash
just sim
```

> [!NOTE]
> On first use, the script will download the official NumWorks simulator and compile it. This can take a long time, so please be patient.
> 
> Add a number of CPU cores to the build process to speed it up :
> ```bash
> just sim 8 # replace 8 with the number of CPU cores you want to use
> ```

A window should open with the NumWorks calculator simulator. You can now test your application in the simulator.
