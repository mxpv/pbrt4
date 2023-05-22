# pbrt4

[![CI](https://github.com/mxpv/pbrt4/actions/workflows/ci.yml/badge.svg)](https://github.com/mxpv/pbrt4/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/pbrt4)](https://crates.io/crates/pbrt4)
[![docs.rs](https://img.shields.io/docsrs/pbrt4)](https://docs.rs/pbrt4/latest/pbrt4/)
[![Crates.io](https://img.shields.io/crates/l/pbrt4)](https://github.com/mxpv/pbrt4/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/mxpv/pbrt4/status.svg)](https://deps.rs/repo/github/mxpv/pbrt4)

A Rust crate to load [pbrt-v4](https://pbrt.org/fileformat-v4) files.


> The scene description files used by pbrt are plain text files. The file format was designed so that it would be both easy to parse and easy for applications to generate from their own internal representations of scenes.
> 
> A pbrt scene file consists of a series of statements; different statements specify the geometry and light sources in the scene and set overall rendering parameters (such as which light transport algorithm to use or the image resolution).


## Resources
- [PBR book](https://pbr-book.org).
- [pbrt-v4](https://github.com/mmp/pbrt-v4) repo.
- The file format [documentation](https://pbrt.org/fileformat-v4).
- [pbrt-v4-scenes](https://github.com/mmp/pbrt-v4-scenes) repo.

## Getting started

Add the following to your project's Cargo.toml:

```
pbrt4 = "0.1.0"
```

Reading a pbrt file is as easy as:

```rust
let scene = Scene::from_file("file.pbrt")?;

for shape in scene.shapes {
    println!("{:?}", shape)
}
```

Please refer to [examples](./examples) for more examples how to use the crate.
