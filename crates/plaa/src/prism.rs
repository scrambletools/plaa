//! The prism visualizer: a scene drawn on the graphics card by marching
//! rays, filling a sound file's window and moved by the sound heard. The
//! spectrum is worked out here on each frame, its bands and their thirds
//! eased, beats found in the bass, and the scene's time run faster as
//! the sound is louder; `prism.wgsl` draws it at no more than 800,000
//! pixels, then stretches it over the window.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use iced::advanced::graphics::Viewport;
use iced::wgpu;
use iced::widget::shader::{self, Action};
use iced::{Element, Fill, Rectangle, mouse};
use plaa_media::Player;
use plaa_media::spectrum::{FRAMES, Spectrum};

use crate::window::Activity;

/// The bands the scene is moved by.
const BANDS: usize = 32;

/// How quickly the bands rise to the sound, and how far they fall back
/// in a second.
const RISE: f32 = 24.0;
const FALL: f32 = 1.1;

/// How far the bass must rise over its recent level to be a beat, and
/// fall back below it before the next.
const BEAT: f32 = 0.07;
const REARM: f32 = 0.02;

/// How quickly a beat's flash fades, in a second.
const BEAT_FADE: f32 = 2.2;

/// The most pixels the scene is drawn at before it is stretched.
const MOST_PIXELS: f32 = 800_000.0;

/// The scene's colors before they are stretched over the window: as the
/// screen shows them, not linear light.
const SCENE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

/// The size of the shader's `Uniforms`: 12 words and 32 bands.
const UNIFORMS: usize = 176;

/// The scene filling the widget, moved by what `player` plays. `surface`
/// tells windows apart. Pointer moves are noted in `activity`, and
/// `on_activity` is sent when one should bring the hidden overlay back.
pub fn view<'a, Message: Clone + 'a>(
    player: &'a Player,
    surface: u64,
    activity: Arc<Activity>,
    hide_cursor: bool,
    on_activity: Message,
) -> Element<'a, Message> {
    shader::Shader::new(Program {
        player,
        surface,
        activity,
        hide_cursor,
        on_activity,
    })
    .width(Fill)
    .height(Fill)
    .into()
}

struct Program<'a, Message> {
    player: &'a Player,
    surface: u64,
    activity: Arc<Activity>,
    hide_cursor: bool,
    on_activity: Message,
}

/// What the scene is moved by, as last heard.
#[derive(Debug, Clone, Copy, Default)]
struct Sound {
    bands: [f32; BANDS],
    bass: f32,
    mid: f32,
    treble: f32,
    kick: f32,
    level: f32,
    flow: f32,
    hue: f32,
    time: f32,
}

/// The sound as the widget follows it from frame to frame.
pub struct Listening {
    spectrum: Spectrum,
    sound: Sound,
    /// The bass's recent level, which a beat rises above.
    recent_bass: f32,
    /// Whether the bass has fallen back since the last beat.
    armed: bool,
    began: Instant,
    /// When it was last worked out, while it moves.
    at: Option<Instant>,
}

impl Default for Listening {
    fn default() -> Self {
        Self {
            spectrum: Spectrum::new(),
            sound: Sound::default(),
            recent_bass: 0.0,
            armed: true,
            began: Instant::now(),
            at: None,
        }
    }
}

/// The mean of `levels`.
fn mean(levels: &[f32]) -> f32 {
    levels.iter().sum::<f32>() / levels.len().max(1) as f32
}

impl Listening {
    /// Takes in what `player` played by `now`. Returns whether the scene
    /// still moves, so the next frame is wanted.
    fn listen(&mut self, player: &Player, now: Instant) -> bool {
        let heard = player
            .heard(FRAMES)
            .map(|(frames, rate)| self.spectrum.bands(&frames, rate, BANDS));
        self.hear(heard, now)
    }

    /// Takes in the bands `heard` by `now`, if any, as `listen` does.
    fn hear(&mut self, heard: Option<Vec<f32>>, now: Instant) -> bool {
        let fresh = heard.clone().unwrap_or_else(|| vec![0.0; BANDS]);
        let elapsed = self
            .at
            .map_or(0.0, |at| now.saturating_duration_since(at).as_secs_f32())
            .min(0.1);
        let sound = &mut self.sound;
        let rise = 1.0 - (-RISE * elapsed).exp();
        for (level, fresh) in sound.bands.iter_mut().zip(&fresh) {
            *level = if *fresh > *level {
                *level + (fresh - *level) * rise
            } else {
                (*level - FALL * elapsed).max(*fresh)
            };
        }
        sound.bass = mean(&sound.bands[..6]);
        sound.mid = mean(&sound.bands[6..18]);
        sound.treble = mean(&sound.bands[18..]);
        sound.level = mean(&sound.bands);
        // A beat is the bass rising well over where it has lately been.
        let bass = mean(&fresh[..6]);
        let rising = bass - self.recent_bass;
        self.recent_bass += (bass - self.recent_bass) * (1.0 - (-2.0 * elapsed).exp());
        if self.armed && rising > BEAT {
            sound.kick = 1.0;
            self.armed = false;
        } else {
            sound.kick = (sound.kick - BEAT_FADE * elapsed).max(0.0);
            if rising < REARM {
                self.armed = true;
            }
        }
        sound.flow += elapsed * (0.25 + 1.6 * sound.level + 0.8 * sound.kick);
        sound.hue = (sound.hue + elapsed * (0.008 + 0.05 * sound.treble)).fract();
        sound.time = now.saturating_duration_since(self.began).as_secs_f32() % 3600.0;
        let moving =
            heard.is_some() || sound.kick > 0.0 || sound.bands.iter().any(|level| *level > 0.0);
        self.at = moving.then_some(now);
        moving
    }
}

