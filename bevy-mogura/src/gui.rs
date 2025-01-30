use bevy::prelude::*;
use crate::camera;

pub fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut bevy_egui::EguiContext, With<bevy_window::PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };

    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut());
    });
}

pub fn set_camera_viewport(
    ui_state: Res<UiState>,
    primary_window: Query<&mut bevy_window::Window, With<bevy_window::PrimaryWindow>>,
    egui_settings: Query<&bevy_egui::EguiSettings>,
    mut cameras: Query<&mut Camera, With<camera::MainCamera>>,
) {
    let mut camera = cameras.single_mut();

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let scale_factor = window.scale_factor() * egui_settings.single().scale_factor;

    let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor;

    let physical_position = UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32);
    let physical_size = UVec2::new(viewport_size.x as u32, viewport_size.y as u32);

    let rect = physical_position + physical_size;

    let window_size = window.physical_size();

    if rect.x <= window_size.x && rect.y <= window_size.y {
        camera.viewport = Some(bevy_render::camera::Viewport {
            physical_position,
            physical_size,
            depth: 0.0..1.0,
        });
    }
}


// ref: https://github.com/vladbat00/bevy_egui/issues/47
pub fn absorb_egui_inputs(
    ui_state: Res<UiState>,
    mut contexts: bevy_egui::EguiContexts,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    // mut mouse_wheel: ResMut<Events<MouseWheel>>, //error!
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();
    if !(ctx.wants_pointer_input() || ctx.is_pointer_over_area()) {
        return;
    }

    // match ui_state.state {
    //     egui_dock::DockState<MainView> => { return; }
    //     _ => {}
    // }

    dbg!(&ui_state);

    let modifiers = [
        KeyCode::SuperLeft,
        KeyCode::SuperRight,
        KeyCode::ControlLeft,
        KeyCode::ControlRight,
        KeyCode::AltLeft,
        KeyCode::AltRight,
        KeyCode::ShiftLeft,
        KeyCode::ShiftRight,
    ];

    let pressed = modifiers.map(|key| keyboard.pressed(key).then_some(key));

    mouse.reset_all();
    // mouse_wheel.clear();
    keyboard.reset_all();

    for key in pressed.into_iter().flatten() {
        keyboard.pressed(key);
    }
}


#[derive(Debug, Eq, PartialEq)]
pub enum InspectorSelection {
    Entities,
    Resource(std::any::TypeId, String),
    Asset(std::any::TypeId, String, bevy_asset::UntypedAssetId),
}

#[derive(Debug)]
pub enum EguiWindow {
    MainView,
    Hierarchy,
    Resources,
    Assets,
    Inspector,
}

#[derive(Resource, Debug)]
pub struct UiState {
    state: egui_dock::DockState<EguiWindow>,
    viewport_rect: egui::Rect,
    selected_entities: bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities,
    selection: InspectorSelection,
}


impl UiState {
    pub fn new() -> Self {
        let mut state = egui_dock::DockState::new(vec![EguiWindow::MainView]);
        let tree = state.main_surface_mut();
        let [main, _inspector] = tree.split_right(egui_dock::NodeIndex::root(), 0.75, vec![EguiWindow::Inspector]);
        let [main, _hierarchy] = tree.split_left(main, 0.2, vec![EguiWindow::Hierarchy]);
        let [main, _bottom] = tree.split_below(main, 0.8, vec![EguiWindow::Resources, EguiWindow::Assets]);

        Self {
            state,
            selected_entities: bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities::default(),
            selection: InspectorSelection::Entities,
            viewport_rect: egui::Rect::NOTHING,
        }
    }

    pub fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            selected_entities: &mut self.selected_entities,
            selection: &mut self.selection,
        };

        egui_dock::DockArea::new(&mut self.state)
            .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }
}


pub struct TabViewer<'a> {
    world: &'a mut World,
    viewport_rect: &'a mut egui::Rect,
    selected_entities: &'a mut bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities,
    selection: &'a mut InspectorSelection,
}


impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = type_registry.read();

        match window {
            EguiWindow::MainView => {
                *self.viewport_rect = ui.clip_rect();
            }
            EguiWindow::Hierarchy => {
                let selected = bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(self.world, ui, self.selected_entities);
                if selected {
                    *self.selection = InspectorSelection::Entities;
                }
            }
            EguiWindow::Resources => select_resource(ui, &type_registry, self.selection),
            EguiWindow::Assets => select_asset(ui, &type_registry, self.world, self.selection),
            EguiWindow::Inspector => match *self.selection {
                InspectorSelection::Entities => match self.selected_entities.as_slice() {
                    &[entity] => bevy_inspector_egui::bevy_inspector::ui_for_entity_with_children(self.world, entity, ui),
                    entities => bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(self.world, entities, ui),
                },
                InspectorSelection::Resource(type_id, ref name) => {
                    ui.label(name);
                    bevy_inspector_egui::bevy_inspector::by_type_id::ui_for_resource(
                        self.world,
                        type_id,
                        ui,
                        name,
                        &type_registry,
                    );
                },
                InspectorSelection::Asset(type_id, ref name, handle) => {
                    ui.label(name);
                    bevy_inspector_egui::bevy_inspector::by_type_id::ui_for_asset(
                        self.world,
                        type_id,
                        handle,
                        ui,
                        &type_registry,
                    );
                }
            }
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiWindow::MainView)
    }
}

fn select_resource(
    ui: &mut egui::Ui,
    type_registry: &bevy_reflect::TypeRegistry,
    selection: &mut InspectorSelection,
) {
    let mut resources: Vec<_> = type_registry
        .iter()
        .filter(|registration| registration.data::<ReflectResource>().is_some())
        .map(|registration| {
            (
                registration.type_info().type_path_table().short_path(),
                registration.type_id(),
            )
        })
        .collect();

    resources.sort_by(|a, b| a.0.cmp(&b.0));

    for (resource_name, type_id) in resources {
        let selected = match *selection {
            InspectorSelection::Resource(selected, _) => selected == type_id,
            _ => false,
        };

        if ui.selectable_label(selected, resource_name).clicked() {
            *selection = InspectorSelection::Resource(type_id, resource_name.to_string());
        }
    }
}

fn select_asset(
    ui: &mut egui::Ui,
    type_registry: &bevy_reflect::TypeRegistry,
    world: &mut World,
    selection: &mut InspectorSelection,
) {
    let mut assets: Vec<_> = type_registry
        .iter()
        .filter_map(|registration| {
            let reflect_asset = registration.data::<bevy_asset::ReflectAsset>()?;
            Some((
                registration.type_info().type_path_table().short_path(),
                registration.type_id(),
                reflect_asset,
            ))
        })
        .collect();

    assets.sort_by(|(a, ..), (b, ..)| a.cmp(b));

    for (asset_name, asset_type_id, reflect_asset) in assets {
        let handles: Vec<_> = reflect_asset.ids(world).collect();

        ui.collapsing(format!("{asset_name} ({})", handles.len()), |ui| {
            for handle in handles {
                let selected = match *selection {
                    InspectorSelection::Asset(_, _, selected) => selected == handle,
                    _ => false,
                };

                if ui.selectable_label(selected, format!("{:?}", handle)).clicked() {
                    *selection = InspectorSelection::Asset(asset_type_id, asset_name.to_string(), handle);
                }
            }
        });
    }
}

