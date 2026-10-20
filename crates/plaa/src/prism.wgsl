// The prism visualizer, drawn by marching rays through a scene the sound
// moves: a lace orb in thin-film colors around a white-hot core that
// swells with each beat, in a crown of glass shards rising with the
// spectrum, over a glowing grid, against a nebula folded from Kali's
// fractal. Drawn small, then stretched over the window with its colors
// parting toward the edges, vignetted and grained.

struct Uniforms {
    // The scene's size in pixels.
    size: vec2<f32>,
    // Seconds of the scene, which runs faster as the sound is louder.
    flow: f32,
    // How far the colors have turned, a whole turn being 1.
    hue: f32,
    // The low, middle and high parts of the spectrum, 0 to 1.
    bass: f32,
    mid: f32,
    treble: f32,
    // A beat, 1 as it lands, falling to 0.
    kick: f32,
    // How loud it all is, 0 to 1.
    level: f32,
    // Seconds since the scene began, for the grain.
    time: f32,
    // The target stores linear light, so the result is decoded.
    srgb_target: u32,
    // Unused: the bands start at a multiple of 16 bytes.
    padding: f32,
    // The spectrum's 32 bands, four to a vector.
    bands: array<vec4<f32>, 8>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var scene_picture: texture_2d<f32>;
@group(1) @binding(1) var scene_sampler: sampler;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    // From the top left, 0 to 1.
    @location(0) uv: vec2<f32>,
};

const TAU: f32 = 6.28318530718;
const STEPS: i32 = 128;
// The sphere the orb and the crown stay inside.
const BOUND: f32 = 3.9;
// The crown's radius, and how many shards it has.
const CROWN: f32 = 2.6;
const SHARDS: f32 = 32.0;
const GROUND: f32 = -2.3;

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOut {
    // A strip of two triangles over the whole target.
    let corner = vec2<f32>(f32(index & 1u), f32(index >> 1u));
    var out: VertexOut;
    out.position = vec4<f32>(corner * 2.0 - 1.0, 0.0, 1.0);
    out.uv = vec2<f32>(corner.x, 1.0 - corner.y);
    return out;
}

fn band(index: i32) -> f32 {
    let kept = clamp(index, 0, 31);
    return uniforms.bands[kept / 4][kept % 4];
}

// Inigo Quilez's cosine palette, all the hues, turned by the hue.
fn palette(at: f32) -> vec3<f32> {
    return 0.5 + 0.5 * cos(TAU * (at + uniforms.hue + vec3<f32>(0.0, 0.33, 0.67)));
}

// The palette with little green: magentas, violets, blues and teals,
// for the nebula.
fn deep(at: f32) -> vec3<f32> {
    return palette(at * 0.8 + 0.6) * vec3<f32>(1.0, 0.45, 1.0);
}

fn turn(plane: vec2<f32>, angle: f32) -> vec2<f32> {
    let cosine = cos(angle);
    let sine = sin(angle);
    return vec2<f32>(cosine * plane.x - sine * plane.y, sine * plane.x + cosine * plane.y);
}

fn hash(cell: vec3<f32>) -> f32 {
    return fract(sin(dot(cell, vec3<f32>(127.1, 311.7, 74.7))) * 43758.5453);
}

fn srgb_to_linear(encoded: vec3<f32>) -> vec3<f32> {
    let low = encoded / 12.92;
    let high = pow((encoded + 0.055) / 1.055, vec3<f32>(2.4));
    return select(high, low, encoded <= vec3<f32>(0.04045));
}

fn linear_to_srgb(linear: vec3<f32>) -> vec3<f32> {
    let low = linear * 12.92;
    let high = 1.055 * pow(linear, vec3<f32>(1.0 / 2.4)) - 0.055;
    return select(high, low, linear <= vec3<f32>(0.0031308));
}

// The ACES filmic curve, as Krzysztof Narkowicz fits it.
fn filmic(color: vec3<f32>) -> vec3<f32> {
    let lifted = color * (2.51 * color + 0.03);
    let rolled = color * (2.43 * color + 0.59) + 0.14;
    return clamp(lifted / rolled, vec3<f32>(0.0), vec3<f32>(1.0));
}

