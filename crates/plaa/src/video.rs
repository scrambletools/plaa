//! The picture: a shader widget that asks the player for the picture due
//! on every frame it draws, uploads its planes to the graphics card once,
//! and converts them to the screen's colors there.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use iced::advanced::graphics::Viewport;
use iced::wgpu;
use iced::widget::shader::{self, Action};
use iced::{Element, Fill, Rectangle};
use iced::{keyboard, mouse};
use plaa_media::frame::{Layout, Matrix, Transfer};
use plaa_media::{Player, SubtitlePicture, Transform, VideoFrame};

use crate::window::Activity;

/// How much a step of the wheel zooms in.
const ZOOM_STEP: f32 = 1.25;
/// How far the picture zooms in at most.
const MOST_ZOOM: f32 = 16.0;

/// How far the picture is zoomed in, and where: the point of it shown in
/// the middle of the widget, from the picture's own middle, in fractions
/// of its width and height, rightward and downward.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zoom {
    pub scale: f32,
    pub center: (f32, f32),
}

impl Default for Zoom {
    fn default() -> Self {
        Self {
            scale: 1.0,
            center: (0.0, 0.0),
        }
    }
}

/// A change to the zoom, for a picture fitted to `extent` of the widget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoomChange {
    /// The wheel turned `steps`, out where negative, at `pointer`, in
    /// fractions of the widget from its middle.
    Wheel {
        steps: f32,
        pointer: (f32, f32),
        extent: (f32, f32),
    },
    /// The zoomed picture was dragged `moved`, in fractions of the widget.
    Drag {
        moved: (f32, f32),
        extent: (f32, f32),
    },
}

impl Zoom {
    /// Changed by `change`.
    pub fn changed(self, change: ZoomChange) -> Self {
        match change {
            ZoomChange::Wheel {
                steps,
                pointer,
                extent,
            } => self.at(steps, pointer, extent),
            ZoomChange::Drag { moved, extent } => self.dragged(moved, extent),
        }
    }

    pub fn is_zoomed(self) -> bool {
        self.scale > 1.0
    }

    /// Dragged `moved`, in fractions of the widget: the picture follows
    /// the pointer, as far as it still covers the widget.
    fn dragged(self, moved: (f32, f32), extent: (f32, f32)) -> Self {
        let follow = |center: f32, moved: f32, extent: f32| center - moved / (extent * self.scale);
        Self {
            scale: self.scale,
            center: (
                follow(self.center.0, moved.0, extent.0),
                follow(self.center.1, moved.1, extent.1),
            ),
        }
        .clamped(extent)
    }

    /// Zoomed in `steps` steps of the wheel, out where negative, keeping
    /// the point of the picture at `pointer` where it is: `pointer` in
    /// fractions of the widget from its middle, for a picture fitted to
    /// `extent` of it.
    fn at(self, steps: f32, pointer: (f32, f32), extent: (f32, f32)) -> Self {
        let scale = (self.scale * ZOOM_STEP.powf(steps)).clamp(1.0, MOST_ZOOM);
        // The picture's point under the pointer, before and after.
        let under = |center: f32, pointer: f32, extent: f32, scale: f32| {
            center + pointer / (extent * scale)
        };
        let keep = |center: f32, pointer: f32, extent: f32| {
            under(center, pointer, extent, self.scale) - pointer / (extent * scale)
        };
        Self {
            scale,
            center: (
                keep(self.center.0, pointer.0, extent.0),
                keep(self.center.1, pointer.1, extent.1),
            ),
        }
        .clamped(extent)
    }

    /// Kept so the picture covers the widget where it is larger than it,
    /// and in its middle where it is not.
    fn clamped(self, extent: (f32, f32)) -> Self {
        let clamp = |center: f32, extent: f32| {
            // How wide the picture shows, in widgets.
            let shown = extent * self.scale;
            if shown <= 1.0 {
                0.0
            } else {
                let most = 0.5 - 0.5 / shown;
                center.clamp(-most, most)
            }
        };
        Self {
            scale: self.scale,
            center: (
                clamp(self.center.0, extent.0),
                clamp(self.center.1, extent.1),
            ),
        }
    }

