use bevy::prelude::*;

#[derive(Resource)]
pub struct ObjectAtlases {
    pub trees_image: Handle<Image>,
    pub trees_layout: Handle<TextureAtlasLayout>,

    pub bushes_image: Handle<Image>,
    pub bushes_layout: Handle<TextureAtlasLayout>,
}

pub fn load_object_atlases(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let trees_image = asset_server.load("trees.png");
    let bushes_image = asset_server.load("bushes.png");

    let trees_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        4,
        4,
        None,
        None,
    ));

    let bushes_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        4,
        4,
        None,
        None,
    ));

    commands.insert_resource(ObjectAtlases {
        trees_image,
        trees_layout,
        bushes_image,
        bushes_layout,
    });
}