// A sphere's shell carved into lace by a gyroid, turning, its holes
// opening with the highs, its lace finer with the middle.
fn orb(point: vec3<f32>) -> f32 {
    let radius = 1.15 + 0.3 * uniforms.bass + 0.12 * uniforms.kick;
    let across = turn(point.xz, uniforms.flow * 0.23);
    var turned = vec3<f32>(across.x, point.y, across.y);
    let upward = turn(turned.yz, uniforms.flow * 0.17);
    turned = vec3<f32>(turned.x, upward.x, upward.y);
    let scale = 3.2 + 1.8 * uniforms.mid;
    let folded = turned * scale;
    let gyroid = abs(dot(sin(folded), cos(folded.zxy))) / scale;
    let shell = abs(length(point) - radius) - 0.07;
    return max(shell, gyroid - 0.035 - 0.06 * uniforms.treble) * 0.6;
}

fn core(point: vec3<f32>) -> f32 {
    return length(point) - (0.34 + 0.22 * uniforms.kick + 0.12 * uniforms.bass);
}

// The crown's own space: tipped toward the camera, and spinning.
fn crown_space(point: vec3<f32>) -> vec3<f32> {
    let tipped = turn(point.yz, 0.38);
    let spun = turn(vec2<f32>(point.x, tipped.y), uniforms.flow * 0.09);
    return vec3<f32>(spun.x, tipped.x, spun.y);
}

// Shard `index` of the crown, from the point in the crown's space: its
// distance, its band and that band's level. The bands go up one side and
// down the other, so the crown meets itself.
fn shard(crown: vec3<f32>, index: f32) -> vec3<f32> {
    let outward = turn(crown.xz, -index * TAU / SHARDS);
    let which = (i32(index) + 64) % 32;
    let shown = select((31 - which) * 2 + 1, which * 2, which < 16);
    let level = band(shown);
    let height = 0.05 + 1.05 * level;
    let local = vec3<f32>(outward.x - CROWN, crown.y, outward.y);
    let outside = abs(local) - vec3<f32>(0.06, height, 0.03);
    let box = length(max(outside, vec3<f32>(0.0)))
        + min(max(outside.x, max(outside.y, outside.z)), 0.0) - 0.025;
    return vec3<f32>(box * 0.8, f32(shown), level);
}

// The nearest shard, of the two either side of the point: shards differ
// in height, so the one in the point's own sector is not always nearest.
fn shards(point: vec3<f32>) -> vec3<f32> {
    let crown = crown_space(point);
    let below = floor(atan2(crown.z, crown.x) / (TAU / SHARDS));
    let first = shard(crown, below);
    let second = shard(crown, below + 1.0);
    return select(second, first, first.x < second.x);
}

// The core's glow along the ray to `until`, as if all light came from its
// middle: the integral of 1 / (a little + distance squared) squared,
// worked out whole, so it has no seams where rays take different steps.
fn core_glow(origin: vec3<f32>, direction: vec3<f32>, until: f32) -> f32 {
    let closest = -dot(origin, direction);
    let miss = max(dot(origin, origin) - closest * closest, 0.0);
    let squared = 0.04 + miss;
    let spread = sqrt(squared);
    let end = until - closest;
    let start = -closest;
    let upto = end / (2.0 * squared * (squared + end * end))
        + atan(end / spread) / (2.0 * squared * spread);
    let before = start / (2.0 * squared * (squared + start * start))
        + atan(start / spread) / (2.0 * squared * spread);
    return upto - before;
}

fn distance_at(point: vec3<f32>) -> f32 {
    return min(min(orb(point), core(point)), shards(point).x);
}

fn normal_at(point: vec3<f32>) -> vec3<f32> {
    let offset = vec2<f32>(1.0, -1.0) * 0.0015;
    return normalize(
        offset.xyy * distance_at(point + offset.xyy)
        + offset.yyx * distance_at(point + offset.yyx)
        + offset.yxy * distance_at(point + offset.yxy)
        + offset.xxx * distance_at(point + offset.xxx)
    );
}

