use std::time::Duration;

use bevy::{
    asset::HandleId,
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        texture::ImageSampler,
    },
};
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: CONWAYS_SCREEN_SIZE.x,
                        height: CONWAYS_SCREEN_SIZE.y,
                        title: "To do".to_string(),
                        resizable: true,
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSampler::nearest_descriptor(),
                }),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .add_system(swap_colours)
        .run();
}

const CONWAYS_MAP_SIZE: Vec2 = Vec2::new(160.0, 160.0);
const CONWAYS_SCREEN_SIZE: Vec2 = Vec2::new(800.0, 800.0);

#[derive(Resource)]
struct GameOfLifeTimer(Timer);

#[derive(Resource)]
struct GameOfLife(HandleId);

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: (CONWAYS_MAP_SIZE.x) as u32,
            height: (CONWAYS_MAP_SIZE.y) as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    for (i, x) in image.data.iter_mut().enumerate() {
        if (i + 1) % 4 == 0 {
            *x = rand::thread_rng().gen_range(0..=1) * 255;
        }
    }

    let image = images.add(image.clone());
    let id = image.id();

    commands.insert_resource(GameOfLife(id));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(CONWAYS_SCREEN_SIZE),
            ..default()
        },
        texture: image,
        ..default()
    });

    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(GameOfLifeTimer(Timer::new(
        Duration::from_millis(500),
        TimerMode::Repeating,
    )))
}

fn swap_colours(
    mut images: ResMut<Assets<Image>>,
    id: Res<GameOfLife>,
    // time: Res<Time>,
    // mut timer: ResMut<GameOfLifeTimer>
) {
    // timer.0.tick(time.delta());
    // if timer.0.just_finished() {

    let handle = Handle::weak(id.0);

    if let Some(image) = images.get_mut(&handle) {

        let mut new_image: Vec<u8> = vec![0; image.data.len()];

        let old_image: Vec<u8> = image
            .data
            .iter()
            .enumerate()
            .filter(|(i, _)| (i + 1) % 4 == 0)
            .map(|x| *x.1)
            .collect();

        for (i, x) in old_image.iter().enumerate() {
            let num_neighbors = count_neighbors(&old_image, i.try_into().unwrap());

            if !(2..=3).contains(&num_neighbors) {
                new_image[i] = 0
            } else if num_neighbors == 2 {
                new_image[i] = *x
            } else if num_neighbors == 3 {
                new_image[i] = 255;
            }
        }

        for (i, x) in image.data.iter_mut().enumerate() {
            if (i + 1) % 4 == 0 {
                *x = new_image[((i + 1) / 4) - 1];
            }
        }
    }
    // }
}

fn count_neighbors(image: &Vec<u8>, pixel: i32) -> i32 {
    let mut total = 0;

    for x in (0..3).map(|x| x - 1) {
        for y in (0..3).map(|y| y - 1) {
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
                    continue;
                }

                let index = (pixel + index) as usize;

                if index >= image.len() {
                    continue;
                } else {
                    let number = image[index];

                    if number == 255 {
                        total += 1;
                    }
                }
            }
        }
    }
    total
}
