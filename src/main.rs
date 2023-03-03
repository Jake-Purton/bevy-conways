use bevy::{prelude::*, render::{render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}}, asset::HandleId};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 640.0,
                height: 640.0,
                title: "To do".to_string(),
                resizable: true,
                ..Default::default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .add_system(swap_colours)
        .run();
}



#[derive(Resource)]
struct GameOfLife(HandleId);

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: 640,
            height: 640,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let image = images.add(image.clone());
    let id = image.id();

    commands.insert_resource(GameOfLife (id));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(640.0, 640.0)),
            ..default()
        },
        texture: image,
        ..default()
    });

    commands.spawn(Camera2dBundle::default());

}

fn swap_colours (
    mut images: ResMut<Assets<Image>>,
    id: Res<GameOfLife>
) {

    let handle = Handle::weak(id.0);

    if let Some(image) = images.get_mut(&handle) {

        // println!("{:?}", image.size());

        for i in 0..409600 {
            image.data[i] = 255;
        }
    }
}