impl<Message: Clone> shader::Program<Message> for Program<'_, Message> {
    type State = Listening;
    type Primitive = Primitive;

    fn update(
        &self,
        listening: &mut Self::State,
        event: &iced::Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        match event {
            iced::Event::Window(iced::window::Event::RedrawRequested(now)) => listening
                .listen(self.player, *now)
                .then(Action::request_redraw),
            iced::Event::Mouse(mouse::Event::CursorMoved { .. }) => self
                .activity
                .touch()
                .then(|| Action::publish(self.on_activity.clone())),
            _ => None,
        }
    }

    fn draw(
        &self,
        listening: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Primitive {
        Primitive {
            surface: self.surface,
            sound: listening.sound,
        }
    }

    fn mouse_interaction(
        &self,
        _listening: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if self.hide_cursor && cursor.is_over(bounds) {
            mouse::Interaction::Hidden
        } else {
            mouse::Interaction::default()
        }
    }
}

#[derive(Debug)]
pub struct Primitive {
    surface: u64,
    sound: Sound,
}

/// The values the shader reads, laid out as its `Uniforms`.
fn uniforms(sound: &Sound, size: (u32, u32), srgb_target: bool) -> [u8; UNIFORMS] {
    let mut bytes = [0u8; UNIFORMS];
    let words = [
        size.0 as f32,
        size.1 as f32,
        sound.flow,
        sound.hue,
        sound.bass,
        sound.mid,
        sound.treble,
        sound.kick,
        sound.level,
        sound.time,
    ];
    for (index, word) in words.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&word.to_ne_bytes());
    }
    bytes[40..44].copy_from_slice(&u32::from(srgb_target).to_ne_bytes());
    for (index, level) in sound.bands.iter().enumerate() {
        let at = 48 + index * 4;
        bytes[at..at + 4].copy_from_slice(&level.to_ne_bytes());
    }
    bytes
}

/// The size the scene is drawn at for a widget `width` by `height`
/// pixels: the same, or smaller in its shape when that is too many.
fn scene_size(width: f32, height: f32) -> (u32, u32) {
    let shrink = (MOST_PIXELS / (width * height).max(1.0)).sqrt().min(1.0);
    (
        (width * shrink).round().max(1.0) as u32,
        (height * shrink).round().max(1.0) as u32,
    )
}

impl shader::Primitive for Primitive {
    type Pipeline = Pipeline;

    fn prepare(
        &self,
        pipeline: &mut Pipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: &Rectangle,
        viewport: &Viewport,
    ) {
        let scale = viewport.scale_factor();
        let placed = Rectangle {
            x: bounds.x * scale,
            y: bounds.y * scale,
            width: bounds.width * scale,
            height: bounds.height * scale,
        };
        let size = scene_size(placed.width, placed.height);
        let srgb = pipeline.srgb;
        let surface = pipeline
            .surfaces
            .entry(self.surface)
            .or_insert_with(|| Surface::new(device, &pipeline.uniforms_layout));
        surface.used = Instant::now();
        surface.placed = placed;
        if surface
            .scene
            .as_ref()
            .is_none_or(|scene| scene.size != size)
        {
            surface.scene = Some(Scene::new(
                device,
                size,
                &pipeline.picture_layout,
                &pipeline.sampler,
            ));
        }
        queue.write_buffer(&surface.uniforms, 0, &uniforms(&self.sound, size, srgb));
    }

    fn render(
        &self,
        pipeline: &Pipeline,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        let Some(surface) = pipeline.surfaces.get(&self.surface) else {
            return;
        };
        let Some(scene) = &surface.scene else {
            return;
        };
        if clip_bounds.width == 0 || clip_bounds.height == 0 {
            return;
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("plaa prism scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &scene.view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&pipeline.scene);
            pass.set_bind_group(0, &surface.group, &[]);
            pass.draw(0..4, 0..1);
        }
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("plaa prism"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        let placed = surface.placed;
        pass.set_viewport(
            placed.x,
            placed.y,
            placed.width.max(1.0),
            placed.height.max(1.0),
            0.0,
            1.0,
        );
        pass.set_scissor_rect(
            clip_bounds.x,
            clip_bounds.y,
            clip_bounds.width,
            clip_bounds.height,
        );
        pass.set_pipeline(&pipeline.finish);
        pass.set_bind_group(0, &surface.group, &[]);
        pass.set_bind_group(1, &scene.group, &[]);
        pass.draw(0..4, 0..1);
    }
}

/// A window's scene on the graphics card.
struct Surface {
    uniforms: wgpu::Buffer,
    group: wgpu::BindGroup,
    scene: Option<Scene>,
    /// Where the widget is in the window, in pixels.
    placed: Rectangle,
    used: Instant,
}

impl Surface {
    fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("plaa prism uniforms"),
            size: UNIFORMS as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("plaa prism uniforms"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms.as_entire_binding(),
            }],
        });
        Self {
            uniforms,
            group,
            scene: None,
            placed: Rectangle::default(),
            used: Instant::now(),
        }
    }
}