// The nebula, Kali's fractal folded along the view `folds` times,
// drifting with the scene and folding tighter with the bass; and stars.
fn sky(direction: vec3<f32>, folds: i32) -> vec3<f32> {
    var point = direction * 0.55 + vec3<f32>(0.31, 0.17, 0.23 + uniforms.flow * 0.004);
    var previous = 0.0;
    var color = vec3<f32>(0.0);
    let fold = 0.56 + 0.03 * uniforms.bass;
    for (var step = 0; step < folds; step++) {
        point = abs(point) / dot(point, point) - fold;
        let size = length(point);
        color += deep(f32(step) * 0.09) * abs(size - previous) * 0.022;
        previous = size;
    }
    color = color * color * 0.6;
    let chance = hash(floor(direction * 220.0));
    let twinkle = 0.6 + 0.4 * sin(uniforms.time * 3.0 + chance * 100.0);
    color += vec3<f32>(smoothstep(0.9975, 1.0, chance) * twinkle);
    return color + deep(0.7) * 0.05 * (1.0 - abs(direction.y));
}

// A grid of light on the ground far below, rippling out from under the
// orb with the bass, and the core's light pooled there.
fn ground(origin: vec3<f32>, direction: vec3<f32>) -> vec4<f32> {
    if direction.y > -0.001 {
        return vec4<f32>(0.0);
    }
    let along = (GROUND - origin.y) / direction.y;
    let spot = origin + direction * along;
    let out = length(spot.xz);
    let ripple = 0.25 * uniforms.bass * sin(out * 2.2 - uniforms.flow * 3.0) / (1.0 + out * 0.3);
    let cells = abs(fract((spot.xz + ripple) * 0.75) - 0.5);
    let line = min(cells.x, cells.y);
    let width = 0.006 + along * 0.0012;
    let lit = smoothstep(width * 2.0, 0.0, line) * exp(-along * 0.11);
    let pool = vec3<f32>(1.0, 0.55, 0.25) * exp(-out * 0.7) * (0.2 + 0.8 * uniforms.kick);
    let color = palette(out * 0.06 + 0.55) * lit * (0.2 + 0.9 * uniforms.level) + pool;
    // Into the sky at the horizon, with no edge.
    return vec4<f32>(color, smoothstep(0.0, 0.18, -direction.y));
}

