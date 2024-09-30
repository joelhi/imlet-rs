use imlet::{
    algorithms::marching_cubes::generate_iso_surface,
    types::{
        computation::{
            functions::{CustomSDF, Gyroid, XCoord, ZCoord},
            operations::{self, shape},
            ImplicitModel, ModelError,
        },
        geometry::{BoundingBox, Mesh, Sphere, Vec3},
    },
    utils::{io::parse_obj_file, logging::init_info, math_helper::Pi},
    viewer::app::run_viewer, //viewer::app::run_viewer,
};
use num_traits::Float;

pub fn main() {
    init_info();

    // Build model
    let cell_size = 0.25;

    let (model, bounds) = match build_model() {
        Ok((model, bounds)) => (model, bounds),
        Err(error) => {
            panic!("{}", error)
        }
    };

    let mut field = model
        .generate_field("VariableOffset", &bounds.offset(5.), cell_size)
        .unwrap();

    field.smooth(0.85, 4);
    let iso_surface = Mesh::from_triangles(&generate_iso_surface(&field, 0.), false);

    run_viewer(&iso_surface.convert::<f32>());
}

fn build_model<T: Pi + Float + Send + Sync + 'static>(
) -> Result<(ImplicitModel<T>, BoundingBox<T>), ModelError> {
    let mut model = ImplicitModel::new();

    let geometry = parse_obj_file("bunny_y.obj", false).unwrap();
    let bounds = geometry.bounds();
    let mesh_tag = model.add_function("Shape", CustomSDF::from_mesh(&geometry))?;
    let gyroid_tag = model.add_function(
        "Gyroid",
        Gyroid::with_equal_spacing(T::from(5.5).unwrap(), false),
    )?;
    let offset_gyroid = model.add_operation_with_inputs(
        "OffsetGyroid",
        shape::Thickness::new(T::from(2.).unwrap()),
        &[&gyroid_tag],
    )?;
    let z_coord = model.add_function("XCoord", XCoord::remapped(bounds.min.x, bounds.max.x))?;
    let max_offfset = model.add_constant("MaxOffset", T::from(-3.0).unwrap())?;
    let multiplication = model.add_operation_with_inputs(
        "OffsetDist",
        operations::math::Multiply::new(),
        &[&z_coord, &max_offfset],
    )?;
    let intersection = model.add_operation_with_inputs(
        "Intersection",
        shape::BooleanIntersection::new(),
        &[&mesh_tag, &offset_gyroid],
    )?;
    let _ = model.add_operation_with_inputs(
        "VariableOffset",
        operations::math::Add::new(),
        &[&intersection, &multiplication],
    )?;

    Ok((model, bounds))
}
