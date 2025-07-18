// Copyright (c) 2019-present Dmitry Stepanov and Fyrox Engine contributors.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::{
    core::{algebra::Vector2, math::Rect, sstorage::ImmutableString},
    renderer::{
        cache::{
            shader::{binding, property, PropertyGroup, RenderMaterial},
            uniform::UniformBufferCache,
        },
        framework::{error::FrameworkError, framebuffer::GpuFrameBuffer, gpu_texture::GpuTexture},
        make_viewport_matrix,
        resources::RendererResources,
        RenderPassStatistics,
    },
};

#[derive(Default)]
pub struct FxaaRenderer {}

impl FxaaRenderer {
    pub(crate) fn render(
        &self,
        viewport: Rect<i32>,
        frame_texture: &GpuTexture,
        frame_buffer: &GpuFrameBuffer,
        uniform_buffer_cache: &mut UniformBufferCache,
        renderer_resources: &RendererResources,
    ) -> Result<RenderPassStatistics, FrameworkError> {
        let mut statistics = RenderPassStatistics::default();

        let frame_matrix = make_viewport_matrix(viewport);

        let inv_screen_size = Vector2::new(1.0 / viewport.w() as f32, 1.0 / viewport.h() as f32);
        let properties = PropertyGroup::from([
            property("worldViewProjection", &frame_matrix),
            property("inverseScreenSize", &inv_screen_size),
        ]);
        let material = RenderMaterial::from([
            binding(
                "screenTexture",
                (frame_texture, &renderer_resources.nearest_clamp_sampler),
            ),
            binding("properties", &properties),
        ]);

        statistics += renderer_resources.shaders.fxaa.run_pass(
            1,
            &ImmutableString::new("Primary"),
            frame_buffer,
            &renderer_resources.quad,
            viewport,
            &material,
            uniform_buffer_cache,
            Default::default(),
            None,
        )?;

        Ok(statistics)
    }
}
