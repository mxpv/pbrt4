use pbrt4::{
    param::Spectrum,
    types::{Camera, Light, Shape},
    Scene,
};

fn assert_eq_f32(a: f32, b: f32) {
    assert!(a - b <= f32::EPSILON, "{} != {}", a, b);
}

fn assert_eq_f32_arr<const N: usize>(a: [f32; N], b: [f32; N]) {
    for i in 0..N {
        assert_eq_f32(a[i], b[i])
    }
}

#[test]
fn disney_cloud() {
    let scene = Scene::from_file("assets/disney-cloud/disney-cloud.pbrt").unwrap();

    let camera = scene.camera.unwrap();
    let Camera::Perspective { fov, .. } = camera.params else {
        panic!("Unexpected camera type");
    };

    assert_eq_f32(fov, 31.07);

    assert_eq!(scene.lights.len(), 2);

    // Infinite light
    {
        let infinite = &scene.lights[0];

        let Light::Infinite { spectrum , ..} = infinite else {
            panic!("Unexpected light type at 0, want Infinite");
        };

        let Spectrum::Rgb(rgb) = spectrum.unwrap() else {
            panic!("Unexpected spectrum value type");
        };

        assert_eq_f32_arr(rgb, [0.03, 0.07, 0.23]);
    }

    // Distant light
    {
        let distant = &scene.lights[1];
        assert!(matches!(distant, Light::Distant));
    }

    assert_eq!(scene.materials.len(), 2);

    assert_eq!(scene.shapes.len(), 2);

    // Disk shape
    {
        let disk = &scene.shapes[0];
        assert!(matches!(disk.params, Shape::Disk { .. }));
        assert_eq!(disk.material_index, Some(0));
    }

    // Sphere
    {
        let sphere = &scene.shapes[1];
        let Shape::Sphere { alpha, radius, zmin, zmax, phimax } = sphere.params else {
            panic!("Unexpected shape at 1, want Sphere");
        };

        assert_eq!(sphere.material_index, Some(1));

        assert_eq_f32(radius, 1.442_249_5);

        // Default parameters.
        assert_eq_f32(zmin, -1.442_249_5);
        assert_eq_f32(zmax, 1.442_249_5);
        assert_eq_f32(phimax, 360.0);
        assert_eq_f32(alpha, 1.0);
    }
}