@fragment
fn fs_scene(in: VertexOut) -> @location(0) vec4<f32> {
    let shorter = min(uniforms.size.x, uniforms.size.y);
    let centered = (in.uv * 2.0 - 1.0) * uniforms.size / shorter * vec2<f32>(1.0, -1.0);
    // Round and round, bobbing, a little nearer with the bass.
    let angle = uniforms.flow * 0.11 + 0.6;
    let away = 6.4 - 0.6 * uniforms.bass;
    let origin = vec3<f32>(
        sin(angle) * away,
        1.6 + 0.8 * sin(uniforms.flow * 0.05),
        cos(angle) * away
    );
    let forward = normalize(-origin);
    let right = normalize(cross(forward, vec3<f32>(0.0, 1.0, 0.0)));
    let up = cross(right, forward);
    // Each beat punches the lens out a little.
    let lens = 1.8 - 0.18 * uniforms.kick;
    let direction = normalize(forward * lens + right * centered.x + up * centered.y);

    let backdrop = sky(direction, 11);
    var color = backdrop;
    let below = ground(origin, direction);
    color = mix(color, color * 0.35 + below.rgb, below.a);

    // Into the bounding sphere, through the scene, gathering the core's
    // and the shards' glow on the way.
    var glow = vec3<f32>(0.0);
    var hit = false;
    var kind = 0;
    var shard = vec3<f32>(0.0);
    var travelled = 0.0;
    var point = origin;
    let middle = dot(origin, direction);
    let reach = middle * middle - (dot(origin, origin) - BOUND * BOUND);
    if reach > 0.0 {
        let root = sqrt(reach);
        let far = -middle + root;
        travelled = max(-middle - root, 0.0);
        for (var step = 0; step < STEPS; step++) {
            point = origin + direction * travelled;
            let lace = orb(point);
            let heart = core(point);
            let nearest_shard = shards(point);
            var nearest = lace;
            kind = 0;
            if heart < nearest {
                nearest = heart;
                kind = 1;
            }
            if nearest_shard.x < nearest {
                nearest = nearest_shard.x;
                kind = 2;
                shard = nearest_shard;
            }
            if nearest < 0.0005 + 0.001 * travelled {
                hit = true;
                break;
            }
            // Glow by the length of the step, so it gathers evenly however
            // the steps fall, close around each shard.
            let length = min(nearest, 0.25);
            glow += palette(nearest_shard.y / 32.0 * 0.8 + 0.1) * nearest_shard.z
                * 0.006 * length / (0.004 + nearest_shard.x * nearest_shard.x);
            // At least a little way, so a ray grazing a shard goes on.
            travelled += max(nearest, 0.0015 + 0.0008 * travelled);
            if travelled > far {
                break;
            }
        }
    }

    if hit {
        let normal = normal_at(point);
        let facing = clamp(dot(normal, -direction), 0.0, 1.0);
        let fresnel = pow(1.0 - facing, 4.0);
        let light = normalize(vec3<f32>(0.5, 0.9, 0.3));
        let diffuse = max(dot(normal, light), 0.0);
        let shine = pow(max(dot(reflect(direction, normal), light), 0.0), 40.0);
        // Reflections are small and bent: a rougher nebula does.
        let mirrored = sky(reflect(direction, normal), 6);
        // The core lights the lace from within.
        let inward = max(dot(normal, -normalize(point)), 0.0);
        let warmth = vec3<f32>(1.0, 0.6, 0.3) * inward
            * (0.4 + 1.6 * uniforms.kick + uniforms.bass) / (0.4 + dot(point, point) * 0.5);
        if kind == 0 {
            // Thin film: the color turns with the angle it is seen at.
            let film = palette(
                1.7 * facing + 0.35 * point.y + uniforms.flow * 0.03 + uniforms.treble * 0.4
            );
            color = film * (0.15 + 0.55 * diffuse) + warmth * 1.4
                + mirrored * (0.25 + fresnel) + vec3<f32>(shine * 0.8);
        } else if kind == 1 {
            let hot = mix(vec3<f32>(1.0, 0.45, 0.15), vec3<f32>(1.0, 0.93, 0.8), facing * facing);
            color = hot * (1.4 + 1.4 * uniforms.kick);
        } else {
            let tint = palette(shard.y / 32.0 * 0.8 + 0.1);
            color = tint * (0.25 + 2.2 * shard.z + 0.2 * diffuse)
                + mirrored * fresnel * 0.8 + vec3<f32>(shine);
        }
        color = mix(color, backdrop, 1.0 - exp(-travelled * 0.015));
    }
    let until = select(1000.0, travelled, hit);
    glow += vec3<f32>(1.0, 0.62, 0.32) * (0.5 + uniforms.kick) * 0.1
        * core_glow(origin, direction, until);
    color += glow;

    // A ring of light spreading from the middle with each beat.
    let spread = 0.15 + (1.0 - uniforms.kick) * 1.6;
    let wave = exp(-abs(length(centered) - spread) * 18.0) * uniforms.kick * uniforms.kick;
    color += palette(spread * 0.3 + 0.5) * wave * 0.6;

    let mapped = filmic(color * (0.85 + 0.15 * uniforms.kick));
    return vec4<f32>(linear_to_srgb(mapped), 1.0);
}

@fragment
fn fs_finish(in: VertexOut) -> @location(0) vec4<f32> {
    // The colors part toward the edges, more on each beat.
    let from_middle = in.uv - 0.5;
    let split = from_middle * (0.012 + 0.05 * uniforms.kick) * length(from_middle);
    let red = textureSample(scene_picture, scene_sampler, in.uv + split).r;
    let green = textureSample(scene_picture, scene_sampler, in.uv).g;
    let blue = textureSample(scene_picture, scene_sampler, in.uv - split).b;
    var shown = vec3<f32>(red, green, blue);
    shown *= 1.0 - 0.45 * pow(length(from_middle) * 1.3, 2.5);
    // Grain at the screen's own pixels, so it stays fine.
    let grain = hash(vec3<f32>(floor(in.position.xy), floor(uniforms.time * 30.0))) - 0.5;
    shown = clamp(shown + grain * 0.03, vec3<f32>(0.0), vec3<f32>(1.0));
    if uniforms.srgb_target == 1u {
        shown = srgb_to_linear(shown);
    }
    return vec4<f32>(shown, 1.0);
}
