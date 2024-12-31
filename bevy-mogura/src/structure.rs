use crate::MoguraPlugins;
// use bevy::prelude::*;
// use bevy::render::mesh::PrimitiveTopology;
// use bevy::render::render_asset::RenderAssetUsages;
use mogura_io::prelude::*;
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{MeshVertexBufferLayoutRef, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

#[derive(Component)]
pub struct StructureParams {
    pub drawing_method: DrawingMethod,
    pub structure_data: Option<Box<dyn StructureData>>,
}

impl Default for StructureParams {
    fn default() -> Self {
        Self {
            drawing_method: DrawingMethod::VDW,
            structure_data: None,
        }
    }
}

#[derive(Debug, Clone)]
enum DrawingMethod {
    Line,
    VDW,
    Licorise,
    CPK,
    Cartoon,
    NewCartoon,
    Bonds,
}

pub fn setup_structure(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    mut mogura_plugins: Res<MoguraPlugins>,
) {
    if let Some(structure_file) = &mogura_plugins.input_structure {
        // todo: use ..default
        // system scheduling ?
        let structure_data = structure_loader(&structure_file);
        let atoms = structure_data.atoms().clone();
        let bonds = structure_data.bonds().clone();
        // let drawing_method = DrawingMethod::VDW;
        // let drawing_method = DrawingMethod::Licorise;
        let drawing_method = DrawingMethod::Line;

        commands
            .spawn((
                StructureParams {
                    structure_data: Some(Box::new(structure_data)),
                    drawing_method: drawing_method.clone(),
                },
                GlobalTransform::default(),
                Transform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
            ))
            .with_children(|parent| match drawing_method {
                DrawingMethod::VDW => {
                    for atom in &atoms {
                        parent.spawn(PbrBundle {
                            mesh: Mesh3d(meshes.add(Sphere::default())),
                            transform: Transform::from_translation(atom.xyz().into()),
                            material: MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
                            ..default()
                        });
                    }
                },
                DrawingMethod::Licorise => {
                    for atom in &atoms {
                        parent.spawn(PbrBundle {
                            mesh: Mesh3d(meshes.add(Sphere::default())),
                            transform: Transform::from_translation(atom.xyz().into()),
                            material: MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
                            ..default()
                        });
                    }
                    for bond in &bonds {
                        let i = bond.0;
                        let j = bond.1;
                        let start = Vec3::new(atoms[i].x(), atoms[i].y(), atoms[i].z());
                        let end = Vec3::new(atoms[j].x(), atoms[j].y(), atoms[j].z());
                        let center = (start + end) / 2.;
                        let direction = end - start;
                        let length = direction.length();
                        let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
                        parent.spawn(PbrBundle {
                            mesh: Mesh3d(meshes.add(Cylinder {
                                radius: 0.3,
                                ..default()
                            })),
                            material: MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
                            transform: Transform {
                                translation: center,
                                rotation,
                                scale: Vec3::new(1., 1., 1.),
                            },
                            ..default()
                        });
                    }
                },
                DrawingMethod::CPK => {
                    for atom in &atoms {
                        parent.spawn(PbrBundle {
                            mesh: Mesh3d(meshes.add(Sphere::default())),
                            transform: Transform::from_translation(atom.xyz().into()),
                            material: MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
                            ..default()
                        });
                    }
                    for bond in &bonds {
                        let i = bond.0;
                        let j = bond.1;
                        let start = Vec3::new(atoms[i].x(), atoms[i].y(), atoms[i].z());
                        let end = Vec3::new(atoms[j].x(), atoms[j].y(), atoms[j].z());
                        let center = (start + end) / 2.;
                        let direction = end - start;
                        let length = direction.length();
                        let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
                        parent.spawn(PbrBundle {
                            mesh: Mesh3d(meshes.add(Cylinder {
                                radius: 0.1,
                                ..default()
                            })),
                            material: MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
                            transform: Transform {
                                translation: center,
                                rotation,
                                scale: Vec3::new(1., 1., 1.),
                            },
                            ..default()
                        });
                    }
                },
                DrawingMethod::Bonds => {
                    for bond in &bonds {
                        let i = bond.0;
                        let j = bond.1;
                        let start = Vec3::new(atoms[i].x(), atoms[i].y(), atoms[i].z());
                        let end = Vec3::new(atoms[j].x(), atoms[j].y(), atoms[j].z());
                        let center = (start + end) / 2.;
                        let direction = end - start;
                        let length = direction.length();
                        let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
                        parent.spawn(PbrBundle {
                            mesh: Mesh3d(meshes.add(Cylinder {
                                radius: 0.1,
                                ..default()
                            })),
                            material: MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
                            transform: Transform {
                                translation: center,
                                rotation,
                                scale: Vec3::new(1., 1., 1.),
                            },
                            ..default()
                        });
                    }
                },
                DrawingMethod::Line => {
                    for bond in &bonds {
                        let i = bond.0;
                        let j = bond.1;
                        let start = Vec3::new(atoms[i].x(), atoms[i].y(), atoms[i].z());
                        let end = Vec3::new(atoms[j].x(), atoms[j].y(), atoms[j].z());
                        parent.spawn((
                            Mesh3d(meshes.add(LineList {
                                lines: vec![(start, end)],
                            })),
                            MeshMaterial3d(materials.add(Color::srgb(1., 1., 1.))),
                            // shader file path problem
                            // MeshMaterial3d(line_materials.add(LineMaterial {
                            //     color: LinearRgba::GREEN,
                            // })),
                        ));
                    }
                },
                DrawingMethod::Cartoon => {

                },
                DrawingMethod::NewCartoon => {

                },
                _ => {}
            });
    }
}


// https://github.com/bevyengine/bevy/blob/main/examples/3d/lines.rs
/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

        Mesh::new(
            // This tells wgpu that the positions are list of lines
            // where every pair is a start and end point
            PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the vertices positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct LineMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        const SHADER_ASSET_PATH: &str = "shader/line_material.wgsl";
        SHADER_ASSET_PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}
