use bevy::{
    core_pipeline::core_3d::graph::Node3d,
    core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
    prelude::*,
    render::render_graph::RenderLabel,
    render::{
        extract_component::ExtractComponent, render_graph::InternedRenderLabel,
        render_resource::ShaderType,
    },
    shader::ShaderRef,
};

#[derive(Component, ExtractComponent, Clone, Copy, ShaderType)]
pub struct K {
    pub time: f32,
    /// scales all effects
    pub intensity: f32,
    /// 0.0 - 0.3
    pub base_level: f32,
    /// 0.3 - 3.0
    pub peak_frequency: f32,
    /// length of peaks 2.0 - 12.0
    pub peak_sharpness: f32,
    /// maximum intensity 0.4 - 1.0
    pub peak_intensity: f32,
    /// 0.5 - 4.0
    pub wave_frequency: f32,
    /// 0.0 - 0.3
    pub wave_intensity: f32,
}

impl Default for K {
    fn default() -> Self {
        Self {
            time: 0.0,
            intensity: 1.0,
            base_level: 0.05,
            peak_frequency: 1.0,
            peak_sharpness: 6.0,
            peak_intensity: 0.85,
            wave_frequency: 1.5,
            wave_intensity: 0.1,
        }
    }
}

impl FullscreenMaterial for K {
    fn fragment_shader() -> ShaderRef {
        "k.wgsl".into()
    }

    fn node_edges() -> Vec<InternedRenderLabel> {
        vec![
            Node3d::Tonemapping.intern(),
            Self::node_label().intern(),
            Node3d::EndMainPassPostProcessing.intern(),
        ]
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FullscreenMaterialPlugin::<K>::default())
        .add_systems(Update, update_shader);
}

fn update_shader(time: Res<Time>, mut query: Query<&mut K>) {
    for mut effect in query.iter_mut() {
        effect.time = time.elapsed_secs();
    }
}
