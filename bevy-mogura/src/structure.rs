use crate::MoguraPlugins;
use crate::*;
// use bevy::prelude::*;
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
use mogura_io::prelude::*;

// pub(crate) const BOND_LENGTH_PADDING: f32 = 0.3;

#[derive(Clone)]
pub struct MoguraStructurePlugins;

impl Plugin for MoguraStructurePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<LineMaterial>::default())
            .add_systems(Update, update_structure);
    }
}

#[derive(Copy, Eq, Hash, Debug, Clone, PartialEq)]
pub enum DrawingMethod {
    Line,
    VDW,
    Licorise,
    Bonds,
    Cartoon,
    NewCartoon,
}

#[derive(Component)]
pub struct StructureParams {
    pub drawing_method: DrawingMethod,
}

#[derive(Component)]
pub struct AtomID {
    id: usize,
}

impl AtomID {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Component)]
pub struct BondID {
    pub atomid1: AtomID,
    pub atomid2: AtomID,
}

impl BondID {
    pub fn new(id1: usize, id2: usize) -> Self {
        Self {
            atomid1: AtomID::new(id1),
            atomid2: AtomID::new(id2),
        }
    }
    pub fn atomid1(&self) -> usize {
        self.atomid1.id()
    }
    pub fn atomid2(&self) -> usize {
        self.atomid2.id()
    }
}

pub trait MoguraSelection {
    fn eval(&self, atom: &Atom) -> bool;
    fn select_atoms(&self, atoms: &Vec<Atom>) -> std::collections::HashSet<usize> {
        atoms
            .iter()
            .filter(|atom| self.eval(atom))
            .map(|atom| atom.id())
            .collect()
    }
    fn select_atoms_bonds(
        &self,
        atoms: &Vec<Atom>,
        bonds: &Vec<(usize, usize)>,
    ) -> (
        std::collections::HashSet<usize>,
        std::collections::HashSet<(usize, usize)>,
    ) {
        let selected_atoms = self.select_atoms(atoms);
        let selected_bonds = bonds
            .iter()
            .filter(|bond| selected_atoms.contains(&bond.0) && selected_atoms.contains(&bond.1))
            .map(|bond| *bond)
            .collect();
        (selected_atoms, selected_bonds)
    }
}

impl MoguraSelection for mogura_asl::Selection {
    fn eval(&self, atom: &Atom) -> bool {
        match self {
            mogura_asl::Selection::All => true,
            mogura_asl::Selection::ResName(names) => {
                names.iter().any(|name| name == &atom.residue_name())
            }
            mogura_asl::Selection::ResId(ids) => {
                ids.iter().any(|id| *id == atom.residue_id() as usize)
            }
            mogura_asl::Selection::Index(indices) => {
                indices.iter().any(|index| index == &atom.atom_id())
            }
            mogura_asl::Selection::Name(names) => {
                names.iter().any(|name| name == &atom.atom_name())
            }
            mogura_asl::Selection::Not(selection) => !selection.eval(atom),
            mogura_asl::Selection::And(selections) => selections.iter().all(|s| s.eval(atom)),
            mogura_asl::Selection::Or(selections) => selections.iter().any(|s| s.eval(atom)),
            mogura_asl::Selection::Braket(selection) => selection.eval(atom),
            mogura_asl::Selection::Protein => atom.is_protein(),
            mogura_asl::Selection::Water => atom.is_water(),
            mogura_asl::Selection::Ion => atom.is_ion(),
            mogura_asl::Selection::Backbone => atom.is_backbone(),
            mogura_asl::Selection::Sidechain => atom.is_sidechain(),
        }
    }
}

