use bevy::{prelude::*, render::{render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}}, asset::HandleId};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: CONWAYS_MAP_SIZE.x,
                height: CONWAYS_MAP_SIZE.y,
                title: "To do".to_string(),
                resizable: true,
                ..Default::default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(setup)
        .add_system(swap_colours)
        .run();
}

const CONWAYS_MAP_SIZE: Vec2 = Vec2::new(640.0, 640.0);



#[derive(Resource)]
struct GameOfLife(HandleId);

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: (CONWAYS_MAP_SIZE.x / 4.0) as u32,
            height: (CONWAYS_MAP_SIZE.y / 4.0) as u32,
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
            custom_size: Some(Vec2::new(CONWAYS_MAP_SIZE.x, CONWAYS_MAP_SIZE.y)),
            ..default()
        },
        texture: image,
        ..default()
    });

    commands.spawn(Camera2dBundle::default());

}

fn swap_colours (
    mut images: ResMut<Assets<Image>>,
    id: Res<GameOfLife>,
    keys: Res<Input<KeyCode>>
) {
    if keys.just_pressed(KeyCode::Space) {
        let handle = Handle::weak(id.0);

        if let Some(image) = images.get_mut(&handle) {

            let new_image: Vec<_> = image.data.clone().iter().enumerate().filter(|(i, _)| (i + 1) % 4 == 0 ).map(|x| 255 - x.1).collect();

            for (i, x) in image.data.clone().iter().enumerate() {
                if (*x as u32 + 1) % 4 == 0 {
                    image.data[(*x) as usize] = new_image[i / 4]
                }
            }
        }
    }
}

fn count_neighbors (
    image: &Image,
    pixel: i32,
) -> i32 {

    let mut total = 0;

    for x in (0..3).map(|x| x as i32 - 1) {
        for y in (0..3).map(|y| y as i32 - 1) {
            if x == y && x == 0 {
                continue;
            } else {
                // special cases

                // find if the thing has looped:
                // it has looped if the pixel is on the very left or very right
                // very left == pixel mod 640 = 0 ---> skip 
                // very right == (pixel +1) mod 640 = 0

                let index = x + (y * CONWAYS_MAP_SIZE.x as i32);

                // println!("{}", pixel + index);

                if pixel + index < 0 
                    || (pixel % CONWAYS_MAP_SIZE.x as i32 == 0 && x < 0)
                    || ((pixel + 1) % CONWAYS_MAP_SIZE.x as i32 == 0 && x > 0) 
                {
                    // println!("skipped");
                    continue
                }

                let index = (pixel + index) as usize;

                if !index < image.data.len() {
                    continue
                } else {
                    let number = image.data[index];

                    if number == 255 {
                        total += 1;
                    }
                }
            }
        }
    }
    total
}