    /// Half the picture's size, and where its middle is, in clip space,
    /// for a picture fitted to `extent`.
    fn placed(self, extent: (f32, f32)) -> ((f32, f32), (f32, f32)) {
        let zoom = self.clamped(extent);
        let size = (extent.0 * zoom.scale, extent.1 * zoom.scale);
        // Clip space is two units across and points up.
        let middle = (-2.0 * zoom.center.0 * size.0, 2.0 * zoom.center.1 * size.1);
        (size, middle)
    }
}

/// Draws `player`'s pictures, fitted inside the widget, turned and
/// flipped as `transform` says, and zoomed in as `zoom` says; the command
/// key and the wheel zoom it at the pointer, and dragging moves it while
/// zoomed, each change sent as `on_zoom` for the window to apply in turn.
/// `surface`
/// tells windows apart, as every window's pictures share one pipeline.
/// Pointer moves are noted in `activity`, and `on_activity` is sent when
/// one should bring the hidden overlay back.
pub fn view<'a, Message: Clone + 'a>(
    player: &'a Player,
    surface: u64,
    (transform, zoom): (Transform, Zoom),
    activity: Arc<Activity>,
    hide_cursor: bool,
    on_activity: Message,
    on_zoom: fn(ZoomChange) -> Message,
) -> Element<'a, Message> {
    shader::Shader::new(Program {
        player,
        surface,
        transform,
        zoom,
        activity,
        hide_cursor,
        on_activity,
        on_zoom,
    })
    .width(Fill)
    .height(Fill)
    .into()
}

struct Program<'a, Message> {
    player: &'a Player,
    surface: u64,
    transform: Transform,
    zoom: Zoom,
    activity: Arc<Activity>,
    hide_cursor: bool,
    on_activity: Message,
    on_zoom: fn(ZoomChange) -> Message,
}

/// What the picture keeps between events.
#[derive(Default)]
pub struct Held {
    /// The keys held, as the wheel zooms with the command key.
    modifiers: keyboard::Modifiers,
    /// Where the pointer was as it last dragged the zoomed picture.
    dragging: Option<iced::Point>,
}

impl<Message: Clone> shader::Program<Message> for Program<'_, Message> {
    type State = Held;
    type Primitive = Primitive;

    fn update(
        &self,
        held: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        // Each drawn frame asks for the next at the time its picture is
        // due, so playback runs without the interface being rebuilt.
        match event {
            iced::Event::Window(iced::window::Event::RedrawRequested(_)) => {
                self.player.next_redraw().map(Action::request_redraw_at)
            }
            iced::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let shown = self.activity.touch();
                let Some(from) = held.dragging else {
                    return shown.then(|| Action::publish(self.on_activity.clone()));
                };
                // Dragging keeps the overlay away, and moves the picture.
                held.dragging = Some(*position);
                let frame = self.player.picture()?;
                let moved = (
                    (position.x - from.x) / bounds.width.max(1.0),
                    (position.y - from.y) / bounds.height.max(1.0),
                );
                Some(Action::publish((self.on_zoom)(ZoomChange::Drag {
                    moved,
                    extent: fitted_extent(&frame, self.transform, &bounds),
                })))
            }
            // Not taken, so a double click still fills the screen.
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                if self.zoom.is_zoomed() =>
            {
                held.dragging = cursor.position_over(bounds);
                None
            }
            iced::Event::Mouse(
                mouse::Event::ButtonReleased(mouse::Button::Left) | mouse::Event::CursorLeft,
            ) => {
                held.dragging = None;
                None
            }
            iced::Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                held.modifiers = *modifiers;
                None
            }
            iced::Event::Mouse(mouse::Event::WheelScrolled { delta })
                if held.modifiers.command() =>
            {
                let pointer = cursor.position_in(bounds)?;
                let frame = self.player.picture()?;
                let steps = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => *y,
                    mouse::ScrollDelta::Pixels { y, .. } => y / 40.0,
                };
                let pointer = (
                    pointer.x / bounds.width.max(1.0) - 0.5,
                    pointer.y / bounds.height.max(1.0) - 0.5,
                );
                let change = ZoomChange::Wheel {
                    steps,
                    pointer,
                    extent: fitted_extent(&frame, self.transform, &bounds),
                };
                // Taken here, so the wheel does not change the volume too.
                Some(Action::publish((self.on_zoom)(change)).and_capture())
            }
            _ => None,
        }
    }

    fn draw(&self, _held: &Self::State, _cursor: mouse::Cursor, _bounds: Rectangle) -> Primitive {
        Primitive {
            surface: self.surface,
            frame: self.player.picture(),
            subtitle: self.player.subtitle_picture(),
            transform: self.transform,
            zoom: self.zoom,
        }
    }

    fn mouse_interaction(
        &self,
        held: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if held.dragging.is_some() {
            mouse::Interaction::Grabbing
        } else if !cursor.is_over(bounds) {
            mouse::Interaction::default()
        } else if self.hide_cursor {
            mouse::Interaction::Hidden
        } else if self.zoom.is_zoomed() {
            // A zoomed picture can be dragged.
            mouse::Interaction::Grab
        } else {
            mouse::Interaction::default()
        }
    }
}

