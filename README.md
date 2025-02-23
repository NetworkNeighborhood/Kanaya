# Kanaya

> [!WARNING]  
> **Work in progress.**
>
> At the moment, barely any function is implemented.
>
> Kanaya is a codename. A final name for the project has not been decided yet.

Kanaya is a GUI editor for Microsoft Windows visual styles. It is inspired by [msstyleEditor](//github.com/nptr/msstyleEditor) and [Windows Style Builder](//www.vistastylebuilder.com/).

## Building

Visual Studio is a requirement to build. This is because, in addition to the Rust source code of Kanaya, you also need to build the Restyle library, which is a Visual Studio project.

Git is also required to be on PATH in order to build to the Restyle library, because we use it for applying the patches.

Besides the Rust compiler and Cargo, you'll need to install Clang libraries. The command that I used for my setup is below. [See the `bindgen` documentation for more information.](//rust-lang.github.io/rust-bindgen/requirements.html)

```cmd
winget install LLVM.LLVM
```

## Restyle integration

Restyle integration is implemented in the [`restyle`](/restyle/) folder. Kanaya relies on a slightly-modified version of Restyle, and as such we have a set of additional files and source patches to mix in with the upstream source code.

This modified version of Restyle allows us to add own helper functions and call them directly from Rust code, rather than spawning a foreign process. Kanaya will communicate with the Restyle library a lot, so constanting spawning processes isn't desirable.

The build system is implemented in [`main_gui/build/build_restyle.rs`](/main_gui/build/build_restyle.rs). This file manages the patching and MSBuild (Visual Studio) build processes, which run before any of the main GUI program's Rust code is built.