use bevy::prelude::*;
use mogura_io::prelude::*;

// #[derive(Component)]
// pub struct StructureParams {
//     pub drawing_method: DrawingMethod,
//     pub structure_data: Box<dyn StructureData>,
// }
//
// impl Default for StructureParams {
//     fn default() -> Self {
//         Self {
//             drawing_method: DrawingMethod::VDW,
//         }
//     }
// }

#[derive(Debug)]
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
) {
    let test_file = "/Users/th/works/Project_mogura/Project_moveit/input/water_obabel_3.pdb";
    let test_file = "/Users/th/works/Project_mogura/Project_moveit/input/8GNG.pdb";

    let structure_data = structure_loader(test_file);

    // todo: make parent hierarchy
    for atom in structure_data.atoms() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Sphere::default()),
            transform: Transform::from_translation(atom.xyz().into()),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        });
    }
}
