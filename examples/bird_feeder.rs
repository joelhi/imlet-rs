use implicit::{engine::{algorithms::marching_cubes::generate_iso_surface, types::{functions::*, DenseFieldF32, Mesh, Plane, XYZ}, utils::{self, io::write_as_obj}}, viewer::{material::Material, window::run}};


pub fn main() {
    utils::logging::init();

    // Inputs
    let num_pts = 300;
    let size = 100.0;
    let length = 5.0;

    // Build model
    let center = XYZ::new(size / 2.0, size / 2.0, size / 2.0);

    let sphere = Sphere::new(center, 0.4 * size);

    let offset_sphere = Offset::new(sphere, 3.0);

    let gyroid = Gyroid::with_equal_spacing(length);

    let thick_gyroid = Difference::new(Offset::new(gyroid, -0.75), Offset::new(gyroid, 0.75));

    let spheres = Difference::new(sphere, offset_sphere);

    let trimmed_sphere = ClippingPlane {
        function: spheres,
        plane: Plane::new(XYZ::new(0.0, size / 1.8, 0.0), -1.0 * XYZ::y_axis()),
    };

    let bowl = Intersection::new(thick_gyroid, trimmed_sphere);

    let r = 2.5;
    let translation = XYZ::new(0.0, 4.0, 0.0);
    let edge = Torus::new(center + translation,0.4*size - r , r);

    let line1 = Line::new(XYZ::new(size/2.0, size/2.0 + 4.0, r), XYZ::new(size/2.0, size/2.0 + 4.0, size- r), r);

    let line2 = Line::new(XYZ::new(r, size/2.0 + 4.0, size/2.0), XYZ::new(size-r, size/2.0 + 4.0, size/2.0), r);

    let line_union = Union::new(line1, line2);

    let line_union = Difference::new(line_union, Sphere::new(center + translation, size*0.28));

    let union = Union::new(edge, bowl);

    let union = Union::new(union, line_union);

    let inner_ring = Torus::new(center + translation, size*0.28 , r);

    let union = Union::new(union, inner_ring);

    let clean = Max::with_constant(union, 0.1);

    let clipped = ClippingPlane {
        function: clean,
        plane: Plane::new(XYZ::new(0.0, (center + translation).y, 0.0), -1.0 * XYZ::y_axis()),
    };


    // Design space
    let mut grid = DenseFieldF32::new(
        XYZ::origin(),
        size / (num_pts as f32),
        num_pts,
        num_pts,
        num_pts,
    );

    grid.evaluate(&clipped, true);


    // generate mesh
    let triangles = generate_iso_surface(&grid, 0.0);
    let mesh = Mesh::from_triangles(&triangles);

    // Run viewer
    pollster::block_on(run(&mesh, Material::Normal));

}
