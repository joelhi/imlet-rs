pub mod engine{
    pub mod algorithms {
        pub mod marching_cubes;
        mod tables;
    }
    pub mod types {
        mod dense_field;
        mod hash_grid;
        mod implicit_func;
        mod mesh;
        mod xyz;
    
        pub use dense_field::*;
        pub use implicit_func::*;
        pub use mesh::*;
        pub use xyz::*;

        use hash_grid::SpatialHashGrid;
    }
    pub mod utils {
        pub mod implicit_functions;
        pub mod io;
    }
}
pub mod viewer{
    pub mod window;
    pub mod material;
    mod vertex;
    mod camera;
    mod camera_controller;
    mod texture;
}

pub mod examples{
    mod gyroid;
    mod compute_shader;
    mod bitmask_gyroid;
    pub use gyroid::run_gyroid;
    pub use compute_shader::run_compute_shader;
    pub use bitmask_gyroid::run_bitmask_gyroid;
}