#[derive(Debug)]
pub struct Primitive {
    surface: u64,
    frame: Option<Arc<VideoFrame>>,
    /// The picture subtitle showing over it.
    subtitle: Option<Arc<SubtitlePicture>>,
    /// How an edit turns and flips the picture.
    transform: Transform,
    zoom: Zoom,
}

/// The size of the shader's `Uniforms`: 36 words.
const UNIFORMS: usize = 144;

/// The values the shader reads, laid out as its `Uniforms`.
fn uniforms(
    frame: &VideoFrame,
    transform: Transform,
    (extent, middle): ((f32, f32), (f32, f32)),
    srgb_target: bool,
    subtitle: Option<&SubtitlePicture>,
) -> [u8; UNIFORMS] {
    let colors = frame.colors;
    let (kr, kb) = match colors.matrix {
        Matrix::Bt601 => (0.299, 0.114),
        Matrix::Bt709 => (0.2126, 0.0722),
        Matrix::Bt2020 => (0.2627, 0.0593),
    };
    let kg = 1.0 - kr - kb;
    let rows = [
        [1.0, 0.0, 2.0 - 2.0 * kr],
        [
            1.0,
            -2.0 * kb * (1.0 - kb) / kg,
            -2.0 * kr * (1.0 - kr) / kg,
        ],
        [1.0, 2.0 - 2.0 * kb, 0.0],
    ];
    // Where black, white and neutral chroma sit for the bit depth.
    let largest = ((1u64 << colors.bits) - 1) as f32;
    let step = (1u64 << colors.bits.saturating_sub(8)) as f32;
    let (luma_offset, luma_scale, chroma_offset, chroma_scale) = if colors.full_range {
        (0.0, 1.0, (largest + 1.0) / 2.0 / largest, 1.0)
    } else {
        (
            16.0 * step / largest,
            largest / (219.0 * step),
            128.0 * step / largest,
            largest / (224.0 * step),
        )
    };
    let (layout, wide) = match frame.layout {
        Layout::Planar8 => (0, 0),
        Layout::Planar16 => (0, 1),
        Layout::SemiPlanar8 => (1, 0),
        Layout::SemiPlanar16 => (1, 1),
        Layout::Rgba => (2, 0),
    };
    let transfer = match colors.transfer {
        Transfer::Standard => 0,
        Transfer::Pq => 1,
        Transfer::Hlg => 2,
    };
    let mut words = [0u32; UNIFORMS / 4];
    for (row, values) in rows.iter().enumerate() {
        for (column, value) in values.iter().enumerate() {
            words[row * 4 + column] = (*value as f32).to_bits();
        }
    }
    words[12] = extent.0.to_bits();
    words[13] = extent.1.to_bits();
    words[14] = f32::to_bits(luma_offset);
    words[15] = f32::to_bits(luma_scale);
    words[16] = f32::to_bits(chroma_offset);
    words[17] = f32::to_bits(chroma_scale);
    // RGBA is read as normalized bytes already.
    words[18] = if frame.layout == Layout::Rgba {
        (1.0f32 / 255.0).to_bits()
    } else {
        colors.scale.to_bits()
    };
    let shown = transform.after_turns(frame.quarter_turns);
    words[19] = u32::from(shown.turns);
    words[20] = layout;
    words[21] = wide;
    words[22] = transfer;
    words[23] = u32::from(colors.wide_gamut);
    words[24] = u32::from(srgb_target);
    words[26] = u32::from(shown.mirrored);
    if let Some(subtitle) = subtitle.filter(|subtitle| {
        subtitle.canvas.0 > 0 && subtitle.canvas.1 > 0 && subtitle.width > 0 && subtitle.height > 0
    }) {
        let (width, height) = (subtitle.canvas.0 as f32, subtitle.canvas.1 as f32);
        words[25] = 1;
        // Placed on the screen as without the edit, so they read upright,
        // unless the edit turns the picture on its side.
        words[27] = if transform.turns.is_multiple_of(2) {
            u32::from(frame.quarter_turns % 4)
        } else {
            4
        };
        words[28] = (subtitle.x as f32 / width).to_bits();
        words[29] = (subtitle.y as f32 / height).to_bits();
        words[30] = (subtitle.width as f32 / width).to_bits();
        words[31] = (subtitle.height as f32 / height).to_bits();
    }
    words[32] = middle.0.to_bits();
    words[33] = middle.1.to_bits();
    let mut bytes = [0u8; UNIFORMS];
    for (chunk, word) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(words) {
        *chunk = word.to_ne_bytes();
    }
    bytes
}

