use bevy::prelude::*;
use std::collections::VecDeque;
use std::f32::consts::PI;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub row: usize,
}
impl Clone for AnimationIndices {
    fn clone(&self) -> Self {
        AnimationIndices {
            first: self.first,
            last: self.last,
            row: self.row,
        }
    }
}

#[derive(Component)]
pub struct Bird {
    pub poops: VecDeque<Posion>,
    pub turn: f32,
}

#[derive(Component)]
pub struct Slime {
    pub property: Stuff,
    pub move_max: f32,
    pub move_min: f32,
    pub move_time: Timer,
}

#[derive(Component)]
pub struct Witch;

#[derive(Component)]
pub struct TowerPosion;

#[derive(Component)]
pub struct BridPosion;

#[derive(Component)]
pub struct WitchFailed(pub f32);

#[derive(Component)]
pub struct Rollat(pub f32);

#[derive(Component)]
pub struct RollatHandle(pub f32);

#[derive(Component)]
pub struct Exploer(pub f32, pub f32);

#[derive(Component)]
pub struct Die;

#[derive(Component)]
pub struct Poop(pub Posion, pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);
impl Clone for AnimationTimer {
    fn clone(&self) -> Self {
        AnimationTimer(self.0.clone())
    }
}
#[derive(Resource)]
pub struct ResourceImage {
    pub bg1: Handle<Image>,
    pub bg2: Handle<Image>,
    pub bg3: Handle<Image>,
    pub front: Handle<Image>,
    pub bird: (SpriteSheetBundle, AnimationIndices, AnimationTimer),
    pub slime: (SpriteSheetBundle, AnimationIndices, AnimationTimer),
    pub slime_death: (SpriteSheetBundle, AnimationIndices, AnimationTimer),
    pub slime_pice: (SpriteSheetBundle, AnimationIndices, AnimationTimer),
    pub witch: (SpriteSheetBundle, AnimationIndices, AnimationTimer),
    pub rollat: Handle<Image>,
    pub handle: Handle<Image>,
}
#[derive(Resource)]
pub struct ResourcePoop {
    pub fire: Handle<Image>,
    pub light: Handle<Image>,
    pub poison: Handle<Image>,
    pub water: Handle<Image>,
    pub boom: (SpriteSheetBundle, AnimationIndices, AnimationTimer),
}
#[derive(Resource)]
pub struct ResourcePosion {
    pub posion: Handle<Image>,
    pub push: bool,
    pub lock: bool
}


#[derive(PartialEq)]
pub enum Stuff {
    Fire,
    Light,
    Poison,
    Water,
}
impl Clone for Stuff{
    fn clone(&self) -> Self {
        match self {
            Stuff::Fire => Stuff::Fire,
            Stuff::Light => Stuff::Light,
            Stuff::Poison => Stuff::Poison,
            Stuff::Water => Stuff::Water
        }
    }
}
pub struct Posion {
    pub property: Stuff,
    pub stronger: i32,
}
#[derive(Resource)]
pub struct ResourceTower {
    pub bird_speed: f32,
    pub hp: i32,
    pub posions: VecDeque<Posion>,
}

#[derive(Resource)]
pub struct ResourceWitch {
    pub stuff: Option<Stuff>,
    pub property: Option<Stuff>,
    pub stronger: i32
}

