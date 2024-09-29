use imlet::{
    types::{
        computation::{functions::CustomSDF, operations::shape, ImplicitModel, ModelError},
        geometry::Mesh,
    },
    utils::{
        io::{parse_obj_file, write_obj_file},
        logging::init_info,
    },
    //viewer::app::run_viewer,
};
use num_traits::Float;

pub fn main() {
    init_info();

    let mesh: imlet::types::geometry::Mesh<f32> =
        parse_obj_file("assets/geometry/sphere.obj", false).unwrap();

    let bounds = mesh.bounds();
    let (x_size, y_size, z_size) = bounds.dimensions();
    // Build model
    let cell_size = x_size.max(y_size).max(z_size) / 25.0;

    let model = match build_model(&mesh) {
        Ok(model) => model,
        Err(error) => {
            panic!("{}", error)
        }
    };

    let iso_surface = model
        .generate_iso_surface("Mesh", &bounds, cell_size)
        .unwrap();

    write_obj_file(&iso_surface, "sphere_sdf").unwrap();

    //run_viewer(&iso_surface.convert::<f32>());
}

fn build_model<T: Float + Send + Sync + 'static>(
    mesh: &Mesh<T>,
) -> Result<ImplicitModel<T>, ModelError> {
    let mut model = ImplicitModel::new();

    let mesh_tag = model.add_function("Mesh", CustomSDF::from_mesh(&mesh))?;
    let _ = model.add_operation_with_inputs(
        "Offset",
        shape::Offset::new(T::from(0.1).unwrap()),
        &[&mesh_tag],
    )?;

    Ok(model)
}