/// Half the fitted picture's size in clip space: the widget's whole width
/// or height, keeping the picture's shape as it shows.
fn fitted_extent(frame: &VideoFrame, transform: Transform, bounds: &Rectangle) -> (f32, f32) {
    let mut aspect = if frame.aspect.is_finite() && frame.aspect > 0.0 {
        frame.aspect as f32
    } else {
        frame.width as f32 / frame.height.max(1) as f32
    };
    if transform.after_turns(frame.quarter_turns).turns % 2 == 1 {
        aspect = 1.0 / aspect;
    }
    if bounds.width <= 0.0 || bounds.height <= 0.0 {
        return (1.0, 1.0);
    }
    let space = bounds.width / bounds.height;
    if space > aspect {
        (aspect / space, 1.0)
    } else {
        (1.0, space / aspect)
    }
}

impl shader::Primitive for Primitive {
    type Pipeline = Pipeline;

    fn prepare(
        &self,
        pipeline: &mut Pipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: &Rectangle,
        _viewport: &Viewport,
    ) {
        let Some(frame) = &self.frame else {
            pipeline.surfaces.remove(&self.surface);
            return;
        };
        let placed = self
            .zoom
            .placed(fitted_extent(frame, self.transform, bounds));
        let srgb = pipeline.srgb;
        let surface = pipeline
            .surfaces
            .entry(self.surface)
            .or_insert_with(|| Surface::new(device));
        surface.used = Instant::now();
        if surface.frame != frame.id {
            surface.upload(
                device,
                queue,
                frame,
                &pipeline.layout,
                &pipeline.sampler,
                &pipeline.blank,
            );
        }
        let subtitle = self.subtitle.as_deref();
        surface.show_subtitle(device, queue, subtitle, &pipeline.subtitle_layout);
        queue.write_buffer(
            &surface.uniforms,
            0,
            &uniforms(frame, self.transform, placed, srgb, subtitle),
        );
    }

    fn draw(&self, pipeline: &Pipeline, render_pass: &mut wgpu::RenderPass<'_>) -> bool {
        let Some(surface) = pipeline.surfaces.get(&self.surface) else {
            return true;
        };
        let (Some(_), Some(bind_group)) = (&self.frame, &surface.bind_group) else {
            return true;
        };
        render_pass.set_pipeline(&pipeline.pipeline);
        render_pass.set_bind_group(0, bind_group, &[]);
        let subtitle = surface
            .subtitle
            .as_ref()
            .filter(|_| self.subtitle.is_some())
            .map_or(&pipeline.no_subtitle, |subtitle| &subtitle.group);
        render_pass.set_bind_group(1, subtitle, &[]);
        render_pass.draw(0..4, 0..1);
        true
    }
}

