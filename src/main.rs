use std::env;

use imlet::utils::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        for s in args.iter() {
            println!("{}", s);
        }
        panic!("Expected to find two arguments. A model to run and a cell size.");
    }

    let model = io::read_model_from_file::<f32>(&args[1]).unwrap();
    let output = &args[2];
    let cell_size = args[3]
        .parse::<f32>()
        .expect("Second argument should be a valid f64 value.");

    let mut mesh = model.generate_iso_surface(output, cell_size).unwrap();
    mesh.compute_vertex_normals_par();
    #[cfg(feature = "viewer")]
    {
        imlet::viewer::show_mesh_with_settings(
            &mesh,
            model.config().map(|c| c.bounds),
            &imlet::viewer::DisplaySettings::new(),
        );
    }
}
