use clap::{Arg, ArgMatches, Command};
use imlet::utils::io;

fn main() {
    imlet::utils::logging::init_info();
    let matches = Command::new("Imlet CLI")
        .version(imlet::IMLET_VERSION)
        .author("Joel Hilmersson <d.j.hilmersson@gmail.com>")
        .about("A CLI for running and visualizing implicit models")
        .subcommand(
            Command::new("show-obj")
                .about("Show an OBJ file")
                .arg(Arg::new("file").help("Path to the OBJ file").required(true)),
        )
        .subcommand(
            Command::new("run-model")
                .about("Run a serialized model")
                .arg(
                    Arg::new("file")
                        .help("Path to the serialized model file")
                        .required(true),
                )
                .arg(Arg::new("output").help("Target node").required(true))
                .arg(
                    Arg::new("cell_size")
                        .help("Resolution value")
                        .required(true),
                )
                .arg(
                    Arg::new("save")
                        .long("save")
                        .help("Save the output to a file")
                        .required(false), // Indicates this flag expects a value
                )
                .arg(
                    Arg::new("show")
                        .long("show")
                        .help("Display the result")
                        .action(clap::ArgAction::SetTrue), // Indicates this flag toggles a boolean
                ),
        )
        .get_matches();

    // Handle the commands and arguments
    match matches.subcommand() {
        Some(("show-obj", sub_matches)) => handle_show_obj(sub_matches),
        Some(("run-model", sub_matches)) => handle_run_model(sub_matches),
        _ => eprintln!("Use --help to see usage."),
    }
}

fn handle_show_obj(matches: &ArgMatches) {
    let file_path = matches
        .get_one::<String>("file")
        .expect("File argument is required");
    let mut mesh = imlet::utils::io::parse_obj_file::<f32>(file_path, false, false).unwrap();
    mesh.compute_vertex_normals_par();

    #[cfg(feature = "viewer")]
    {
        imlet::viewer::show_mesh(&mesh, Some(mesh.bounds()));
    }
    #[cfg(not(feature = "viewer"))]
    {
        log::error!("Enable the viewer feature to see the model.");
    }
}

fn handle_run_model(matches: &ArgMatches) {
    let file_path = matches
        .get_one::<String>("file")
        .expect("File argument is required");
    let output = matches
        .get_one::<String>("output")
        .expect("Target argument is required");
    let cell_size: f32 = matches
        .get_one::<String>("cell_size")
        .expect("Resolution argument is required")
        .parse()
        .expect("Resolution must be a valid number");

    let model = io::read_model_from_file::<f32>(file_path).unwrap();

    let mut mesh = model.generate_iso_surface(&output, cell_size).unwrap();
    mesh.compute_vertex_normals_par();

    if let Some(save_path) = matches.get_one::<String>("save") {
        io::write_obj_file(&mesh, &save_path).unwrap();
    }

    if matches.get_flag("show") {
        #[cfg(feature = "viewer")]
        {
            imlet::viewer::show_mesh(&mesh, model.config().map(|c| c.bounds));
        }
        #[cfg(not(feature = "viewer"))]
        {
            log::error!("Enable the viewer feature to see the model.");
        }
    }
}
