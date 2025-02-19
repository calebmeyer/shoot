use bevy::{prelude::*, window::WindowResized};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_resize)
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&Window>) {
    let image = asset_server.load("space_tile.png");

    let window = window.single();
    let width = window.width();
    let height = window.height();
    info!("Window width: {}", width);
    info!("Window height: {}", height);

    // Spawn camera
    commands.spawn(Camera2d);

    commands
        .spawn(Sprite {
            image: image.clone(),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 0.5,
            },
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        })
        .insert(Background(image.clone()));

    // Instructions
    // commands.spawn((
    //     TextFont::from_font_size(32.0),
    //     Text::new("Arrow keys to move\n"),
    //     Instructions,
    // ));

    // commands
    //     .spawn((
    //         Sprite::from_image(asset_server.load("spaceship.png")),
    //         // Too big, make it 1/10th the size
    //         Transform::from_scale(Vec3::new(0.1, 0.1, 0.1))
    //             .with_translation(Vec3::new(0.0, 50.0, 1.0)),
    //     ))
    //     .insert(Player);
}

#[derive(Component)]
pub struct Background(Handle<Image>);

pub fn handle_resize(
    mut commands: Commands,
    mut resize_reader: EventReader<WindowResized>,
    background_entity: Query<Entity, With<Background>>,
    background: Query<&Background>,
) {
    for event in resize_reader.read() {
        info!("Window resized to {}x{}", event.width, event.height);

        // The below code causes a crash
        // let old = background_entity.single();
        // commands.entity(old).despawn();

        let image = background.single().0.clone();

        // Update the sprite's custom size when the window is resized
        commands
            .spawn(Sprite {
                image: image.clone(),
                image_mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: true,
                    stretch_value: 0.5,
                },
                custom_size: Some(Vec2::new(event.width, event.height)),
                ..default()
            })
            .insert(Background(image.clone()));
    }

    resize_reader.clear();
}
