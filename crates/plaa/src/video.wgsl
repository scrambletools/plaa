// Draws a decoded picture: reads its planes, weighing the texels each
// screen pixel covers when shrunk, turns Y'CbCr into R'G'B', maps HDR and wide gamut
// pictures into what a standard screen shows, and fits the picture
// inside the widget, turned upright.

struct Uniforms {
    // Rows of the Y'CbCr to R'G'B' matrix; w is unused.
    row_r: vec4<f32>,
    row_g: vec4<f32>,
    row_b: vec4<f32>,
    // Half the picture's size within the widget, in clip space.
    extent: vec2<f32>,
    // Black and the centre of chroma, and how far white and full chroma
    // are from them, in 0 to 1 values.
    luma_offset: f32,
    luma_scale: f32,
    chroma_offset: f32,
    chroma_scale: f32,
    // What a stored value is multiplied by to give 0 to 1.
    value_scale: f32,
    // Clockwise quarter turns.
    turns: u32,
    // 0: Y, U and V planes; 1: Y and interleaved UV; 2: RGBA.
    plane_layout: u32,
    // Samples are two bytes, read as two channels.
    wide: u32,
    // 0: standard; 1: PQ; 2: HLG.
    transfer: u32,
    wide_gamut: u32,
    // The target stores linear light, so the result is decoded first.
    srgb_target: u32,
    _padding0: u32,
    _padding1: u32,
    _padding2: u32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var plane0: texture_2d<f32>;
@group(0) @binding(2) var plane1: texture_2d<f32>;
@group(0) @binding(3) var plane2: texture_2d<f32>;
@group(0) @binding(4) var picture_sampler: sampler;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOut {
    // A strip of two triangles: bottom left, bottom right, top left,
    // top right.
    let corner = vec2<f32>(f32(index & 1u), f32(index >> 1u));
    var out: VertexOut;
    out.position = vec4<f32>((corner * 2.0 - 1.0) * uniforms.extent, 0.0, 1.0);
    // Where on screen, from the top left.
    let screen = vec2<f32>(corner.x, 1.0 - corner.y);
    // The picture's point shown there, turned clockwise.
    switch uniforms.turns {
        case 1u: { out.uv = vec2<f32>(screen.y, 1.0 - screen.x); }
        case 2u: { out.uv = vec2<f32>(1.0 - screen.x, 1.0 - screen.y); }
        case 3u: { out.uv = vec2<f32>(1.0 - screen.y, screen.x); }
        default: { out.uv = screen; }
    }
    return out;
}

// One sample: a byte as is, or two bytes, low then high.
fn one(texel: vec4<f32>) -> f32 {
    if uniforms.wide == 1u {
        return dot(texel.rg, vec2<f32>(255.0, 65280.0)) * uniforms.value_scale;
    }
    return texel.r * 255.0 * uniforms.value_scale;
}

// Two interleaved samples, as U and V.
fn two(texel: vec4<f32>) -> vec2<f32> {
    if uniforms.wide == 1u {
        return vec2<f32>(
            dot(texel.rg, vec2<f32>(255.0, 65280.0)),
            dot(texel.ba, vec2<f32>(255.0, 65280.0)),
        ) * uniforms.value_scale;
    }
    return texel.rg * 255.0 * uniforms.value_scale;
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

// SMPTE ST 2084: signal to light, in units of 10,000 nits.
fn pq_to_linear(encoded: vec3<f32>) -> vec3<f32> {
    let m1 = 0.1593017578125;
    let m2 = 78.84375;
    let c1 = 0.8359375;
    let c2 = 18.8515625;
    let c3 = 18.6875;
    let power = pow(max(encoded, vec3<f32>(0.0)), vec3<f32>(1.0 / m2));
    return pow(max(power - c1, vec3<f32>(0.0)) / (c2 - c3 * power), vec3<f32>(1.0 / m1));
}

// ARIB STD-B67: signal to scene light, 0 to 1.
fn hlg_to_scene(encoded: vec3<f32>) -> vec3<f32> {
    let a = 0.17883277;
    let b = 0.28466892;
    let c = 0.55991073;
    let low = encoded * encoded / 3.0;
    let high = (exp((encoded - c) / a) + b) / 12.0;
    return select(high, low, encoded <= vec3<f32>(0.5));
}

const BT2020_LUMA = vec3<f32>(0.2627, 0.6780, 0.0593);
// Nits shown as the screen's white.
const REFERENCE_WHITE = 203.0;
// Brightest highlight assumed in HDR pictures, in nits.
const PEAK = 1000.0;

// Brings highlights above white down smoothly, keeping hue: extended
// Reinhard on luminance, with the peak landing on white.
fn tone_map(light: vec3<f32>) -> vec3<f32> {
    let luminance = dot(light, BT2020_LUMA);
    if luminance <= 0.0 {
        return vec3<f32>(0.0);
    }
    let white = PEAK / REFERENCE_WHITE;
    let mapped = luminance * (1.0 + luminance / (white * white)) / (1.0 + luminance);
    return light * (mapped / luminance);
}

// BT.2020 primaries to BT.709's, in linear light.
fn bt2020_to_bt709(light: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        dot(light, vec3<f32>(1.6605, -0.5876, -0.0728)),
        dot(light, vec3<f32>(-0.1246, 1.1329, -0.0083)),
        dot(light, vec3<f32>(-0.0182, -0.1006, 1.1187)),
    );
}

// At most this many texels across or down are read for a pixel; pictures
// shrunk further than that skip some.
const MOST_TEXELS = 64;

// The weight of a texel `distance` from a pixel's centre, in units of the
// pixel's size on the picture: a Hermite curve, as mpv shrinks pictures.
fn hermite(distance: f32) -> f32 {
    let x = min(abs(distance), 1.0);
    return (2.0 * x - 3.0) * x * x + 1.0;
}

// The weight of the texel from `start` to `start + 1` and of the next one,
// together, and how far past the first's centre one linear sample weighs
// the two that way.
fn pair(start: f32, centre: f32, reach: f32) -> vec2<f32> {
    let near = hermite((start + 0.5 - centre) / reach);
    let far = hermite((start + 1.5 - centre) / reach);
    let both = near + far;
    return vec2<f32>(both, select(0.0, far / both, both > 0.0));
}

// The plane at `uv` for a screen pixel `pixel` in size, in 0 to 1 units.
// Where the picture is shrunk, a pixel weighs every texel within its size
// of its centre, so fine lines and text keep steady instead of showing
// moiré where single samples would skip texels; at its size or larger it
// is filtered linearly.
fn sample_plane(plane: texture_2d<f32>, uv: vec2<f32>, pixel: vec2<f32>) -> vec4<f32> {
    let size = vec2<f32>(textureDimensions(plane));
    let texels = pixel * size;
    if all(texels <= vec2<f32>(1.0)) {
        return textureSampleLevel(plane, picture_sampler, uv, 0.0);
    }
    let reach = max(texels, vec2<f32>(1.0));
    let centre = uv * size;
    let first = floor(centre - reach - 0.5);
    let count = min(
        vec2<i32>(floor(centre + reach - 0.5) - first) + 1,
        vec2<i32>(MOST_TEXELS),
    );
    var sum = vec4<f32>(0.0);
    var weight = 0.0;
    // Two texels at a time: one linear sample between them weighs both.
    for (var row = 0; row < count.y; row += 2) {
        let down = pair(first.y + f32(row), centre.y, reach.y);
        for (var column = 0; column < count.x; column += 2) {
            let across = pair(first.x + f32(column), centre.x, reach.x);
            let texel = first + vec2<f32>(f32(column) + across.y, f32(row) + down.y) + 0.5;
            let share = across.x * down.x;
            sum += textureSampleLevel(plane, picture_sampler, texel / size, 0.0) * share;
            weight += share;
        }
    }
    return sum / max(weight, 1e-6);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    // The picture is only scaled and turned, so a screen pixel covers the
    // same part of it everywhere.
    let pixel = abs(dpdx(in.uv)) + abs(dpdy(in.uv));
    var encoded: vec3<f32>;
    if uniforms.plane_layout == 2u {
        encoded = sample_plane(plane0, in.uv, pixel).rgb;
    } else {
        let luma = one(sample_plane(plane0, in.uv, pixel));
        var chroma: vec2<f32>;
        if uniforms.plane_layout == 1u {
            chroma = two(sample_plane(plane1, in.uv, pixel));
        } else {
            chroma = vec2<f32>(
                one(sample_plane(plane1, in.uv, pixel)),
                one(sample_plane(plane2, in.uv, pixel)),
            );
        }
        let yuv = vec3<f32>(
            (luma - uniforms.luma_offset) * uniforms.luma_scale,
            (chroma.x - uniforms.chroma_offset) * uniforms.chroma_scale,
            (chroma.y - uniforms.chroma_offset) * uniforms.chroma_scale,
        );
        encoded = vec3<f32>(
            dot(uniforms.row_r.xyz, yuv),
            dot(uniforms.row_g.xyz, yuv),
            dot(uniforms.row_b.xyz, yuv),
        );
    }
    encoded = clamp(encoded, vec3<f32>(0.0), vec3<f32>(1.0));

    var shown = encoded;
    if uniforms.transfer == 1u {
        let light = pq_to_linear(encoded) * (10000.0 / REFERENCE_WHITE);
        let fitted = clamp(bt2020_to_bt709(tone_map(light)), vec3<f32>(0.0), vec3<f32>(1.0));
        shown = linear_to_srgb(fitted);
    } else if uniforms.transfer == 2u {
        // The display side of HLG for a 1,000 nit screen: system gamma 1.2.
        let scene = hlg_to_scene(encoded);
        let scene_luminance = max(dot(scene, BT2020_LUMA), 1e-6);
        let light = scene * pow(scene_luminance, 0.2) * (PEAK / REFERENCE_WHITE);
        let fitted = clamp(bt2020_to_bt709(tone_map(light)), vec3<f32>(0.0), vec3<f32>(1.0));
        shown = linear_to_srgb(fitted);
    } else if uniforms.wide_gamut == 1u {
        let light = pow(encoded, vec3<f32>(2.4));
        shown = pow(clamp(bt2020_to_bt709(light), vec3<f32>(0.0), vec3<f32>(1.0)), vec3<f32>(1.0 / 2.4));
    }

    if uniforms.srgb_target == 1u {
        shown = srgb_to_linear(shown);
    }
    return vec4<f32>(shown, 1.0);
}
