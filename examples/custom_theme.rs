use bevy::prelude::*;
use sickle_ui::{
    prelude::*, theme::theme_colors::ThemeColors, widgets::inputs::slider::SliderAxis,
    SickleUiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sickle UI -  Custom Material Theme".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SickleUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, on_theme_loaded)
        .run();
}

#[derive(Resource)]
struct CustomMaterialTheme {
    handle: Handle<ThemeColors>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(CustomMaterialTheme {
        handle: asset_server.load::<ThemeColors>("themes/material-theme.json"),
    });

    let main_camera = commands
        .spawn((Camera3dBundle {
            camera: Camera {
                order: 1,
                clear_color: Color::BLACK.into(),
                ..default()
            },
            ..default()
        },))
        .id();

    commands.ui_builder(UiRoot).container(
        (
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            TargetCamera(main_camera),
        ),
        |container| {
            container.floating_panel(
                FloatingPanelConfig {
                    title: Some("My Panel".into()),
                    ..default()
                },
                FloatingPanelLayout {
                    size: Vec2::splat(300.),
                    position: Some(Vec2::splat(100.)),
                    droppable: false,
                },
                |panel| {
                    panel.slider(SliderConfig {
                        label: Some("Slider".into()),
                        min: 0.,
                        max: 10.,
                        initial_value: 5.,
                        show_current: true,
                        axis: SliderAxis::Horizontal,
                    });

                    panel.radio_group(vec!["A", "B"], Some(0), false);
                },
            );
        },
    );
}

fn on_theme_loaded(
    mut theme_data: ResMut<ThemeData>,
    mut reader: EventReader<AssetEvent<ThemeColors>>,
    custom_theme: Res<CustomMaterialTheme>,
    themes: Res<Assets<ThemeColors>>,
) {
    for event in reader.read() {
        if event.is_loaded_with_dependencies(&custom_theme.handle) {
            let Some(theme_colors) = themes.get(&custom_theme.handle) else {
                warn!("none!?");
                return;
            };
            theme_data.colors = theme_colors.clone();
        }
    }
}
