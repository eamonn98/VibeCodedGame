use bevy::prelude::*;

use crate::core::InputState;
use crate::player::PlayerEntity;
use crate::systems::MovementState;
use crate::world::debug::{ChunkDebugStats, DebugTerrainSettings};
use crate::world::{ChunkSettings, TerrainSettings, TileRegistry, WorldChunks};

#[derive(Component)]
struct DebugOverlayRoot;

#[derive(Component)]
struct CollisionStatusText;

#[derive(Component)]
struct DebugToggleText;

#[derive(Component)]
struct TileInfoText;

#[derive(Component)]
struct ChunkStatsText;

pub struct DebugOverlayPlugin;

impl Plugin for DebugOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_debug_overlay).add_systems(
            Update,
            (
                update_collision_status,
                update_debug_toggle_text,
                update_tile_info_text,
                update_chunk_stats_text,
            ),
        );
    }
}

fn update_chunk_stats_text(
    mut query: Query<&mut Text, With<ChunkStatsText>>,
    stats: Res<ChunkDebugStats>,
) {
    let Ok(mut text) = query.get_single_mut() else {
        return;
    };

    let section = &mut text.sections[1];
    section.value = format!(
        "frame g:{} u:{} r:{} un:{} | total g:{} un:{} loaded:{} toggles:{}",
        stats.frame_generated,
        stats.frame_updated,
        stats.frame_reused,
        stats.frame_unloaded,
        stats.total_generated,
        stats.total_unloaded,
        stats.total_loaded,
        stats.overlay_toggles,
    );
}

fn setup_debug_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(Val::Px(16.0)),
                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                ..Default::default()
            },
            DebugOverlayRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Collision: ",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                        },
                    ),
                    TextSection::new(
                        "clear",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::srgba(0.3, 0.9, 0.3, 1.0),
                        },
                    ),
                ]),
                CollisionStatusText,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Tiles Overlay: ",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                        },
                    ),
                    TextSection::new(
                        "visible",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::srgba(0.3, 0.9, 0.3, 1.0),
                        },
                    ),
                    TextSection::new(
                        " (press V)",
                        TextStyle {
                            font: font.clone(),
                            font_size: 14.0,
                            color: Color::srgba(1.0, 1.0, 1.0, 0.7),
                        },
                    ),
                ]),
                DebugToggleText,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Tile: ",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                        },
                    ),
                    TextSection::new(
                        "unknown",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                        },
                    ),
                ]),
                TileInfoText,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Chunks: ",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                        },
                    ),
                    TextSection::new(
                        "frame g:0 u:0 r:0 un:0 | total g:0 un:0 loaded:0 toggles:0",
                        TextStyle {
                            font,
                            font_size: 18.0,
                            color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                        },
                    ),
                ]),
                ChunkStatsText,
            ));
        });
}

fn update_collision_status(
    mut query: Query<&mut Text, With<CollisionStatusText>>,
    movement_query: Query<&MovementState>,
) {
    let Ok(mut text) = query.get_single_mut() else {
        return;
    };

    let blocked = movement_query.iter().any(|state| state.blocked);

    let section = &mut text.sections[1];
    if blocked {
        section.value = "blocked".into();
        section.style.color = Color::srgba(0.9, 0.3, 0.3, 1.0);
    } else {
        section.value = "clear".into();
        section.style.color = Color::srgba(0.3, 0.9, 0.3, 1.0);
    }
}

fn update_debug_toggle_text(
    mut query: Query<&mut Text, With<DebugToggleText>>,
    input: Res<InputState>,
    mut debug_settings: ResMut<DebugTerrainSettings>,
    mut stats: ResMut<ChunkDebugStats>,
) {
    let Ok(mut text) = query.get_single_mut() else {
        return;
    };

    if input.toggle_debug_tiles {
        debug_settings.show_tiles = !debug_settings.show_tiles;
        stats.overlay_toggles = stats.overlay_toggles.saturating_add(1);
    }

    let visible_section = &mut text.sections[1];
    if debug_settings.show_tiles {
        visible_section.value = "visible".into();
        visible_section.style.color = Color::srgba(0.3, 0.9, 0.3, 1.0);
    } else {
        visible_section.value = "hidden".into();
        visible_section.style.color = Color::srgba(0.6, 0.6, 0.6, 1.0);
    }
}

fn update_tile_info_text(
    mut query: Query<&mut Text, With<TileInfoText>>,
    player_query: Query<&Transform, With<PlayerEntity>>,
    chunks: Res<WorldChunks>,
    chunk_settings: Res<ChunkSettings>,
    terrain_settings: Res<TerrainSettings>,
    tiles: Res<TileRegistry>,
) {
    let Ok(mut text) = query.get_single_mut() else {
        return;
    };

    let Ok(player_transform) = player_query.get_single() else {
        text.sections[1].value = "no player".into();
        text.sections[1].style.color = Color::srgba(0.7, 0.7, 0.7, 1.0);
        return;
    };

    let world_pos = player_transform.translation.truncate();
    if let Some(tile_id) =
        chunks.tile_at_world(&chunk_settings, terrain_settings.ground_layer, world_pos)
    {
        if let Some(def) = tiles.get(tile_id) {
            text.sections[1].value = def.name.clone();
            text.sections[1].style.color = if def.walkable {
                Color::srgba(0.3, 0.9, 0.3, 1.0)
            } else {
                Color::srgba(0.9, 0.3, 0.3, 1.0)
            };
        } else {
            text.sections[1].value = format!("Tile {:?}", tile_id);
            text.sections[1].style.color = Color::srgba(0.6, 0.6, 0.9, 1.0);
        }
    } else {
        text.sections[1].value = "void".into();
        text.sections[1].style.color = Color::srgba(0.7, 0.7, 0.7, 1.0);
    }
}
