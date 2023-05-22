use std::env;

use pbrt4::{Result, Scene};

/// Dump reads a pbrt file and dumps everything to stdout.
/// Usage:
/// `❯ cargo run --example dump -- ./assets/disney-cloud/disney-cloud.pbrt`
///
fn main() -> Result<()> {
    let path = env::args().nth(1).expect("Path to pbrt file expected");

    println!("Loading scene: {}", &path);
    let scene = Scene::from_file(&path)?;

    println!("Global options: {:#?}", scene.options);

    if let Some(camera) = scene.camera {
        println!("Camera: {:#?}", camera);
    }

    if let Some(film) = scene.film {
        println!("Film: {:#?}", film);
    }

    if let Some(integrator) = scene.integrator {
        println!("Integrator: {:#?}", integrator);
    }

    if let Some(accelerator) = scene.accelerator {
        println!("Accelerator: {:#?}", accelerator);
    }

    if let Some(sampler) = scene.sampler {
        println!("Sampler: {:#?}", sampler);
    }

    println!("World begin");

    for texture in scene.textures {
        println!("Texture: {:#?}", texture);
    }

    for material in scene.materials {
        println!("Material: {:#?}", material);
    }

    for light in scene.lights {
        println!("Light: {:#?}", light);
    }

    for medium in scene.mediums {
        println!("Medium: {:#?}", medium);
    }

    for shape in scene.shapes {
        println!("Shape: {:#?}", shape);
    }

    println!("Done");
    Ok(())
}
