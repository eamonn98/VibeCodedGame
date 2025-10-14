use bevy::prelude::*;

mod hd2d_pipeline;
mod lighting;
mod post_processing;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hd2d_pipeline::Hd2dPipelinePlugin,
            lighting::LightingPlugin,
            post_processing::PostProcessingPlugin,
        ));
    }
}
