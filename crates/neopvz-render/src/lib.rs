use std::{collections::HashMap, sync::Arc};

use bytemuck::{Pod, Zeroable};
use neopvz_core::{LOGICAL_HEIGHT, LOGICAL_WIDTH};
use thiserror::Error;
use wgpu::util::DeviceExt;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::Window,
};

pub const TITLE_IMAGE_ID: u32 = 1;
pub const SEED_CHOOSER_IMAGE_ID: u32 = 2;
pub const DAY_BACKGROUND_IMAGE_ID: u32 = 3;
pub const UI_PIXEL_IMAGE_ID: u32 = 4;
pub const SCREEN_PIXEL_IMAGE_ID: u32 = 5;
pub const TITLE_LOGO_IMAGE_ID: u32 = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LogicalViewport {
    pub width: u32,
    pub height: u32,
}

impl Default for LogicalViewport {
    fn default() -> Self {
        Self {
            width: LOGICAL_WIDTH,
            height: LOGICAL_HEIGHT,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ViewportRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn letterbox_rect(
    window_width: u32,
    window_height: u32,
    logical_viewport: LogicalViewport,
) -> ViewportRect {
    if window_width == 0
        || window_height == 0
        || logical_viewport.width == 0
        || logical_viewport.height == 0
    {
        return ViewportRect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
    }

    let scale = (window_width as f64 / logical_viewport.width as f64)
        .min(window_height as f64 / logical_viewport.height as f64);
    let width = (logical_viewport.width as f64 * scale).round() as u32;
    let height = (logical_viewport.height as f64 * scale).round() as u32;

    ViewportRect {
        x: (window_width - width) / 2,
        y: (window_height - height) / 2,
        width,
        height,
    }
}

pub fn logical_position(
    window_width: u32,
    window_height: u32,
    position: PhysicalPosition<f64>,
    logical_viewport: LogicalViewport,
) -> Option<(f32, f32)> {
    let rect = letterbox_rect(window_width, window_height, logical_viewport);
    if rect.width == 0
        || position.x < f64::from(rect.x)
        || position.y < f64::from(rect.y)
        || position.x >= f64::from(rect.x + rect.width)
        || position.y >= f64::from(rect.y + rect.height)
    {
        return None;
    }

    let scale = f64::from(rect.width) / f64::from(logical_viewport.width);
    Some((
        ((position.x - f64::from(rect.x)) / scale) as f32,
        ((position.y - f64::from(rect.y)) / scale) as f32,
    ))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageAsset {
    pub resource_id: u32,
    pub width: u32,
    pub height: u32,
    pub rgba8: Vec<u8>,
}

impl ImageAsset {
    pub fn new(
        resource_id: u32,
        width: u32,
        height: u32,
        rgba8: Vec<u8>,
    ) -> Result<Self, ImageAssetError> {
        let expected = usize::try_from(width)
            .ok()
            .and_then(|width| {
                usize::try_from(height)
                    .ok()
                    .and_then(|height| width.checked_mul(height))
            })
            .and_then(|pixels| pixels.checked_mul(4))
            .ok_or(ImageAssetError::InvalidDimensions { width, height })?;
        if width == 0 || height == 0 || rgba8.len() != expected {
            return Err(ImageAssetError::InvalidDataLength {
                width,
                height,
                expected,
                actual: rgba8.len(),
            });
        }

        Ok(Self {
            resource_id,
            width,
            height,
            rgba8,
        })
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ImageAssetError {
    #[error("image dimensions are too large: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },
    #[error("image data length mismatch for {width}x{height}: expected {expected}, got {actual}")]
    InvalidDataLength {
        width: u32,
        height: u32,
        expected: usize,
        actual: usize,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteCommand {
    pub resource_id: u32,
    pub x: f32,
    pub y: f32,
    pub z: i32,
    pub scale: f32,
    pub alpha: f32,
}

#[derive(Clone, Debug, Default)]
pub struct RenderFrame {
    pub sprites: Vec<SpriteCommand>,
}

impl RenderFrame {
    pub fn sort_for_submission(&mut self) {
        self.sprites.sort_by_key(|sprite| sprite.z);
    }
}

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("failed to create GPU surface: {0}")]
    SurfaceCreation(String),
    #[error("no compatible GPU adapter was found: {0}")]
    Adapter(String),
    #[error("failed to create GPU device: {0}")]
    Device(String),
    #[error("the GPU surface has no supported configuration")]
    NoSurfaceConfiguration,
    #[error("invalid image asset: {0}")]
    InvalidImage(#[from] ImageAssetError),
    #[error("sprite references unloaded image {0}")]
    MissingImage(u32),
    #[error("GPU surface validation failed while acquiring a frame")]
    SurfaceValidation,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct SpriteVertex {
    position: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

struct GpuImage {
    _texture: wgpu::Texture,
    _view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

#[derive(Clone, Copy, Debug)]
struct DrawCall {
    resource_id: u32,
    start: u32,
    end: u32,
}

pub struct GpuRenderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    logical_viewport: LogicalViewport,
    pipeline: wgpu::RenderPipeline,
    texture_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    images: HashMap<u32, GpuImage>,
}

impl GpuRenderer {
    pub async fn new(window: Arc<Window>) -> Result<Self, RendererError> {
        let instance_descriptor = wgpu::InstanceDescriptor::new_without_display_handle_from_env();
        #[cfg(target_os = "windows")]
        let instance_descriptor = if std::env::var_os("WGPU_BACKEND").is_none() {
            // Prefer Windows' native backend; WGPU_BACKEND remains an explicit override.
            wgpu::InstanceDescriptor {
                backends: wgpu::Backends::DX12 | wgpu::Backends::GL,
                ..instance_descriptor
            }
        } else {
            instance_descriptor
        };
        let instance = wgpu::Instance::new(instance_descriptor);
        let surface = instance
            .create_surface(window.clone())
            .map_err(|error| RendererError::SurfaceCreation(error.to_string()))?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .map_err(|error| RendererError::Adapter(error.to_string()))?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .map_err(|error| RendererError::Device(error.to_string()))?;

        let size = window.inner_size();
        let surface_width = size.width.max(1);
        let surface_height = size.height.max(1);
        let config = surface
            .get_default_config(&adapter, surface_width, surface_height)
            .ok_or(RendererError::NoSurfaceConfiguration)?;
        surface.configure(&device, &config);

        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sprite texture layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("sprite sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("sprite shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sprite pipeline layout"),
            bind_group_layouts: &[Some(&texture_layout)],
            immediate_size: 0,
        });
        let vertex_attributes = wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x2,
            2 => Float32x4,
        ];
        let vertex_buffers = [Some(wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SpriteVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &vertex_attributes,
        })];
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("sprite pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &vertex_buffers,
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        Ok(Self {
            instance,
            surface,
            window,
            device,
            queue,
            config,
            size,
            logical_viewport: LogicalViewport::default(),
            pipeline,
            texture_layout,
            sampler,
            images: HashMap::new(),
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn add_image(&mut self, asset: ImageAsset) -> Result<(), RendererError> {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("sprite texture"),
            size: wgpu::Extent3d {
                width: asset.width,
                height: asset.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &asset.rgba8,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(asset.width.checked_mul(4).ok_or(
                    RendererError::InvalidImage(ImageAssetError::InvalidDimensions {
                        width: asset.width,
                        height: asset.height,
                    }),
                )?),
                rows_per_image: Some(asset.height),
            },
            wgpu::Extent3d {
                width: asset.width,
                height: asset.height,
                depth_or_array_layers: 1,
            },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sprite bind group"),
            layout: &self.texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        self.images.insert(
            asset.resource_id,
            GpuImage {
                _texture: texture,
                _view: view,
                bind_group,
            },
        );
        Ok(())
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        if size.width == 0 || size.height == 0 {
            return;
        }
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self, frame: &RenderFrame) -> Result<(), RendererError> {
        if self.size.width == 0 || self.size.height == 0 {
            return Ok(());
        }

        let mut sorted = frame.clone();
        sorted.sort_for_submission();
        let (vertices, draw_calls) = self.build_batch(&sorted)?;
        let viewport = letterbox_rect(self.size.width, self.size.height, self.logical_viewport);
        if viewport.width == 0 || viewport.height == 0 {
            return Ok(());
        }

        let vertex_buffer = (!vertices.is_empty()).then(|| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("sprite vertex buffer"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                })
        });

        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                drop(texture);
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface = self
                    .instance
                    .create_surface(self.window.clone())
                    .map_err(|error| RendererError::SurfaceCreation(error.to_string()))?;
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err(RendererError::SurfaceValidation);
            }
        };
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("sprite command encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sprite render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_scissor_rect(viewport.x, viewport.y, viewport.width, viewport.height);
            if let Some(vertex_buffer) = &vertex_buffer {
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                for draw_call in draw_calls {
                    let image = self
                        .images
                        .get(&draw_call.resource_id)
                        .expect("batch image was checked before encoding");
                    render_pass.set_bind_group(0, &image.bind_group, &[]);
                    render_pass.draw(draw_call.start..draw_call.end, 0..1);
                }
            }
        }
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        self.queue.present(surface_texture);
        Ok(())
    }

    fn build_batch(
        &self,
        frame: &RenderFrame,
    ) -> Result<(Vec<SpriteVertex>, Vec<DrawCall>), RendererError> {
        let viewport = letterbox_rect(self.size.width, self.size.height, self.logical_viewport);
        let logical_scale = viewport.width as f32 / self.logical_viewport.width as f32;
        let mut vertices = Vec::with_capacity(frame.sprites.len() * 6);
        let mut draw_calls = Vec::with_capacity(frame.sprites.len());

        for sprite in &frame.sprites {
            let image = self
                .images
                .get(&sprite.resource_id)
                .ok_or(RendererError::MissingImage(sprite.resource_id))?;
            let x = viewport.x as f32 + sprite.x * logical_scale;
            let y = viewport.y as f32 + sprite.y * logical_scale;
            let width = image_width(image) * sprite.scale * logical_scale;
            let height = image_height(image) * sprite.scale * logical_scale;
            let x1 = x + width;
            let y1 = y + height;
            let color = [1.0, 1.0, 1.0, sprite.alpha.clamp(0.0, 1.0)];
            let start = u32::try_from(vertices.len()).unwrap_or(u32::MAX);
            vertices.extend([
                SpriteVertex::new(ndc(x, y, self.size), [0.0, 0.0], color),
                SpriteVertex::new(ndc(x1, y, self.size), [1.0, 0.0], color),
                SpriteVertex::new(ndc(x1, y1, self.size), [1.0, 1.0], color),
                SpriteVertex::new(ndc(x, y, self.size), [0.0, 0.0], color),
                SpriteVertex::new(ndc(x1, y1, self.size), [1.0, 1.0], color),
                SpriteVertex::new(ndc(x, y1, self.size), [0.0, 1.0], color),
            ]);
            let end = u32::try_from(vertices.len()).unwrap_or(u32::MAX);
            draw_calls.push(DrawCall {
                resource_id: sprite.resource_id,
                start,
                end,
            });
        }

        Ok((vertices, draw_calls))
    }
}

fn image_width(image: &GpuImage) -> f32 {
    image._texture.width() as f32
}

fn image_height(image: &GpuImage) -> f32 {
    image._texture.height() as f32
}

fn ndc(x: f32, y: f32, size: PhysicalSize<u32>) -> [f32; 2] {
    [
        x / size.width as f32 * 2.0 - 1.0,
        1.0 - y / size.height as f32 * 2.0,
    ]
}

impl SpriteVertex {
    fn new(position: [f32; 2], uv: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            position,
            uv,
            color,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letterbox_rect_preserves_the_logical_aspect_ratio() {
        assert_eq!(
            letterbox_rect(1280, 720, LogicalViewport::default()),
            ViewportRect {
                x: 160,
                y: 0,
                width: 960,
                height: 720,
            }
        );
        assert_eq!(
            letterbox_rect(800, 600, LogicalViewport::default()),
            ViewportRect {
                x: 0,
                y: 0,
                width: 800,
                height: 600,
            }
        );
    }

    #[test]
    fn logical_position_ignores_letterbox_borders() {
        assert_eq!(
            logical_position(
                1280,
                720,
                PhysicalPosition::new(10.0, 10.0),
                LogicalViewport::default(),
            ),
            None
        );
        assert_eq!(
            logical_position(
                1280,
                720,
                PhysicalPosition::new(160.0, 0.0),
                LogicalViewport::default(),
            ),
            Some((0.0, 0.0))
        );
    }

    #[test]
    fn render_frame_sort_keeps_equal_depth_order() {
        let mut frame = RenderFrame {
            sprites: vec![
                SpriteCommand {
                    resource_id: 1,
                    x: 0.0,
                    y: 0.0,
                    z: 2,
                    scale: 1.0,
                    alpha: 1.0,
                },
                SpriteCommand {
                    resource_id: 2,
                    x: 0.0,
                    y: 0.0,
                    z: 1,
                    scale: 1.0,
                    alpha: 1.0,
                },
                SpriteCommand {
                    resource_id: 3,
                    x: 0.0,
                    y: 0.0,
                    z: 2,
                    scale: 1.0,
                    alpha: 1.0,
                },
            ],
        };

        frame.sort_for_submission();

        assert_eq!(
            frame
                .sprites
                .iter()
                .map(|sprite| sprite.resource_id)
                .collect::<Vec<_>>(),
            vec![2, 1, 3]
        );
    }

    #[test]
    fn image_asset_rejects_mismatched_rgba_data() {
        assert!(matches!(
            ImageAsset::new(1, 2, 2, vec![0; 3]),
            Err(ImageAssetError::InvalidDataLength {
                width: 2,
                height: 2,
                expected: 16,
                actual: 3,
            })
        ));
    }
}
