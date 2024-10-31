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
    let cell_size = 2.5;
    let bounds = BoundingBox::new(
        Vec3::new(offset, offset, offset),
        Vec3::new(offset + size, offset + size, offset + size),
    );

    // Function
    let mut model = ImplicitModel::new();

    let output = model
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

    println!("{}", model);

    // Generate mesh
    #[cfg(feature = "viewer")]
    {
        // let mesh = model
        //     .generate_iso_surface(&output, &bounds, cell_size)
        //     .unwrap();

        imlet::viewer::run_explorer(model, bounds);
    }
    #[cfg(not(feature = "viewer"))]
    {
        let _ = model
            .generate_iso_surface(&output, &bounds, cell_size)
            .unwrap();

        println!("Enable the viewer feature by using (--features viewer) to show the result");
    }
}
