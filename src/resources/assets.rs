extern crate specs_derive;

use std::collections::HashMap;

use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, RonFormat, ProgressCounter},
    assets::{AssetStorage, Loader},
    renderer::{
        sprite::SpriteSheetHandle,
        ImageFormat, SpriteSheet, 
            SpriteSheetFormat, Texture,

    },
    prelude::*,
};

use crate::components::{AnimationPrefabData};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum AssetType {
    //Background,
    //Laser,
    LaserImpact,
    Thrust,
    //LightShip,
    //DarkShip,
}

#[derive(Default)]
pub struct SpriteSheetList {
    sprite_sheets: HashMap<AssetType, SpriteSheetHandle>,
}

impl SpriteSheetList {
    pub fn insert(&mut self, asset_type: AssetType, sprite_sheet_handle: SpriteSheetHandle) {
        self.sprite_sheets.insert(asset_type, sprite_sheet_handle);
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets.get(&asset_type)
    }
}

#[derive(Default)]
pub struct PrefabList {
    prefabs: HashMap<AssetType, Handle<Prefab<AnimationPrefabData>>>,
}

impl PrefabList {
    pub fn insert(
        &mut self,
        asset_type: AssetType,
        prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    ) {
        self.prefabs.insert(asset_type, prefab_handle);
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&Handle<Prefab<AnimationPrefabData>>> {
        self.prefabs.get(&asset_type)
    }
}

/// Loads SpriteSheetHandle's for all the assets in teh AssetType list into the world
pub fn load_assets(world: &mut World, asset_type_list: Vec<AssetType>) -> ProgressCounter {
    let mut sprite_sheet_list = SpriteSheetList::default();
    let mut prefab_list = PrefabList::default();
    let mut progress_counter = ProgressCounter::new();

    for &asset_type in asset_type_list.iter() {
        let (texture_path, ron_path) = match asset_type {
            AssetType::LaserImpact => ("", "prefab/small_explosion.ron"),
            AssetType::Thrust => ("texture/thrust.png", "texture/thrust.ron"),
        };

        match asset_type {
            AssetType::Thrust => {
                let sprite_sheet_handle = 
                    get_sprite_sheet_handle(world, texture_path, ron_path, &mut progress_counter);
                sprite_sheet_list.insert(asset_type, sprite_sheet_handle);
            }
            AssetType::LaserImpact => {
                let prefab_handle = get_animation_prefab_handle(world, ron_path, &mut progress_counter);
                prefab_list.insert(asset_type, prefab_handle);
            }
        };
    };

    world.insert(sprite_sheet_list);
    world.insert(prefab_list);
    progress_counter
}

fn get_animation_prefab_handle(
    world: &mut World,
    ron_path: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<AnimationPrefabData>> {
    world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load(ron_path, RonFormat, progress_counter)
    })
}

pub fn get_sprite_sheet_handle(
    world: &World,
    texture_path: &str,
    ron_path: &str,
    progress_counter: &mut ProgressCounter,
) -> SpriteSheetHandle {
    // Load the sprite sheet needed to render graphics
    let texture_handle = {
        let loader = &world.read_resource::<Loader>();
        let texture_storage = &world.read_resource::<AssetStorage<Texture>>();
        loader.load(texture_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = &world.read_resource::<Loader>();
    let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        progress_counter,
        &sprite_sheet_store,
    )
}

/// helper function to load sprites
pub fn load_sprite_sheet(world: &mut World, path: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}.png", path),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}.ron", path),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}