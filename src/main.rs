#![allow(clippy::type_complexity)]
#![warn(clippy::pedantic)]
// Not crimes.
#![allow(clippy::wildcard_imports)]
#![allow(clippy::needless_pass_by_value)]
// Crimes that are hard to fix.
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]
// Unstable features:

use bevy::{
    a11y::AccessibilityPlugin,
    prelude::*,
    winit::{WakeUp, WinitPlugin},
};
use bevy_vulkano::VulkanoPlugin;
use procedural_macros::{nothing, vulkano_module};
use vulkano::{
    buffer::{allocator::SubbufferAllocator, BufferContents},
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    pipeline::graphics::vertex_input::Vertex,
};

fn main() {
    App::new()
        .add_plugins((
            AccessibilityPlugin,
            WindowPlugin::default(),
            WinitPlugin::<WakeUp>::default(),
            VulkanoPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("blah");
}

#[derive(BufferContents, Vertex, Clone)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}

mod render_system_runner {
    use std::{collections::HashMap, sync::Arc};

    use bevy::{ecs::system::SystemState, prelude::*};
    use bevy_vulkano::{BevyVulkanoContext, VulkanoRenderers};
    use vulkano::{
        buffer::allocator::SubbufferAllocator,
        command_buffer::{
            allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder,
            PrimaryAutoCommandBuffer,
        },
    };

    // fn render_start(world: &mut World, parameters: &mut SystemState<(VulkanoRenderers, Query<Entity, With<Window>>)>) {
    //     let (vulkano_windows, windows) = parameters.get_mut(world);

    //     let mut command_buffer_builders = HashMap::with_capacity(windows.iter().len());

    //     for window in &windows {
    //         let window = vulkano_windows.get_vulkano_window_mut(window).unwrap();

    //         let before = match primary_window.renderer.acquire() {
    //             Err(e) => {
    //                 bevy::log::error!("Failed to start frame: {}", e);
    //                 return;
    //             }
    //             Ok(f) => f,
    //         };

    //         command_buffer_builders.insert(window, ());
    //     }
    // }

    fn render_end(world: &mut World) {}

    // #[derive(Deref, DerefMut)]
    // pub struct RenderContext {
    //     #[deref]
    //     context: BevyVulkanoContext,
    //     command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    //     subbuffer_allocator: Option<SubbufferAllocator>,
    // }

    // impl RenderContext {
    //     pub fn command_buffer_allocator(&self) -> &Arc<StandardCommandBufferAllocator> {
    //         &self.command_buffer_allocator
    //     }
    // }
}

mod blah_expanded {
    use super::MyVertex;
    use bevy::prelude::*;
    use vulkano::{buffer::{allocator::SubbufferAllocator, Subbuffer}, DeviceSize};

    #[derive(Resource)]
    pub struct RenderData {
        pub vertices: Vec<MyVertex>,
    }

    pub fn main(subbuffer_allocator: NonSend<SubbufferAllocator>, render_data: Res<RenderData>) {
        subbuffer_allocator.reserve(render_data.vertices.len() as DeviceSize).unwrap();

        // For all often buffers we should store this stuff in a hashmap somehow.
        let render_data_vertices_buffer = 
            subbuffer_allocator
            .allocate_slice(render_data.vertices.len().try_into().unwrap())
            .unwrap();
        render_data_vertices_buffer
            .write()
            .unwrap()
            .clone_from_slice(&render_data.vertices);
    }
}

struct ExternalTest {}

nothing!(
    blah

    Data external_test {
        External,
    },

    Data render_data {
        Resource,

        Vertices Often vertices MyVertex,
    },

    System main {
        Vertices render_data.vertices,
    },
);

nothing!{
    ugh

    System main {
        Pipeline render_data.pipeline,
        Vertices render_data.vertices,
    }
}