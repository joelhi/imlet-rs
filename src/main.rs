/// Run a new window of the explorer.
#[cfg(feature = "viewer")]
pub fn main() {
    imlet::viewer::run_explorer::<f64>();
}
