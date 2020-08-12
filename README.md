# Yavanna
* Project with **Rust**, **GTK+** and **Iced**.

## State
Might probably build a windows version (the shutdown version if for linux :)

## Origin of the name
"The Sleep of Yavanna refers to the period in Middle-earth's history between the Spring of Arda and the rising of Moon and Sun, during which almost all living things slept and Middle-earth was lit only by starlight." [Source](lotr.fandom.com) 

## Purpose 
I was tired to fall asleep watching Netflix and never cut my portable, so here is a little timer that shutdown the system after a given time or at a specified hour. 

## How to run it
TODO: Make a release

Or you can use Cargo with `cargo run` to compile and run it or you can just compile it with `make build`.

## Requirements 

You need to have on your system in order to run it:
* `GTK+` (which is usually installed on most of Linux distros)
* or nothing if you use `iced` front

If you want to compile it :
* `Makefile`, `Rust` and `Cargo`

## Compilation

You need to specify a feature for use a specific front (only include those dependencies)

* To build with iced front: `make build-iced-release`
* To build with gtk front (linux only): `make build-gtk-release`
