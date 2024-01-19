
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::globals::Globals
#import bevy_pbr::mesh_view_bindings::view,

struct PostProcessingSettings {
    intensity: f32, // 4
    color_aberration: mat3x3<f32>, // 9 * 4
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec2<u32>
#endif
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> settings: PostProcessingSettings;
@group(0) @binding(3) var<uniform> globals: Globals;

const SCREEN_SHAPE_FACTOR: f32 = 0.2;
const ROWS: f32 = 128.0;
const BRIGHTNESS: f32 = 4.0;
const EDGES_TRANSITION_SIZE: f32 = 0.025;
const CHANNEL_MASK_MIN: f32 = 0.1;

fn apply_screen_shape(uv: vec2<f32>, factor: f32) -> vec2<f32> {
    var uv2 = uv - vec2(0.5, 0.5);
    uv2 = uv2 * (uv2.yx * uv2.yx * factor + 1.0);
    return uv2 + vec2(0.5, 0.5);
}

fn pixelate(uv: vec2<f32>, size: vec2<f32>) -> vec2<f32> {
    return floor(uv * size) / size;
}

fn get_texture_color(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(screen_texture, texture_sampler, uv);
}

fn apply_pixel_rows(color: vec4<f32>, uv: vec2<f32>, rows: f32) -> vec4<f32> {
    var f = abs(fract(uv.y * ROWS) - 0.5) * 2.;
    f = f * f;
    return mix(color, vec4<f32>(0., 0., 0., 1.), f);
}

fn apply_pixel_cols(color: vec4<f32>, uv: vec2<f32>, cols: f32) -> vec4<f32> {
    var f = abs(fract(uv.x * cols * 3.) - 0.5) * 2.;
    f = f * f;

    var channel = u32(fract(uv.x * cols) * 3.0);

    var channel_mask = vec4(1.0, CHANNEL_MASK_MIN, CHANNEL_MASK_MIN, 1.0);
    if channel == 1u {
        channel_mask = vec4(CHANNEL_MASK_MIN, 1.0, CHANNEL_MASK_MIN, 1.0);
    } else if channel == 2u {
        channel_mask = vec4(CHANNEL_MASK_MIN, CHANNEL_MASK_MIN, 1.0, 1.0);
    }

    let new_color = color * channel_mask;
    return mix(new_color, vec4<f32>(0., 0., 0., 1.), f);
}

fn apply_screen_edges(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let ratio = 800.0 / 600.0;

    let edge_x = min(uv.x / EDGES_TRANSITION_SIZE, (1.0 - uv.x) / EDGES_TRANSITION_SIZE);
    let edge_y = min(uv.y / EDGES_TRANSITION_SIZE / ratio, (1.0 - uv.y) / EDGES_TRANSITION_SIZE / ratio);

    let edge = vec2(
        max(edge_x, 0.0),
        max(edge_y, 0.0),
    );
    
    var f = min(edge.x, edge.y);
    f = min(f, 1.0);

    return vec4(color.xyz * f, 1.0);
} 

fn apply_brightness(color: vec4<f32>) -> vec4<f32> {
    return color * vec4(vec3(BRIGHTNESS), 1.0);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv / vec2<f32>(800.0, 600.0);
    uv = apply_screen_shape(uv, SCREEN_SHAPE_FACTOR);
    
    let cols = ROWS * 800.0 / 600.0;

    var texture_uv = uv;
    texture_uv = pixelate(texture_uv, vec2(cols, ROWS));

    var color = get_texture_color(texture_uv);

    color = apply_pixel_rows(color, uv, ROWS);
    color = apply_pixel_cols(color, uv, cols);

    color = apply_brightness(color);
    color = apply_screen_edges(color, uv);

    return color;
}