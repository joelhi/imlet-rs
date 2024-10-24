use imlet::{
    types::computation::{
        functions::{CustomGeometry, Gyroid},
        operations::shape::{BooleanIntersection, Thickness},
        ImplicitModel,
    },
    utils::{self, io::parse_obj_file},
};

pub fn main() {
    utils::logging::init_info();

    let mesh = parse_obj_file("assets/geometry/bunny.obj", false).unwrap();

    let cell_size = 0.5;
    let model_space = mesh.bounds().offset(cell_size);

    // Build model
    let mut model = ImplicitModel::new();

    let mesh_tag = model
        .add_function("Mesh", CustomGeometry::from_mesh(&mesh))
        .unwrap();

    let gyroid_tag = model
        .add_function("Gyroid", Gyroid::with_equal_spacing(5., false))
        .unwrap();

    let offset_gyroid = model
        .add_operation_with_inputs("OffsetGyroid", Thickness::new(2.), &[&gyroid_tag])
        .unwrap();

    let output = model
        .add_operation_with_inputs(
            "Output",
            BooleanIntersection::new(),
            &[&mesh_tag, &offset_gyroid],
        )
        .unwrap();

    #[cfg(feature = "viewer")]
    {
        imlet::viewer::run_explorer(model, &model_space);
    }
    #[cfg(not(feature = "viewer"))]
    {
        let _ = model
            .generate_iso_surface(&output, &model_space, cell_size)
            .unwrap();

        println!("Enable the viewer feature by using (--features viewer) to show the result");
    }
}
