pub trait ImplicitFunction<T: Float + Debug + Send + Sync>: Sync + Send {
    fn bounds(&self) -> BoundingBox<T>;

    fn closet_point(&self, query: Vec3<T>)->Vec3<T>;
}