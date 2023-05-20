//! High level scene description.

use std::str::FromStr;

/// The coordinate system.
#[derive(Debug, Default, Eq, PartialEq)]
pub enum CoordinateSystem {
    /// Translate the scene so that the camera is at the origin.
    #[default]
    CameraWorld,
    /// Use camera space.
    Camera,
    /// Uses world space.
    World,
}

impl FromStr for CoordinateSystem {
    type Err = (); // TODO: Use error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cameraworld" => Ok(CoordinateSystem::CameraWorld),
            "camera" => Ok(CoordinateSystem::Camera),
            "world" => Ok(CoordinateSystem::World),
            _ => Err(()),
        }
    }
}

/// Scene-wide rendering options.
#[derive(Debug)]
pub struct Options {
    /// Forces all pixel samples to be through the center of the pixel area.
    pub disable_pixel_jitter: bool,
    /// Forces point sampling at the finest MIP level for all texture lookups.
    pub disable_texture_filtering: bool,
    /// Forces all samples within each pixel to sample the same wavelengths.
    pub disable_wavelength_jitter: bool,
    /// Global scale factor applied to triangle edge lengths before evaluating
    /// the edge length test for refinement when applying displacement mapping.
    pub displacement_edge_scale: f32,
    /// Specifies the filename of an image to use when computing mean squared
    /// error versus the number of pixel samples taken
    pub mse_reference_image: Option<String>,
    /// Filename for per-sample mean squared error results.
    pub mse_reference_out: Option<String>,
    /// Specifies the coordinate system to use for rendering computation.
    pub render_coord_sys: CoordinateSystem,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            disable_pixel_jitter: false,
            disable_texture_filtering: false,
            disable_wavelength_jitter: false,
            displacement_edge_scale: 1.0,
            mse_reference_image: None,
            mse_reference_out: None,
            render_coord_sys: CoordinateSystem::CameraWorld,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_coord_sys() {
        assert_eq!(
            CoordinateSystem::from_str("cameraworld"),
            Ok(CoordinateSystem::CameraWorld)
        );
        assert_eq!(
            CoordinateSystem::from_str("camera"),
            Ok(CoordinateSystem::Camera)
        );
        assert_eq!(
            CoordinateSystem::from_str("world"),
            Ok(CoordinateSystem::World)
        );

        assert!(CoordinateSystem::from_str("").is_err());
        assert!(CoordinateSystem::from_str("foo").is_err());
    }
}
