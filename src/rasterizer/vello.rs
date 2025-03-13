use std::cell::RefCell;
use crate::painter::commands::PaintCommand;
use vello::peniko::Color;
use vello::{AaConfig, Renderer, Scene};
use vello::wgpu::{Device, Queue, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use crate::rasterizer::Rasterable;
use crate::common::texture::TextureId;
use crate::common::get_texture_store;
use crate::tiler::Tile;

mod rectangle;
mod brush;
mod text;

pub struct VelloRasterizer<'a> {
    device: &'a Device,
    queue: &'a Queue,
    renderer: &'a RefCell<Renderer>,
}

impl<'a> VelloRasterizer<'a> {
    pub fn new(device: &'a Device, queue: &'a Queue, renderer: &'a RefCell<Renderer>) -> Self {
        Self {
            device,
            queue,
            renderer,
        }
    }
}

impl Rasterable for VelloRasterizer<'_> {
    fn rasterize(&self, tile: &Tile) -> TextureId {
        let mut scene = Scene::new();

        let width = tile.rect.width as u32;
        let height = tile.rect.height as u32;

        for element in &tile.elements {
            for command in &element.paint_commands {
                match command {
                    PaintCommand::Rectangle(command) => {
                        rectangle::do_paint_rectangle(&mut scene, &tile, &command);
                    }
                    PaintCommand::Text(command) => {
                        println!("Text: {}", command.text);
                        // match text::do_paint_text(&mut scene, &tile, &command) {
                        //     Ok(_) => {}
                        //     Err(e) => {
                        //         println!("Failed to paint text: {:?}", e);
                        //     }
                        // }
                    }
                }
            }
        }

        let texture = create_offscreen_texture(&self.device, width, height);

        let render_params = vello::RenderParams {
            base_color: Color::new([0.0, 0.0, 0.0, 0.0]),   // Transparent texture
            width: tile.rect.width as u32,
            height: tile.rect.height as u32,
            antialiasing_method: AaConfig::Area,
        };

        self.renderer.borrow_mut().render_to_texture(
            &self.device,
            &self.queue,
            &scene,
            &texture.create_view(&Default::default()),
            &render_params,
        ).unwrap();

        let texture_data = read_texture_to_image(&self.device, &self.queue, &texture, width, height);

        let binding = get_texture_store();
        let mut texture_store = binding.write().expect("Failed to get texture store");
        let texture_id = texture_store.add(width as usize, height as usize, texture_data.to_vec());

        texture_id
    }
}

fn create_offscreen_texture(device: &Device, width: u32, height: u32) -> Texture {
    device.create_texture(&TextureDescriptor {
        label: Some("Tile texture"),
        size: vello::wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8Unorm,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC | TextureUsages::STORAGE_BINDING,
        view_formats: &[],
    })
}

fn read_texture_to_image(device: &Device, queue: &Queue, texture: &Texture, width: u32, height: u32) -> Vec<u8> {
    let buffer_size = (width * height * 4) as vello::wgpu::BufferAddress;
    let buffer = device.create_buffer(&vello::wgpu::BufferDescriptor {
        label: Some("Texture Read Buffer"),
        size: buffer_size,
        usage: vello::wgpu::BufferUsages::COPY_DST | vello::wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&vello::wgpu::CommandEncoderDescriptor {
        label: Some("Texture Copy Encoder"),
    });
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        vello::wgpu::ImageCopyBuffer {
            buffer: &buffer,
            layout: vello::wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
        },
        vello::wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(std::iter::once(encoder.finish()));

    // Map the buffer and read the data
    let buffer_slice = buffer.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    buffer_slice.map_async(vello::wgpu::MapMode::Read, move |result| {
        sender.send(result).unwrap();
    });
    device.poll(vello::wgpu::Maintain::Wait);
    receiver.recv().unwrap().unwrap();

    let data = buffer_slice.get_mapped_range();
    let result = data.to_vec();
    drop(data);
    buffer.unmap();
    result
}