/// The planes of the picture a window shows, on the graphics card.
struct Surface {
    textures: Vec<(wgpu::Texture, (u32, u32), wgpu::TextureFormat)>,
    uniforms: wgpu::Buffer,
    bind_group: Option<wgpu::BindGroup>,
    /// Where pictures are written on their way to the textures.
    staging: Vec<Staging>,
    /// The subtitle picture last shown over them.
    subtitle: Option<SubtitleTexture>,
    /// The picture uploaded, by its id.
    frame: u64,
    used: Instant,
}

/// A subtitle picture on the graphics card, by its id.
struct SubtitleTexture {
    id: u64,
    texture: wgpu::Texture,
    group: wgpu::BindGroup,
}

/// The subtitle layout's group for `view`.
fn subtitle_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    view: &wgpu::TextureView,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("plaa video subtitle"),
        layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(view),
        }],
    })
}

/// A buffer pictures are written into and the card copies from, kept and
/// reused: wgpu's own staging is fresh memory for every write, which at 8K
/// costs several times what the copy does.
struct Staging {
    buffer: wgpu::Buffer,
    /// Mapped, so it can be written; cleared while the card copies from it.
    ready: Arc<AtomicBool>,
}

/// How many staging buffers a window keeps: one being copied from while
/// the next picture is written into the other.
const STAGING_BUFFERS: usize = 2;

