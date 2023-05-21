//! Scene loader

use std::collections::HashMap;

use glam::{Mat4, Vec3};

use crate::{
    param::ParamList,
    types::{
        Accelerator, Camera, Film, Integrator, Light, Material, Medium, Options, Sampler, Shape,
        Texture,
    },
    Element, Error, Parser, Result,
};

/// A number of directives modify the current graphics state.
/// Examples include the transformation directives (Transformations),
/// and the directive that sets the current material.
#[derive(Default, Clone)]
pub struct State<'a> {
    /// The reverse-orientation setting, specified by the `ReverseOrientation`
    /// directive, is part of the graphics state.
    pub reverse_orientation: bool,

    pub transform_matrix: Mat4,

    pub current_inside_medium: Option<&'a str>,
    pub current_outside_medium: Option<&'a str>,
    pub material_index: Option<usize>,

    pub shape_params: ParamList<'a>,
    pub light_params: ParamList<'a>,
    pub material_params: ParamList<'a>,
    pub medium_params: ParamList<'a>,
    pub texture_params: ParamList<'a>,
}

pub struct CameraEntity {
    pub params: Camera,
    pub transform: Mat4,
}

pub struct ShapeEntity {
    pub params: Shape,
    pub reverse_orientation: bool,
    pub material_index: Option<usize>,
}

#[derive(Default)]
pub struct Scene {
    /// Global options.
    pub options: Options,
    /// Camera.
    pub camera: Option<CameraEntity>,
    pub film: Option<Film>,
    pub integrator: Option<Integrator>,
    pub accelerator: Option<Accelerator>,
    pub sampler: Option<Sampler>,
    pub textures: Vec<Texture>,
    pub materials: Vec<Material>,
    pub lights: Vec<Light>,
    pub mediums: Vec<Medium>,
    pub shapes: Vec<ShapeEntity>,
}

