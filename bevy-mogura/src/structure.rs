use crate::MoguraPlugins;
use bevy::prelude::*;
use mogura_io::prelude::*;

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
    NewCartoon,
}

pub fn setup_structure(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mogura_plugins: Res<MoguraPlugins>,
) {
    if let Some(structure_file) = &mogura_plugins.input_structure {
        let structure_data = structure_loader(&structure_file);
        let atoms = structure_data.atoms().clone();
        let drawing_method = DrawingMethod::VDW;

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
                    for atom in atoms {
                        parent.spawn(PbrBundle {
                            mesh: meshes.add(Sphere::default()),
                            transform: Transform::from_translation(atom.xyz().into()),
                            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
                            ..default()
                        });
                    }
                }
                _ => {}
            });
    }
}