/// Where each plane goes in a staging buffer: its offset and row length,
/// rows padded as copies to textures need. Returns them and the size.
fn staging_layout(planes: &[((u32, u32), wgpu::TextureFormat)]) -> (Vec<(u64, u32)>, u64) {
    let align = u64::from(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
    let mut total = 0;
    let layout = planes
        .iter()
        .map(|((width, height), format)| {
            let row = u64::from(*width) * u64::from(format.block_copy_size(None).unwrap_or(1));
            let row = row.next_multiple_of(align);
            let offset = total;
            total += (row * u64::from(*height)).next_multiple_of(align);
            (offset, row as u32)
        })
        .collect();
    (layout, total.max(align))
}

impl Surface {
    fn new(device: &wgpu::Device) -> Self {
        Self {
            textures: Vec::new(),
            uniforms: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("plaa video uniforms"),
                size: UNIFORMS as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            bind_group: None,
            staging: Vec::new(),
            subtitle: None,
            frame: 0,
            used: Instant::now(),
        }
    }

    /// Puts `subtitle` on the card, once: premultiplied, so filtering it
    /// blends no dark edges in. Keeps the texture while the size stays.
    fn show_subtitle(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        subtitle: Option<&SubtitlePicture>,
        layout: &wgpu::BindGroupLayout,
    ) {
        let Some(subtitle) = subtitle.filter(|subtitle| subtitle.width > 0 && subtitle.height > 0)
        else {
            return;
        };
        if self
            .subtitle
            .as_ref()
            .is_some_and(|shown| shown.id == subtitle.id)
        {
            return;
        }
        let size = wgpu::Extent3d {
            width: subtitle.width,
            height: subtitle.height,
            depth_or_array_layers: 1,
        };
        let reusable = self
            .subtitle
            .take()
            .filter(|shown| shown.texture.size() == size);
        let (texture, group) = match reusable {
            Some(shown) => (shown.texture, shown.group),
            None => {
                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("plaa video subtitle"),
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                });
                let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                let group = subtitle_group(device, layout, &view);
                (texture, group)
            }
        };
        let premultiplied: Vec<u8> = subtitle
            .rgba
            .as_chunks::<4>()
            .0
            .iter()
            .flat_map(|[red, green, blue, alpha]| {
                let times = |value: u8| ((u16::from(value) * u16::from(*alpha) + 127) / 255) as u8;
                [times(*red), times(*green), times(*blue), *alpha]
            })
            .collect();
        queue.write_texture(
            texture.as_image_copy(),
            &premultiplied,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(subtitle.width * 4),
                rows_per_image: Some(subtitle.height),
            },
            size,
        );
        self.subtitle = Some(SubtitleTexture {
            id: subtitle.id,
            texture,
            group,
        });
    }

    fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        frame: &VideoFrame,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        blank: &wgpu::TextureView,
    ) {
        let formats = plane_formats(frame.layout);
        let wanted: Vec<((u32, u32), wgpu::TextureFormat)> = formats
            .iter()
            .enumerate()
            .map(|(index, format)| (frame.plane_size(index), *format))
            .collect();
        let same = self.textures.len() == wanted.len()
            && self.textures.iter().zip(&wanted).all(
                |((_, size, format), (wanted_size, wanted_format))| {
                    size == wanted_size && format == wanted_format
                },
            );
        if !same {
            self.textures = wanted
                .iter()
                .map(|((width, height), format)| {
                    let texture = device.create_texture(&wgpu::TextureDescriptor {
                        label: Some("plaa video plane"),
                        size: wgpu::Extent3d {
                            width: (*width).max(1),
                            height: (*height).max(1),
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: *format,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                        view_formats: &[],
                    });
                    (texture, (*width, *height), *format)
                })
                .collect();
            let views: Vec<wgpu::TextureView> = self
                .textures
                .iter()
                .map(|(texture, _, _)| texture.create_view(&wgpu::TextureViewDescriptor::default()))
                .collect();
            let view = |index: usize| views.get(index).unwrap_or(blank);
            self.bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("plaa video planes"),
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.uniforms.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(view(0)),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(view(1)),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::TextureView(view(2)),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                ],
            }));
        }
        let planes: Vec<_> = self
            .textures
            .iter()
            .map(|(_, size, format)| (*size, *format))
            .collect();
        let (offsets, size) = staging_layout(&planes);
        self.staging.retain(|staging| staging.buffer.size() == size);
        if !self
            .staging
            .iter()
            .any(|staging| staging.ready.load(Ordering::Acquire))
            && self.staging.len() < STAGING_BUFFERS
        {
            self.staging.push(Staging {
                buffer: device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("plaa video staging"),
                    size,
                    usage: wgpu::BufferUsages::MAP_WRITE | wgpu::BufferUsages::COPY_SRC,
                    mapped_at_creation: true,
                }),
                ready: Arc::new(AtomicBool::new(true)),
            });
        }
        let Some(staging) = self
            .staging
            .iter()
            .find(|staging| staging.ready.load(Ordering::Acquire))
        else {
            // Both still being copied from: wgpu stages this one itself.
            for (index, (texture, (width, height), format)) in self.textures.iter().enumerate() {
                let (data, stride) = frame.plane(index);
                if !plane_fits(data, stride, *width, *height, *format) {
                    continue;
                }
                queue.write_texture(
                    texture.as_image_copy(),
                    data,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(stride as u32),
                        rows_per_image: Some(*height),
                    },
                    extent(*width, *height),
                );
            }
            self.frame = frame.id;
            return;
        };
        {
            let mut mapped = staging.buffer.slice(..).get_mapped_range_mut();
            for (index, ((width, height), format)) in planes.iter().enumerate() {
                let (data, stride) = frame.plane(index);
                if !plane_fits(data, stride, *width, *height, *format) {
                    continue;
                }
                let (offset, padded) = offsets[index];
                let row = *width as usize * format.block_copy_size(None).unwrap_or(1) as usize;
                let rows = *height as usize;
                let target = &mut mapped[offset as usize..][..padded as usize * rows];
                if stride == padded as usize && data.len() >= target.len() {
                    target.copy_from_slice(&data[..padded as usize * rows]);
                } else {
                    for (to, from) in target
                        .chunks_exact_mut(padded as usize)
                        .zip(data.chunks(stride))
                    {
                        to[..row].copy_from_slice(&from[..row]);
                    }
                }
            }
        }
        staging.buffer.unmap();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("plaa video upload"),
        });
        for (index, (texture, (width, height), format)) in self.textures.iter().enumerate() {
            let (data, stride) = frame.plane(index);
            if !plane_fits(data, stride, *width, *height, *format) {
                continue;
            }
            let (offset, padded) = offsets[index];
            encoder.copy_buffer_to_texture(
                wgpu::TexelCopyBufferInfo {
                    buffer: &staging.buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset,
                        bytes_per_row: Some(padded),
                        rows_per_image: Some(*height),
                    },
                },
                texture.as_image_copy(),
                extent(*width, *height),
            );
        }
        queue.submit([encoder.finish()]);
        // Writable again once the card has copied from it.
        staging.ready.store(false, Ordering::Release);
        let ready = staging.ready.clone();
        staging
            .buffer
            .slice(..)
            .map_async(wgpu::MapMode::Write, move |result| {
                ready.store(result.is_ok(), Ordering::Release);
            });
        self.frame = frame.id;
    }
}