impl Scene {
    pub fn load(data: &str) -> Result<Scene> {
        let mut scene = Scene::default();
        let mut parser = Parser::new(data);

        let mut current_state = State::default();
        let mut states_stack = Vec::new();
        let mut is_world_block = false;

        let mut named_coord_systems: HashMap<String, Mat4> = HashMap::default();

        // Texture name to index.
        let mut named_textures: HashMap<String, usize> = HashMap::default();
        let mut named_materials: HashMap<String, usize> = HashMap::default();
        let mut named_mediums: HashMap<String, usize> = HashMap::default();

        loop {
            let element = match parser.parse_next() {
                Ok(element) => element,
                Err(err) if err == Error::EndOfFile => {
                    // TODO: validate.

                    debug_assert!(states_stack.is_empty());
                    debug_assert!(is_world_block);

                    break;
                }
                Err(err) => return Err(err),
            };

            match element {
                Element::AttributeBegin => {
                    states_stack.push(current_state.clone());
                }
                Element::AttributeEnd => match states_stack.pop() {
                    Some(state) => current_state = state,
                    None => return Err(Error::TooManyEndAttributes),
                },
                Element::Attribute { target, params } => match target {
                    "shape" => current_state.shape_params.extend(&params),
                    "light" => current_state.light_params.extend(&params),
                    "material" => current_state.material_params.extend(&params),
                    "medium" => current_state.medium_params.extend(&params),
                    "texture" => current_state.texture_params.extend(&params),
                    _ => unimplemented!(),
                },
                Element::ReverseOrientation => {
                    current_state.reverse_orientation = !current_state.reverse_orientation;
                }
                Element::Translate { v } => {
                    current_state.transform_matrix *= Mat4::from_translation(Vec3::from(v))
                }
                Element::Identity => {
                    current_state.transform_matrix = Mat4::IDENTITY;
                }
                // Transform resets the CTM to the specified matrix.
                Element::Transform { m } => {
                    current_state.transform_matrix = Mat4::from_cols_array(&m);
                }
                // An arbitrary transformation to multiply the CTM with can be specified using ConcatTransform
                Element::ConcatTransform { m } => {
                    current_state.transform_matrix *= Mat4::from_cols_array(&m);
                }
                Element::Scale { v } => {
                    current_state.transform_matrix *= Mat4::from_scale(Vec3::from(v));
                }
                Element::Rotate { angle, v } => {
                    current_state.transform_matrix *= Mat4::from_axis_angle(Vec3::from(v), angle);
                }
                Element::LookAt { eye, look_at, up } => {
                    current_state.transform_matrix *=
                        Mat4::look_at_lh(Vec3::from(eye), Vec3::from(look_at), Vec3::from(up));
                }
                // A name can be associated with the CTM using the CoordinateSystem directive.
                Element::CoordinateSystem { name } => {
                    named_coord_systems.insert(name.to_string(), current_state.transform_matrix);
                }
                // The CTM can later be reset to the recorded transformation using CoordSysTransform.
                Element::CoordSysTransform { name } => {
                    match named_coord_systems.get(name).copied() {
                        Some(mat) => current_state.transform_matrix = mat,
                        None => {
                            // TODO: Material not found, return error.
                            unimplemented!()
                        }
                    }
                }
                // The Camera directive specifies the camera used for viewing the scene.
                Element::Camera { ty, params } => {
                    let camera_from_world = current_state.transform_matrix;
                    // TODO: Support transformStartTime and transformEndTime
                    let world_from_camera = camera_from_world.inverse();

                    // pbrt automatically records the camera transformation matrix in the "camera" named coordinate system.
                    // This can be useful for placing light sources with respect to the camera, for example.

                    // TODO: Fix key
                    named_coord_systems.insert("camera".to_string(), world_from_camera);

                    let camera = Camera::new(ty, params)?;

                    let entity = CameraEntity {
                        params: camera,
                        transform: world_from_camera,
                    };

                    scene.camera = Some(entity);
                }
                Element::Film { ty, params } => {
                    debug_assert!(scene.film.is_none());
                    let film = Film::new(ty, params)?;
                    scene.film = Some(film);
                }
                Element::Integrator { ty, params } => {
                    debug_assert!(scene.integrator.is_none());
                    let integrator = Integrator::new(ty, params)?;
                    scene.integrator = Some(integrator);
                }
                Element::Accelerator { ty, params } => {
                    debug_assert!(scene.accelerator.is_none());
                    let accelerator = Accelerator::new(ty, params)?;
                    scene.accelerator = Some(accelerator);
                }
                Element::PixelFilter { .. } => {
                    todo!("Implement pixel filter");
                }
                Element::ColorSpace { .. } => {
                    todo!("Support color space");
                }
                Element::Sampler { ty, params } => {
                    let sampler = Sampler::new(ty, params)?;

                    debug_assert!(scene.sampler.is_none());
                    scene.sampler = Some(sampler);
                }
                Element::TransformTimes { .. } | Element::ActiveTransform { .. } => {
                    todo!("Support animated transformations")
                }
                Element::Include(..) | Element::Import(..) => {
                    todo!("Support includes/imports")
                }
                Element::WorldBegin => {
                    is_world_block = true;
                    current_state.transform_matrix = Mat4::IDENTITY;
                }
                Element::Option(param) => {
                    scene.options.apply(param)?;
                }
                Element::Texture {
                    name,
                    ty,
                    class,
                    mut params,
                } => {
                    params.extend(&current_state.texture_params);
                    let texture = Texture::new(name, ty, class, params)?;

                    let index = scene.textures.len();
                    scene.textures.push(texture);

                    named_textures.insert(name.to_string(), index);
                }
                // The Material directive specifies the current material, which then applies for all subsequent
                // shape definitions (until the end of the current attribute scope or until a new material is defined.
                Element::Material { ty, mut params } => {
                    params.extend(&current_state.material_params);
                    let material = Material::new(ty, params, &named_textures)?;

                    let index = scene.materials.len();
                    scene.materials.push(material);

                    current_state.material_index = Some(index);
                }
                Element::MakeNamedMaterial { name, mut params } => {
                    params.extend(&current_state.material_params);
                    let material = Material::new(name, params, &named_textures)?;

                    let index = scene.materials.len();
                    scene.materials.push(material);

                    named_materials.insert(name.to_string(), index);
                }
                Element::NamedMaterial { name } => {
                    // TODO: handle material not found case.
                    current_state.material_index = named_materials.get(name).copied();
                }
                Element::LightSource { ty, params } => {
                    // When a light source is created, the current exterior medium is used for rays leaving the light
                    // when bidirectional light transport algorithms are used.
                    //
                    // The user is responsible for specifying media in a way such that rays reaching lights are in the same medium
                    // as rays leaving those lights.

                    // TODO: Handle current_outside_medium

                    let light = Light::new(ty, params)?;
                    scene.lights.push(light);
                }
                // Area lights have geometry associated with them.
                //
                // After an AreaLightSource directive, all subsequent shapes emit light
                // from their surfaces according to the distribution defined by the given
                // area light implementation.
                Element::AreaLightSource { .. } => {
                    unimplemented!()
                }
                Element::Shape {
                    name: ty,
                    mut params,
                } => {
                    params.extend(&current_state.shape_params);
                    let shape = Shape::new(ty, params)?;

                    // When a shape is created, the current interior medium is assumed to be the medium inside the shape,
                    // and the current exterior medium is assumed to be the medium outside the shape.
                    // TODO: handle mediums

                    let entity = ShapeEntity {
                        params: shape,
                        reverse_orientation: current_state.reverse_orientation,
                        material_index: current_state.material_index,
                    };

                    scene.shapes.push(entity);
                }
                Element::ObjectBegin { .. }
                | Element::ObjectEnd
                | Element::ObjectInstance { .. } => {
                    todo!("Support object instancing");
                }
                // MakeNamedMedium associates a user-specified name with medium scattering characteristics.
                Element::MakeNamedMedium { name, mut params } => {
                    params.extend(&current_state.medium_params);
                    let medium = Medium::new(params)?;

                    let index = scene.mediums.len();
                    scene.mediums.push(medium);

                    named_mediums.insert(name.to_string(), index);
                }
                // MediumInterface directive can be used to specify the current "interior" and "exterior" media.
                // A vacuum—no participating media—is represented by empty string "".
                Element::MediumInterface { interior, exterior } => {
                    current_state.current_inside_medium = Some(interior);
                    current_state.current_outside_medium = Some(exterior);
                }
            }
        }

        Ok(scene)
    }
}
