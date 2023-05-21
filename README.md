# pbrt4

[![CI](https://github.com/mxpv/pbrt4/actions/workflows/ci.yml/badge.svg)](https://github.com/mxpv/pbrt4/actions/workflows/ci.yml)

A Rust crate to load [pbrt-v4](https://pbrt.org/fileformat-v4) files.


> The scene description files used by pbrt are plain text files. The file format was designed so that it would be both easy to parse and easy for applications to generate from their own internal representations of scenes.
> 
> A pbrt scene file consists of a series of statements; different statements specify the geometry and light sources in the scene and set overall rendering parameters (such as which light transport algorithm to use or the image resolution).


## Resources
- [PBR book](https://pbr-book.org).
- [pbrt-v4](https://github.com/mmp/pbrt-v4) repo.
- The file format [documentation](https://pbrt.org/fileformat-v4).
- [pbrt-v4-scenes](https://github.com/mmp/pbrt-v4-scenes) repo.

## Example

```rust
let data = fs::read_to_string("file.pbrt")?;
let scene = Scene::load(&data)?;

for shape in scene.shapes {
    println!("{:?}", shape)
}
```