/// The picture the scene is drawn into, and its group for stretching it.
struct Scene {
    size: (u32, u32),
    view: wgpu::TextureView,
    group: wgpu::BindGroup,
}

impl Scene {
    fn new(
        device: &wgpu::Device,
        size: (u32, u32),
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("plaa prism scene"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: SCENE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("plaa prism scene"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        });
        Self { size, view, group }
    }
}

/// What every window's scene shares on the graphics card.
pub struct Pipeline {
    /// Draws the scene into its picture.
    scene: wgpu::RenderPipeline,
    /// Stretches the picture over the widget.
    finish: wgpu::RenderPipeline,
    uniforms_layout: wgpu::BindGroupLayout,
    picture_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    srgb: bool,
    surfaces: HashMap<u64, Surface>,
}

/// How long a window's scene stays after it last drew, as closed windows
/// say nothing.
const KEEP_UNUSED: Duration = Duration::from_secs(5);

impl shader::Pipeline for Pipeline {
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("plaa prism"),
            source: wgpu::ShaderSource::Wgsl(include_str!("prism.wgsl").into()),
        });
        let uniforms_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("plaa prism uniforms"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let picture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("plaa prism scene"),
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
        let pipeline = |label, layouts: &[&wgpu::BindGroupLayout], entry, format| {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: layouts,
                push_constant_ranges: &[],
            });
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&layout),
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
                    entry_point: Some(entry),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                multiview: None,
                cache: None,
            })
        };
        let scene = pipeline(
            "plaa prism scene",
            &[&uniforms_layout],
            "fs_scene",
            SCENE_FORMAT,
        );
        let finish = pipeline(
            "plaa prism",
            &[&uniforms_layout, &picture_layout],
            "fs_finish",
            format,
        );
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("plaa prism"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..wgpu::SamplerDescriptor::default()
        });
        Self {
            scene,
            finish,
            uniforms_layout,
            picture_layout,
            sampler,
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
    use super::*;

    #[test]
    fn the_shader_is_valid_wgsl() {
        let module = naga::front::wgsl::parse_str(include_str!("prism.wgsl"))
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(include_str!("prism.wgsl"))));
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
        assert_eq!(layouter[uniforms].size as usize, UNIFORMS);
    }

    #[test]
    fn large_scenes_are_drawn_smaller_in_their_shape() {
        assert_eq!(scene_size(800.0, 600.0), (800, 600));
        let (width, height) = scene_size(3840.0, 2160.0);
        assert!((width * height) as f32 <= MOST_PIXELS * 1.01);
        assert!(((width as f32 / height as f32) - 16.0 / 9.0).abs() < 0.01);
    }

    #[test]
    fn a_beat_is_found_in_the_bass_once() {
        let mut listening = Listening::default();
        let start = Instant::now();
        let frame = Duration::from_millis(16);
        let at = |count: u32| start + frame * count;
        let quiet = vec![0.1; BANDS];
        let mut loud = quiet.clone();
        loud[..6].fill(0.6);
        for count in 0..60 {
            listening.hear(Some(quiet.clone()), at(count));
        }
        assert_eq!(listening.sound.kick, 0.0);
        // The bass jumps: a beat.
        listening.hear(Some(loud.clone()), at(60));
        assert_eq!(listening.sound.kick, 1.0);
        // Held there, it fades, and no other comes.
        for count in 61..120 {
            listening.hear(Some(loud.clone()), at(count));
            assert!(listening.sound.kick < 1.0);
        }
        assert_eq!(listening.sound.kick, 0.0);
        // Once quiet again, the next jump is a beat.
        for count in 120..240 {
            listening.hear(Some(quiet.clone()), at(count));
        }
        listening.hear(Some(loud), at(240));
        assert_eq!(listening.sound.kick, 1.0);
    }

    #[test]
    fn silence_comes_to_rest() {
        let mut listening = Listening::default();
        let start = Instant::now();
        assert!(listening.hear(Some(vec![0.8; BANDS]), start));
        let mut count = 1;
        while listening.hear(None, start + Duration::from_millis(16) * count) {
            count += 1;
            assert!(count < 500, "never came to rest");
        }
        let flow = listening.sound.flow;
        assert!(!listening.hear(None, start + Duration::from_secs(60)));
        assert_eq!(listening.sound.flow, flow, "still once at rest");
    }
}