/// Whether a plane's data holds every row of a `width` by `height` plane.
fn plane_fits(
    data: &[u8],
    stride: usize,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
) -> bool {
    let row = width as usize * format.block_copy_size(None).unwrap_or(1) as usize;
    let rows = height as usize;
    rows > 0 && stride >= row && data.len() >= stride * (rows - 1) + row
}

fn extent(width: u32, height: u32) -> wgpu::Extent3d {
    wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    }
}

/// The texture each plane goes into: a channel per byte, so two byte
/// samples are put back together in the shader, where filtering them a
/// byte at a time still blends their values correctly.
fn plane_formats(layout: Layout) -> &'static [wgpu::TextureFormat] {
    use wgpu::TextureFormat::{R8Unorm, Rg8Unorm, Rgba8Unorm};
    match layout {
        Layout::Planar8 => &[R8Unorm, R8Unorm, R8Unorm],
        Layout::Planar16 => &[Rg8Unorm, Rg8Unorm, Rg8Unorm],
        Layout::SemiPlanar8 => &[R8Unorm, Rg8Unorm],
        Layout::SemiPlanar16 => &[Rg8Unorm, Rgba8Unorm],
        Layout::Rgba => &[Rgba8Unorm],
    }
}

/// What every window's pictures share on the graphics card.
pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    /// The subtitle picture's own group, so changing it leaves the
    /// planes' alone.
    subtitle_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    /// Bound where a layout has fewer planes than the shader reads.
    blank: wgpu::TextureView,
    /// Bound while no subtitle picture shows.
    no_subtitle: wgpu::BindGroup,
    srgb: bool,
    surfaces: HashMap<u64, Surface>,
}

/// How long a window's planes stay after it last drew, as closed windows
/// say nothing.
const KEEP_UNUSED: Duration = Duration::from_secs(5);

