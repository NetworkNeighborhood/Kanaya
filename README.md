# Kanaya

> [!WARNING]  
> **Work in progress.**
>
> At the moment, barely any function is implemented.
>
> Kanaya is a codename. A final name for the project has not been decided yet.

Kanaya is a GUI editor for Microsoft Windows visual styles. It is inspired by [msstyleEditor](//github.com/nptr/msstyleEditor) and [Windows Style Builder](//www.vistastylebuilder.com/).

## Building

Besides the Rust compiler and Cargo, you'll need to install Clang libraries. The command that I used for my setup is below. [See the `bindgen` documentation for more information.](//rust-lang.github.io/rust-bindgen/requirements.html)

```cmd
winget install LLVM.LLVM
```

## restyle integration

At this moment, restyle integration is not implemented.