pub mod engine{
    pub mod algorithms {
        pub mod marching_cubes;
        mod tables;
    }
    pub mod types {
        pub mod geometry;
        pub mod computation;
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