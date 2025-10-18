use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::rendering::hd2d_pipeline::{BillboardSprite, DepthSorted, Hd2dSettings};
use crate::rendering::lighting::Hd2dLightingMaterial;
use crate::world::chunk::{ChunkCoord, ChunkSettings, LayerData, WorldChunks};
use crate::world::generation::{TerrainChunkUpdated, TerrainSettings};
use crate::world::tiles::{TileId, TileRegistry};

pub struct TerrainRenderPlugin;

impl Plugin for TerrainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainMeshMaterial>().add_systems(
            PostUpdate,
            (apply_chunk_mesh_updates, cleanup_unloaded_chunk_meshes),
        );
    }
}

#[derive(Resource)]
struct TerrainMeshMaterial {
    base: Handle<Hd2dLightingMaterial>,
}

impl FromWorld for TerrainMeshMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<Hd2dLightingMaterial>>();
        let mut material = Hd2dLightingMaterial::default();
        material.params.base_color = Vec4::ONE;
        let handle = materials.add(material);
        Self { base: handle }
    }
}

#[derive(Component)]
struct ChunkMeshEntity {
    coord: ChunkCoord,
    mesh: Handle<Mesh>,
}

fn apply_chunk_mesh_updates(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<TerrainMeshMaterial>,
    chunk_settings: Res<ChunkSettings>,
    terrain_settings: Res<TerrainSettings>,
    tiles: Res<TileRegistry>,
    chunks: Res<WorldChunks>,
    mut events: EventReader<TerrainChunkUpdated>,
    existing: Query<(Entity, &ChunkMeshEntity)>,
    hd2d: Option<Res<Hd2dSettings>>,
) {
    let chunk_dims_vec2 = chunk_settings.chunk_dimensions.as_vec2();
    let chunk_world_size = chunk_dims_vec2 * chunk_settings.tile_size;
    let default_depth_scale = hd2d
        .as_ref()
        .map(|settings| settings.depth_scale)
        .unwrap_or_else(|| Hd2dSettings::default().depth_scale);

    for event in events.read() {
        let Some(chunk) = chunks.loaded.get(&event.coord) else {
            continue;
        };
        let Some(layer) = chunk.layers.get(&terrain_settings.ground_layer) else {
            continue;
        };

        let mesh_data = build_chunk_mesh(layer, chunk_settings.tile_size, chunk_world_size, &tiles);

        if let Some((entity, marker)) = existing
            .iter()
            .find(|(_, marker)| marker.coord == event.coord)
        {
            if let Some(mesh) = meshes.get_mut(&marker.mesh) {
                *mesh = mesh_data;
            } else {
                let mesh_handle = meshes.add(mesh_data);
                commands.entity(entity).insert((
                    Mesh2dHandle(mesh_handle.clone()),
                    ChunkMeshEntity {
                        coord: marker.coord,
                        mesh: mesh_handle,
                    },
                ));
            }
        } else {
            let mesh_handle = meshes.add(mesh_data);
            let chunk_origin_tiles = event.coord.0.as_vec2() * chunk_world_size;
            let translation = Vec3::new(
                chunk_origin_tiles.x + chunk_world_size.x * 0.5,
                chunk_origin_tiles.y + chunk_world_size.y * 0.5,
                0.0,
            );

            commands.spawn((
                MaterialMesh2dBundle::<Hd2dLightingMaterial> {
                    mesh: Mesh2dHandle(mesh_handle.clone()),
                    material: material.base.clone(),
                    transform: Transform::from_translation(translation),
                    ..Default::default()
                },
                ChunkMeshEntity {
                    coord: event.coord,
                    mesh: mesh_handle,
                },
                BillboardSprite::default(),
                DepthSorted {
                    base_z: -50.0,
                    depth_scale: Some(default_depth_scale),
                },
                Name::new(format!(
                    "Terrain Chunk Mesh ({}, {})",
                    event.coord.0.x, event.coord.0.y
                )),
            ));
        }
    }
}

fn cleanup_unloaded_chunk_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    chunks: Res<WorldChunks>,
    query: Query<(Entity, &ChunkMeshEntity)>,
) {
    for (entity, marker) in &query {
        if !chunks.loaded.contains_key(&marker.coord) {
            meshes.remove(&marker.mesh);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn build_chunk_mesh(
    layer: &LayerData,
    tile_size: Vec2,
    chunk_world_size: Vec2,
    tiles: &TileRegistry,
) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let center = chunk_world_size * 0.5;
    let normal = [0.0, 0.0, 1.0];

    for y in 0..layer.size.y {
        for x in 0..layer.size.x {
            let idx = (y * layer.size.x + x) as usize;
            let tile_id = layer.tiles[idx];
            let (color, height) = tile_color(tile_id, tiles, tile_size.y * 0.02);

            let x0 = x as f32 * tile_size.x - center.x;
            let y0 = y as f32 * tile_size.y - center.y;
            let x1 = x0 + tile_size.x;
            let y1 = y0 + tile_size.y;
            let z = height;

            let base_index = positions.len() as u32;

            positions.extend_from_slice(&[[x0, y0, z], [x1, y0, z], [x1, y1, z], [x0, y1, z]]);
            normals.extend_from_slice(&[normal; 4]);
            uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);

            let rgba = color.to_linear().to_vec4();
            colors.extend_from_slice(&[[rgba.x, rgba.y, rgba.z, rgba.w]; 4]);

            indices.extend_from_slice(&[
                base_index,
                base_index + 1,
                base_index + 2,
                base_index,
                base_index + 2,
                base_index + 3,
            ]);
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn tile_color(tile_id: TileId, tiles: &TileRegistry, height_scale: f32) -> (Color, f32) {
    match tiles.get(tile_id) {
        Some(def) => {
            let color = if def.walkable {
                Color::srgb(0.38, 0.78, 0.42)
            } else {
                Color::srgb(0.85, 0.32, 0.32)
            };
            (color, def.height * height_scale)
        }
        None => (Color::srgb(0.25, 0.25, 0.35), 0.0),
    }
}
