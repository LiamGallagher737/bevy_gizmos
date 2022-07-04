//! This contains the [`Gizmo`] type

use bevy::{
    math::{Quat, Vec3},
    prelude::{shape, Assets, Color, Handle, Mesh, ResMut, Transform, Component},
};
use lazy_static::lazy_static;
use std::{
    fmt::{Display, Formatter, Result},
    sync::RwLock,
};

use crate::interactions::GizmoInteractions;

lazy_static! {
    static ref PRIMITIVE_MESH_HANDLES: RwLock<MeshHandles> = RwLock::new(MeshHandles::default());
}

#[derive(Default)]
struct MeshHandles {
    sphere: Handle<Mesh>,
    cube: Handle<Mesh>,
}

pub(crate) fn setup(mut meshes: ResMut<Assets<Mesh>>) {
    if let Ok(mut handles) = PRIMITIVE_MESH_HANDLES.write() {
        handles.sphere = meshes.add(Mesh::from(shape::Icosphere::default()));
        handles.cube = meshes.add(Mesh::from(shape::Cube::default()));
    }
}

pub struct Gizmo {
    pub transform: Transform,
    pub color: Color,
    pub mesh_handle: Handle<Mesh>,
    pub interactions: GizmoInteractions,
}

impl Gizmo {
    pub fn new(translation: Vec3, scale: Vec3, color: Color, mesh_handle: Handle<Mesh>) -> Self {
        Self {
            transform: Transform {
                translation,
                scale,
                ..Default::default()
            },
            color,
            mesh_handle,
            ..Default::default()
        }
    }

    pub fn sphere(translation: Vec3, diameter: f32, color: Color) -> Self {
        Self {
            transform: Transform {
                translation,
                scale: Vec3::new(diameter, diameter, diameter),
                ..Default::default()
            },
            color,
            mesh_handle: PRIMITIVE_MESH_HANDLES.read().unwrap().sphere.clone(),
            ..Default::default()
        }
    }

    pub fn cube(translation: Vec3, size: f32, color: Color) -> Self {
        Self {
            transform: Transform {
                translation,
                scale: Vec3::new(size, size, size),
                ..Default::default()
            },
            color,
            mesh_handle: PRIMITIVE_MESH_HANDLES.read().unwrap().cube.clone(),
            ..Default::default()
        }
    }

    pub fn r#box(translation: Vec3, scale: Vec3, color: Color) -> Self {
        Self {
            transform: Transform {
                translation,
                scale,
                ..Default::default()
            },
            color,
            mesh_handle: PRIMITIVE_MESH_HANDLES.read().unwrap().cube.clone(),
            ..Default::default()
        }
    }

    pub fn with_position(mut self, translation: Vec3) -> Self {
        self.transform.translation = translation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.transform.scale = scale;
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.transform.rotation = rotation;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for Gizmo {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            color: Color::default(),
            mesh_handle: PRIMITIVE_MESH_HANDLES.read().unwrap().sphere.clone(),
            interactions: GizmoInteractions::default(),
        }
    }
}

impl Display for Gizmo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Position: {0} \n Scale: {1}, \n Rotation: {2} \n Color: {3:#?}",
            self.transform.translation, self.transform.scale, self.transform.rotation, self.color
        )
    }
}