fn update_structure(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    mut mogura_state: ResMut<MoguraState>,
    mut current_visualized_structure: Query<(Entity, &mut structure::StructureParams)>,
    mut trackball_camera: Query<&mut bevy_trackball::TrackballCamera, With<Camera>>,
) {
    if mogura_state.redraw {
        mogura_state.redraw = false;

        for (entity, structure_params) in current_visualized_structure.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }

        let structure_data = match mogura_state.structure_data.as_ref() {
            Some(data) => data,
            None => {
                return;
            }
        };
        let atoms = structure_data.atoms();
        let bonds = structure_data.bonds_indirected();

        match &mogura_state.structure_data {
            Some(structure_data) => {
                let center = structure_data.center();
                let center_vec = Vec3::new(center[0], center[1], center[2]);
                let mut trackball_camera = trackball_camera.single_mut();
                trackball_camera.frame.set_target(center_vec.into());
            }
            None => (),
        }

        commands
            .spawn((
                StructureParams {
                    drawing_method: mogura_state.drawing_method.clone(),
                },
                GlobalTransform::default(),
                Transform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
            ))
            .with_children(|parent| match mogura_state.drawing_method {
                DrawingMethod::VDW => {
                    let sphere = meshes.add(Sphere {
                        radius: 0.3,
                        ..default()
                    });
                    let mut mesh_materials = std::collections::HashMap::new();

                    for atom in atoms {
                        if !mesh_materials.contains_key(&atom.element()) {
                            let material = materials.add(atom.color());
                            mesh_materials.insert(atom.element(), material);
                        }
                        parent.spawn((
                            PbrBundle {
                                mesh: Mesh3d(sphere.clone()),
                                transform: Transform::from_translation(atom.xyz().into()),
                                material: MeshMaterial3d(
                                    mesh_materials.get(&atom.element()).unwrap().clone(),
                                ),
                                ..default()
                            },
                            AtomID::new(atom.id()),
                        ));
                    }
                }
                DrawingMethod::Licorise => {
                    let sphere = meshes.add(Sphere {
                        radius: 0.3,
                        ..default()
                    });
                    let mut mesh_materials = std::collections::HashMap::new();

                    let selection =
                        mogura_asl::parse_selection(&mogura_state.atom_selection).unwrap();
                    let (selected_atoms, selected_bonds) =
                        selection.select_atoms_bonds(atoms, &bonds);

                    for atom in atoms {
                        if !selected_atoms.contains(&atom.id()) {
                            continue;
                        }

                        if !mesh_materials.contains_key(&atom.element()) {
                            let material = materials.add(atom.color());
                            mesh_materials.insert(atom.element(), material);
                        }
                        parent.spawn((
                            PbrBundle {
                                mesh: Mesh3d(sphere.clone()),
                                transform: Transform::from_translation(atom.xyz().into()),
                                material: MeshMaterial3d(
                                    mesh_materials.get(&atom.element()).unwrap().clone(),
                                ),
                                ..default()
                            },
                            AtomID::new(atom.id()),
                        ));
                    }

                    let cylinder = meshes.add(Cylinder {
                        radius: 0.2,
                        ..default()
                    });
                    for bond in bonds {
                        if !selected_bonds.contains(&bond) {
                            continue;
                        }

                        let i = bond.0;
                        let j = bond.1;
                        let start = Vec3::new(atoms[i].x(), atoms[i].y(), atoms[i].z());
                        let end = Vec3::new(atoms[j].x(), atoms[j].y(), atoms[j].z());
                        let pos_1_4 = start + (end - start) * 0.25;
                        // let pos_1_4 = start + (end - start) * 0.25 * (1. + BOND_LENGTH_PADDING);
                        let pos_3_4 = start + (end - start) * 0.75;
                        // let pos_3_4 = end + (start - end) * 0.25 * (1. + BOND_LENGTH_PADDING);
                        let direction = end - start;
                        let length = direction.length();
                        let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
                        parent.spawn((
                            PbrBundle {
                                mesh: Mesh3d(cylinder.clone()),
                                material: MeshMaterial3d(
                                    mesh_materials.get(&atoms[i].element()).unwrap().clone(),
                                ),
                                transform: Transform {
                                    translation: pos_1_4,
                                    rotation,
                                    // scale: Vec3::ONE * length / 2. * (1. - BOND_LENGTH_PADDING),
                                    scale: Vec3::ONE * length / 2.,
                                },
                                ..default()
                            },
                            BondID::new(atoms[i].id(), atoms[j].id()),
                        ));
                        parent.spawn((
                            PbrBundle {
                                mesh: Mesh3d(cylinder.clone()),
                                material: MeshMaterial3d(
                                    mesh_materials.get(&atoms[j].element()).unwrap().clone(),
                                ),
                                transform: Transform {
                                    translation: pos_3_4,
                                    rotation,
                                    // scale: Vec3::ONE * length / 2. * (1. - BOND_LENGTH_PADDING),
                                    scale: Vec3::ONE * length / 2.,
                                },
                                ..default()
                            },
                            BondID::new(atoms[j].id(), atoms[i].id()),
                        ));
                    }
                }
                DrawingMethod::Bonds => {
                    let cylinder = meshes.add(Cylinder {
                        radius: 0.2,
                        ..default()
                    });
                    let mut mesh_materials = std::collections::HashMap::new();

                    for bond in bonds {
                        let i = bond.0;
                        let j = bond.1;
                        let start = Vec3::new(atoms[i].x(), atoms[i].y(), atoms[i].z());
                        let end = Vec3::new(atoms[j].x(), atoms[j].y(), atoms[j].z());
                        let pos_1_4 = start + (end - start) * 0.25;
                        // let pos_1_4 = start + (end - start) * 0.25 * (1. + BOND_LENGTH_PADDING);
                        let pos_3_4 = start + (end - start) * 0.75;
                        // let pos_3_4 = end + (start - end) * 0.25 * (1. + BOND_LENGTH_PADDING);
                        let direction = end - start;
                        let length = direction.length();
                        let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
                        if !mesh_materials.contains_key(&atoms[i].element()) {
                            let material = materials.add(atoms[i].color());
                            mesh_materials.insert(atoms[i].element(), material);
                        }
                        if !mesh_materials.contains_key(&atoms[j].element()) {
                            let material = materials.add(atoms[j].color());
                            mesh_materials.insert(atoms[j].element(), material);
                        }
                        parent.spawn((
                            PbrBundle {
                                mesh: Mesh3d(cylinder.clone()),
                                material: MeshMaterial3d(
                                    mesh_materials.get(&atoms[i].element()).unwrap().clone(),
                                ),
                                transform: Transform {
                                    translation: pos_1_4,
                                    rotation,
                                    // scale: Vec3::ONE * length / 2. * (1. - BOND_LENGTH_PADDING),
                                    scale: Vec3::ONE * length / 2.,
                                },
                                ..default()
                            },
                            BondID::new(atoms[i].id(), atoms[j].id()),
                        ));
                        parent.spawn((
                            PbrBundle {
                                mesh: Mesh3d(cylinder.clone()),
                                material: MeshMaterial3d(
                                    mesh_materials.get(&atoms[j].element()).unwrap().clone(),
                                ),
                                transform: Transform {
                                    translation: pos_3_4,
                                    rotation,
                                    scale: Vec3::ONE * length / 2.,
                                },
                                ..default()
                            },
                            BondID::new(atoms[j].id(), atoms[i].id()),
                        ));
                    }
                }
                DrawingMethod::Line => {
                    let mut mesh_materials = std::collections::HashMap::new();
                    let line = meshes.add(LineList {
                        lines: vec![(Vec3::new(0., -0.5, 0.), Vec3::new(0., 0.5, 0.))],
                        // lines: vec![(Vec3::new(0., 0., 0.), Vec3::new(0., 1., 0.))],
                    });
                    for bond in bonds {
                        let i = bond.0;
                        let j = bond.1;
                        let start = Vec3::new(atoms[i].x(), atoms[i].y(), atoms[i].z());
                        let end = Vec3::new(atoms[j].x(), atoms[j].y(), atoms[j].z());
                        let pos_1_4 = start + (end - start) * 0.25;
                        let pos_3_4 = start + (end - start) * 0.75;
                        let direction = end - start;
                        let length = direction.length();
                        let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
                        if !mesh_materials.contains_key(&atoms[i].element()) {
                            let material = line_materials.add(LineMaterial {
                                color: LinearRgba::from(atoms[i].color()),
                            });
                            mesh_materials.insert(atoms[i].element(), material);
                        }
                        if !mesh_materials.contains_key(&atoms[j].element()) {
                            let material = line_materials.add(LineMaterial {
                                color: LinearRgba::from(atoms[j].color()),
                            });
                            mesh_materials.insert(atoms[j].element(), material);
                        }
                        parent.spawn((
                            Mesh3d(line.clone()),
                            MeshMaterial3d(
                                mesh_materials.get(&atoms[i].element()).unwrap().clone(),
                            ),
                            Transform {
                                translation: pos_1_4,
                                rotation,
                                scale: Vec3::ONE * length / 2.,
                            },
                            BondID::new(atoms[i].id(), atoms[j].id()),
                        ));
                        parent.spawn((
                            Mesh3d(line.clone()),
                            MeshMaterial3d(
                                mesh_materials.get(&atoms[j].element()).unwrap().clone(),
                            ),
                            Transform {
                                translation: pos_3_4,
                                rotation,
                                scale: Vec3::ONE * length / 2.,
                            },
                            BondID::new(atoms[j].id(), atoms[i].id()),
                        ));
                    }
                }
                DrawingMethod::Cartoon => {}
                DrawingMethod::NewCartoon => {}
                _ => {}
            });
    }
}

trait AtomColor {
    fn color(&self) -> Color;
}

impl AtomColor for Atom {
    fn color(&self) -> Color {
        match self.element() {
            Some(Element::H) => Color::srgb(0.4, 0.5, 0.3),
            Some(Element::C) => Color::srgb(0.0, 0.4, 0.3),
            Some(Element::N) => Color::srgb(0.0, 0.1, 0.9),
            Some(Element::O) => Color::srgb(0.6, 0.1, 0.1),
            _ => Color::srgb(0.5, 0.5, 0.0),
        }
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

pub const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(12345678912345678912);

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(SHADER_HANDLE.clone())
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
