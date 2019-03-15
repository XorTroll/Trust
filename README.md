# Trust

An unstable (but working) attempt to compile Rust to Nintendo Switch homebrew from 64-bit Windows 10.

## Rust + homebrew

All plain Rust, except using a [custom std fork (libnx-rs-std)](https://github.com/ischeinkman/libnx-rs-std) to make it work fine.

## Setup

NOTE: This guide assumes that devkitPro is installed into `C:\devkitPro`, if not, change all the paths from the project's `.cargo/config` and from this guide to the actual devkitPro location.

- Download Rust (https://www.rust-lang.org/tools/install).

- Download MinGW64 (https://sourceforge.net/projects/mingw-w64/) and locate where it gets installed.

- Locate the C compiler (gcc.exe) there (example: `C:\Program Files\MinGW64\bin\gcc.exe`).

- Create a system environment variable named `CC` with the GCC executable's (full!) path as value.

- Now, make sure you have:

  - `C:\Users\<user>\.cargo\bin` added to PATH. (in order to use cargo, xargo, rustup, ...)
  - `C:\devkitPro\devkitA64\bin` added to PATH. (in order to let the compiler access devkitA64 compilers)
  - `C:\devkitPro\tools\bin` added to PATH. (in order to generate NSO/NRO from the built ELF)
  - The GCC path as CC in system environment variables. (in order to specify the compiler for several crates)

- Install Windows GNU toolchain for Rust, suggested a nightly from 2019-01-19 (`nightly-2019-01-19-x86_64-pc-windows-gnu`), doing this:

```sh
rustup install toolchain nightly-2019-01-19-x86_64-pc-windows-gnu
rustup default nightly-2019-01-19-x86_64-pc-windows-gnu
```

- Install xargo, the cargo wrapper we need for cross-compiling: `cargo install xargo`

- Install rust-src components: `rustup component add rust-src`

- Clone or download this testing project and execute `make`.

- In case errors occur, ensure everything is installed and that PATH environment variable has the required paths.

## The project

The project has a Makefile and a Cargo.toml file to customize.

It's suggested that the Makefile's `NAME` element and the project's name in the TOML data are the same.

As the Makefile is a slightly rewritten version of classical one from libnx, RomFs, custom icons and other stuff is also supported.

Output ELFs/NROs/NSOs are generated on `target\aarch64-none-elf\debug\<name>.<elf/nro/nso>`

As required by rustc, source needs to be located at `src`, being the entrypoint file `main.rs`.

## Credits

For the help given with Rust, thanks to [roblabla](https://github.com/roblabla), [ischeinkman](https://github.com/ischeinkman) and the MegatonHammer Rust community.