#[derive(Event)]
pub struct EventExplore {
    pub pos: Vec3,
    pub stuff: Stuff,
    pub stronger: f32
}
fn get_sprite(
    img: Handle<TextureAtlas>,
    indices: AnimationIndices,
    timer: AnimationTimer,
    pos: Vec3,
    size: Vec2,
    color: Color,
    flip_x: bool,
) -> (SpriteSheetBundle, AnimationIndices, AnimationTimer) {
    (
        SpriteSheetBundle {
            texture_atlas: img,
            sprite: TextureAtlasSprite {
                index: indices.first,
                custom_size: Some(size),
                color,
                flip_x,
                ..default()
            },
            transform: Transform::from_translation(pos),
            ..default()
        },
        indices,
        timer,
    )
}
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let bg1 = asset_server.load("Map/BG1.png");
    let bg2 = asset_server.load("Map/BG2.png");
    let bg3 = asset_server.load("Map/BG3.png");
    let front = asset_server.load("Map/Front.png");
    let bird_img = asset_server.load("BirdFly.png");
    let rollat = asset_server.load("Rollat.png");
    let handle = asset_server.load("RollatHandle.png");
    let witch_img = asset_server.load("Witch.png");
    let slime_img = asset_server.load("Slime.png");
    let fire = asset_server.load("Poop/fire.png");
    let light = asset_server.load("Poop/light.png");
    let poison = asset_server.load("Poop/poison.png");
    let water = asset_server.load("Poop/water.png");
    let slime_death = asset_server.load("SlimeDeath.png");
    let slime_pice = asset_server.load("SlimePice.png");
    let boom_img = asset_server.load("Boom.png");
    let posion = asset_server.load("Posion.png");

    let bird_atlas = TextureAtlas::from_grid(bird_img, Vec2::new(16., 16.), 8, 1, None, None);
    let bird_handle = texture_atlases.add(bird_atlas);
    let animation_indices = AnimationIndices {
        first: 0,
        last: 8,
        row: 0,
    };
    let animation_timer = AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating));

    let bird: (SpriteSheetBundle, AnimationIndices, AnimationTimer) = get_sprite(
        bird_handle,
        animation_indices,
        animation_timer,
        Vec3::new(0., -69.5, 10.),
        Vec2::new(20., 20.),
        Color::from([1., 1., 1.]),
        true,
    );

    let witch_atlas = TextureAtlas::from_grid(witch_img, Vec2::new(32., 48.), 4, 3, None, None);
    let witch_handle = texture_atlases.add(witch_atlas);
    let witch = get_sprite(
        witch_handle,
        AnimationIndices {
            row: 0,
            last: 4,
            first: 0,
        },
        AnimationTimer(Timer::from_seconds(0.4, TimerMode::Repeating)),
        Vec3::new(138., -77., 30.),
        Vec2::new(32., 48.),
        Color::from([1., 1., 1.]),
        false,
    );

    let slime_atlas = TextureAtlas::from_grid(slime_img, Vec2::new(32., 32.), 4, 1, None, None);
    let slime_handle = texture_atlases.add(slime_atlas);
    let slime = get_sprite(
        slime_handle,
        AnimationIndices {
            first: 0,
            last: 4,
            row: 0,
        },
        AnimationTimer(Timer::from_seconds(0.07, TimerMode::Repeating)),
        Vec3::new(-228., -213.5, 25.),
        Vec2::new(32., 32.),
        Color::from([1., 1., 1.]),
        false,
    );

    let slime_death_atlas = TextureAtlas::from_grid(slime_death, Vec2::new(32., 32.), 7, 1, None, None);
    let slime_death_handle = texture_atlases.add(slime_death_atlas);
    let slime_death_entity = get_sprite(
        slime_death_handle,
        AnimationIndices{
            first: 0,
            last: 7,
            row:0
        },
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        Vec3::new(0.,-213.5,0.),
        Vec2::new(32.,32.),
        Color::from([1., 1., 1.]),
        false,
    );

    let slime_pice_atlas = TextureAtlas::from_grid(slime_pice, Vec2::new(8., 8.), 4, 1, None, None);
    let slime_pice_handle = texture_atlases.add(slime_pice_atlas);
    let slime_pice_entity = get_sprite(
        slime_pice_handle,
        AnimationIndices{
            first: 0,
            last: 4,
            row:0
        },
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        Vec3::new(0.,-213.5,0.),
        Vec2::new(8.,8.),
        Color::from([1., 1., 1.]),
        false,
    );

    let boom_atlas = TextureAtlas::from_grid(boom_img, Vec2::new(32., 32.), 6, 1, None, None);
    let boom_handle = texture_atlases.add(boom_atlas);
    let boom = get_sprite(
        boom_handle,
        AnimationIndices{
            first: 0,
            last: 4,
            row:0
        },
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        Vec3::new(0.,-213.5,0.),
        Vec2::new(32.,32.),
        Color::from([1., 1., 1.]),
        false,
    );
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::splat(0.85)).with_translation(Vec3::Y * -60.),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: bg1.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(393., 851.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: bg2.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(393., 851.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 2.),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: bg3.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(393., 851.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 3.),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: front.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(393., 851.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 24.),
        ..default()
    });
    commands.spawn(bird.clone()).insert(Bird {
        poops: VecDeque::with_capacity(3),
        turn: 1.,
    });
    commands.spawn(slime.clone()).insert(Slime {
        property: Stuff::Fire,
        move_max: 3.,
        move_min: -1.,
        move_time: Timer::from_seconds(0.8, TimerMode::Repeating),
    });
    commands.spawn(
        (
            SpriteBundle{
                texture: rollat.clone(),
                transform: Transform::from_translation(Vec3::new(116., 10., 40.)),
                ..default()
            },
            Rollat(PI*1.5)
            )
    );
    commands.spawn(
        (
            SpriteBundle{
                texture: handle.clone(),
                transform: Transform::from_translation(Vec3::new(116., 10., 41.))
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0., 0., 0.2)),
                ..default()
            },
            RollatHandle(0.)
        )
    );
    commands.spawn(witch.clone()).insert(Witch);
    commands.insert_resource(ResourceImage {
        bg1,
        bg2,
        bg3,
        front,
        bird,
        rollat,
        handle,
        slime,
        slime_death: slime_death_entity,
        slime_pice: slime_pice_entity,
        witch,
    });
    commands.insert_resource(ResourcePoop {
        fire,
        light,
        poison,
        water,
        boom
    });
    commands.insert_resource(
        ResourcePosion{
            posion,
            push: false,
            lock: true
        }
    );

    commands.insert_resource(ResourceTower {
        bird_speed: 50.,
        posions: VecDeque::with_capacity(10),
        hp: 10,
    });
    commands.insert_resource(ResourceWitch {
        property: None,
        stronger: 0,
        stuff: None
    })
}

fn animation_forward(
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &AnimationIndices,
    )>,
    res: Res<Time>,
) {
    let delta = res.delta();
    for (mut timer, mut sprite, indices) in query.iter_mut() {
        timer.tick(delta);
        if timer.finished() {
            if sprite.index < indices.last + indices.last * indices.row - 1 {
                sprite.index += 1;
            } else {
                sprite.index = indices.first + indices.last * indices.row;
            }
        }
    }
}

pub struct ResourceSetupPlugin;

impl Plugin for ResourceSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_event::<EventExplore>()
            .add_systems(Update, animation_forward);
    }
}
