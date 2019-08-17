# Chip8 Emulator

This project is an example of python module 
implemented in Rust programming language.
The core of Chip8 emulator was implemented in
Rust and exported to Python with PyO3 crate.
The rest of the application was created in
pure Python with help of pygame and pillow 
packages.

## How to build extension?

You can just run cargo, that will generate 
python native extension:

```
$ cargo build
```

On MacOS few additional linker arguments are
required, so you may have to build as follows:

```
$ cargo rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup
```

## How to run unit tests?

In order to execute unit tests you have to
disable default features, otherwise you may
encounter wired linking errors:

```
$ cargo test --no-default-features
```

## How to build python package?

Please use *maturin* in order to build python 
wheels for you platform. Just run following 
command in project root directory:

```
$ maturin build
```

This should created a python package in 
*taget/wheels* subdirectory.

## How to run emulator?

Please install chip8 python package and run chip8
binary. You have the pass the path to the ROM file
as first command line argument:

```
$ chip8 <path_to_the_rom_file>
```

## Links

* Rust (https://www.rust-lang.org/)
* PyO3 (https://github.com/PyO3)
* Python (https://python.org)
* Pillow (https://python-pillow.org/)
* PyGame (https://www.pygame.org)
