pub mod algorithms;
pub mod types{
    mod grid;
    mod hash_grid;
    mod xyz;
    mod implicit_func;
    mod mesh;

    pub use grid::*;
    pub use xyz::*;
    pub use implicit_func::*;
    pub use mesh::*;
    use hash_grid::SpatialHashGrid;

}
pub mod utils;