impl shader::Pipeline for Pipeline {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("plaa video"),
            source: wgpu::ShaderSource::Wgsl(include_str!("video.wgsl").into()),
        });
        let texture = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("plaa video"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                texture(1),
                texture(2),
                texture(3),
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let subtitle_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("plaa video subtitle"),
            entries: &[texture(0)],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("plaa video"),
            bind_group_layouts: &[&layout, &subtitle_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("plaa video"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            multiview: None,
            cache: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("plaa video"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..wgpu::SamplerDescriptor::default()
        });
        let blank = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("plaa video blank"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &blank,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &[0],
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        let blank = blank.create_view(&wgpu::TextureViewDescriptor::default());
        let no_subtitle = subtitle_group(device, &subtitle_layout, &blank);
        Self {
            pipeline,
            layout,
            subtitle_layout,
            sampler,
            blank,
            no_subtitle,
            srgb: format.is_srgb(),
            surfaces: HashMap::new(),
        }
    }

    fn trim(&mut self) {
        let now = Instant::now();
        self.surfaces
            .retain(|_, surface| now.duration_since(surface.used) < KEEP_UNUSED);
    }
}

#[cfg(test)]
mod tests {
    use super::Zoom;

    /// Where the picture's point `point`, from its middle in fractions,
    /// shows in the widget, from its middle in fractions.
    fn shown_at(zoom: Zoom, point: (f32, f32), extent: (f32, f32)) -> (f32, f32) {
        let (size, middle) = zoom.placed(extent);
        (
            (middle.0 + point.0 * 2.0 * size.0) / 2.0,
            -(middle.1 - point.1 * 2.0 * size.1) / 2.0,
        )
    }

    fn close(left: (f32, f32), right: (f32, f32)) -> bool {
        (left.0 - right.0).abs() < 1e-4 && (left.1 - right.1).abs() < 1e-4
    }

    #[test]
    fn zooming_keeps_the_point_under_the_pointer() {
        // A wide picture, filling the widget's width but not its height.
        let extent = (1.0, 0.6);
        let pointer = (0.2, -0.05);
        let fitted = Zoom::default();
        let under = (pointer.0 / extent.0, pointer.1 / extent.1);
        assert!(close(shown_at(fitted, under, extent), pointer));
        let zoomed = fitted.at(3.0, pointer, extent);
        assert!((zoomed.scale - 1.25f32.powi(3)).abs() < 1e-4);
        // Taller than the widget now, so the point stays on both axes.
        assert!(close(shown_at(zoomed, under, extent), pointer));
        let further = zoomed.at(2.0, (-0.3, 0.2), extent);
        let under = (
            zoomed.center.0 - 0.3 / (extent.0 * zoomed.scale),
            zoomed.center.1 + 0.2 / (extent.1 * zoomed.scale),
        );
        assert!(close(shown_at(further, under, extent), (-0.3, 0.2)));
        // Near its top edge, it moves down rather than show the gap
        // above it: the top of the picture stays at the widget's top.
        let top = fitted.at(3.0, (0.0, -0.25), extent);
        assert!(close(shown_at(top, (0.0, -0.5), extent), (0.0, -0.5)));
    }

    #[test]
    fn dragging_moves_the_picture_with_the_pointer() {
        let extent = (1.0, 0.8);
        let zoomed = Zoom::default().at(4.0, (0.0, 0.0), extent);
        let point = (0.05, -0.02);
        let before = shown_at(zoomed, point, extent);
        let dragged = zoomed.dragged((0.1, -0.05), extent);
        assert!(close(
            shown_at(dragged, point, extent),
            (before.0 + 0.1, before.1 - 0.05)
        ));
        // Not past where the picture's edge meets the widget's.
        let far = dragged.dragged((5.0, 5.0), extent);
        assert!(close(shown_at(far, (-0.5, -0.5), extent), (-0.5, -0.5)));
        // Fitted, there is nothing to move.
        let fitted = Zoom::default();
        assert_eq!(fitted.dragged((0.3, 0.3), extent), fitted);
    }

    #[test]
    fn the_picture_covers_the_widget_while_zoomed() {
        let extent = (1.0, 1.0);
        // Zoomed in at the very corner: the corner stays in the corner.
        let zoomed = Zoom::default().at(4.0, (0.5, 0.5), extent);
        let edge = 0.5 - 0.5 / zoomed.scale;
        assert!(close(zoomed.center, (edge, edge)));
        // Out again all the way: fitted and in the middle.
        let back = zoomed.at(-10.0, (0.1, 0.1), extent);
        assert_eq!(back, Zoom::default());
        // Never smaller than fitted, nor beyond the most.
        assert_eq!(Zoom::default().at(-1.0, (0.0, 0.0), extent).scale, 1.0);
        assert_eq!(
            Zoom::default().at(100.0, (0.0, 0.0), extent).scale,
            super::MOST_ZOOM
        );
        // An axis where the picture is still smaller than the widget
        // stays in the middle.
        let narrow = Zoom::default().at(1.0, (0.3, 0.3), (0.5, 1.0));
        assert_eq!(narrow.center.0, 0.0);
        assert!(narrow.center.1 > 0.0);
    }

    #[test]
    fn the_shader_is_valid_wgsl() {
        let module = naga::front::wgsl::parse_str(include_str!("video.wgsl"))
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(include_str!("video.wgsl"))));
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::empty(),
        )
        .validate(&module)
        .unwrap_or_else(|error| panic!("{error:?}"));
        // The uniforms the shader reads are the bytes `uniforms` writes.
        let uniforms = module
            .types
            .iter()
            .find(|(_, kind)| kind.name.as_deref() == Some("Uniforms"))
            .map(|(handle, _)| handle)
            .expect("a Uniforms struct");
        let mut layouter = naga::proc::Layouter::default();
        layouter.update(module.to_ctx()).expect("a layout");
        assert_eq!(layouter[uniforms].size as usize, super::UNIFORMS);
    }
}
