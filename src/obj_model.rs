pub struct ObjModel {
    triangles: Vec<Box<dyn Hittable>>,
    minimum: Vec3,
    maximum: Vec3,
}

impl ObjModel {
    pub fn new_from_path(path: &Path) -> Self {
        todo!()
    }
}
