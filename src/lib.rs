pub mod engine{
    pub mod algorithms {
        pub mod marching_cubes;
        mod tables;
    }
    pub mod types {
        mod dense_field;
        mod hash_grid;
        mod mesh;
        mod xyz;
        mod plane;
        pub mod computation;
    
        pub use dense_field::*;
        pub use mesh::*;
        pub use xyz::*;
        pub use plane::*;

        use hash_grid::SpatialHashGrid;
    }
    pub mod utils {
        pub mod io;
        pub mod logging;
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