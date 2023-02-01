use std::{path::Path, sync::Arc};

use crate::{
    bvh::{Aabb, BvhNode},
    geometry::{HitRecord, Hittable, Triangle},
    material::{DielectricMaterial, LambertianMaterial, Material, MetalMaterial},
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Clone)]
pub struct ObjModel {
    triangles: BvhNode,
    minimum: Vec3,
    maximum: Vec3,
}

impl ObjModel {
    pub fn new_from_path(path: &Path) -> Self {
        let load_options = tobj::LoadOptions {
            single_index: false,
            ignore_lines: true,
            ignore_points: true,
            triangulate: true,
        };
        let (models, materials) = tobj::load_obj(path, &load_options).unwrap();

        let materials_mapped: Vec<Arc<dyn Material>> = materials
            .unwrap()
            .iter()
            .map(|m| {
                let material: Arc<dyn Material> = match m.illumination_model {
                    Some(7) => Arc::new(DielectricMaterial::new(m.optical_density.into())),
                    Some(5) => Arc::new(MetalMaterial::new_from_color(
                        Color::new(
                            m.diffuse[0].into(),
                            m.diffuse[1].into(),
                            m.diffuse[2].into(),
                        ),
                        1.0 / <f32 as std::convert::Into<f64>>::into(m.shininess),
                    )),
                    _ => Arc::new(LambertianMaterial::new_from_color(Color::new(
                        m.diffuse[0].into(),
                        m.diffuse[1].into(),
                        m.diffuse[2].into(),
                    ))),
                };
                material
            })
            .collect();

        let mut world: Vec<Box<dyn Hittable>> = vec![];
        for model in models {
            let mesh = model.mesh;

            for triangle_index in 0..(mesh.indices.len() / 3) {
                let (vertex_index0, vertex_index1, vertex_index2) = (
                    mesh.indices[triangle_index * 3] as usize,
                    mesh.indices[triangle_index * 3 + 1] as usize,
                    mesh.indices[triangle_index * 3 + 2] as usize,
                );
                let (vertex0, vertex1, vertex2) = (
                    Vec3::new(
                        mesh.positions[vertex_index0 * 3].into(),
                        mesh.positions[vertex_index0 * 3 + 1].into(),
                        mesh.positions[vertex_index0 * 3 + 2].into(),
                    ),
                    Vec3::new(
                        mesh.positions[vertex_index1 * 3].into(),
                        mesh.positions[vertex_index1 * 3 + 1].into(),
                        mesh.positions[vertex_index1 * 3 + 2].into(),
                    ),
                    Vec3::new(
                        mesh.positions[vertex_index2 * 3].into(),
                        mesh.positions[vertex_index2 * 3 + 1].into(),
                        mesh.positions[vertex_index2 * 3 + 2].into(),
                    ),
                );

                let material = match mesh.material_id {
                    Some(i) => materials_mapped[i].clone(),
                    None => Arc::new(LambertianMaterial::new_from_color(Color::new(
                        0.2, 0.7, 0.2,
                    ))),
                };

                let triangle = if mesh.normals.is_empty() {
                    Triangle::new_without_normal(vertex0, vertex1, vertex2, material)
                } else {
                    let normal = Vec3::new(
                        mesh.normals[vertex_index0 * 3].into(),
                        mesh.normals[vertex_index0 * 3 + 1].into(),
                        mesh.normals[vertex_index0 * 3 + 2].into(),
                    );
                    Triangle::new(vertex0, vertex1, vertex2, normal, material)
                };

                world.push(Box::new(triangle));
            }
        }

        let bb = world.bounding_box();
        let minimum = bb.minimum;
        let maximum = bb.maximum;

        Self {
            triangles: BvhNode::new(world),
            minimum,
            maximum,
        }
    }
}

impl Hittable for ObjModel {
    fn bounding_box(&self) -> Aabb {
        Aabb::new(self.minimum, self.maximum)
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.triangles.hit(ray, t_min, t_max)
    }
}
