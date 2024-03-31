pub mod engine{
    pub mod algorithms {
        pub mod marching_cubes;
        mod tables;
    }
    pub mod types {
        mod data_grid;
        mod hash_grid;
        mod implicit_func;
        mod mesh;
        mod xyz;
    
        pub use data_grid::*;
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
    pub mod window_helper;
}