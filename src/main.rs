
use bevy::{prelude::*, window::WindowRef, render::view::WindowSystem, winit::WinitWindows};


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}


fn setup_system(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>
) {
    // camera
    cmds.spawn(Camera2dBundle::default());

    let mut w = windows.iter_mut().next().unwrap();
    w.title = "Hello World!".to_string();
    // speech bubble background and layout
    cmds.spawn(SpriteBundle {
        texture: asset_server.load("textures/textbox.png"),
        transform: Transform {
            translation: Vec3::new(0.0, -300.0, 0.0),
            scale: Vec3::new(w.width() / 400.0, 1.0, 1.0),
            ..default()
        },
        ..default()
    });

    // add character sprite
    cmds.spawn(SpriteBundle {
        texture: asset_server.load("characters/koishi.png"),
        transform: Transform {
            // in the middle of the screen with a z-index below the text
            translation: Vec3::new(0.0, -50.0, 0.0),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..default()
        },
        ..default()
    });


    cmds.spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            position: UiRect {
                bottom: Val::Percent(10.0),
                left: Val::Percent(15.0),
                ..default()
            },
            ..default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: "There is a pipe bomb in your mailbox.".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/Quicksand-VariableFont_wght.ttf"),
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                },
            ],
            ..default()
        },
        ..default()
    });

    // add player sprite
}

