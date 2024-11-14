use imlet::{
    types::{
        computation::ImplicitModel,
        geometry::{BoundingBox, Sphere, Vec3},
    },
    utils,
};

pub fn main() {
    utils::logging::init_info();

    // Inputs
    let size = 100.0;
    let offset = 5.0;
    let bounds = BoundingBox::new(
        Vec3::new(offset, offset, offset),
        Vec3::new(offset + size, offset + size, offset + size),
    );

    // Function
    let mut model = ImplicitModel::new();

    let _ = model
        .add_function(
            "Sphere",
            Sphere::new(
                Vec3::new(
                    offset + size / 2.0,
                    offset + size / 2.0,
                    offset + size / 2.0,
                ),
                size * 0.45,
            ),
        )
        .unwrap();

    // Generate mesh
    #[cfg(feature = "viewer")]
    {
        imlet::viewer::run_explorer(model, bounds);
    }
    #[cfg(not(feature = "viewer"))]
    {
        let cell_size = 0.5;
        let _ = model
            .generate_iso_surface("Sphere", &bounds, cell_size)
            .unwrap();

        println!("Enable the viewer feature by using (--features viewer) to show the result");
    }
}
