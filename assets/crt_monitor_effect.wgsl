
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
// @group(0) @binding(4) var<uniform> view: View;

fn curve(uv: vec2<f32>) -> vec2<f32>
{
    var uv2: vec2<f32> = uv;
	uv2 = (uv2 - 0.5) * 2.0;
	uv2 *= 1.1;	
	uv2.x *= 1.0 + pow((abs(uv2.y) / 5.0), 2.0);
	uv2.y *= 1.0 + pow((abs(uv2.x) / 4.0), 2.0);
	uv2 = (uv2 / 2.0) + 0.5;
	uv2 = uv2 * 0.92 + 0.04;

	return uv2;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32>
{
    var uv = in.uv;
    let time: f32 = globals.time * 2.0;

    // let resolution: vec2<f32> = view.viewport.zw;
    let resolution: vec2<f32> = vec2<f32>(800.0, 600.0);

    let q: vec2<f32> = uv / resolution;
    
    uv = q;
    uv = curve(uv);

    let originalcol: vec3<f32> = textureSample(screen_texture, texture_sampler, vec2<f32>(q.x, q.y)).rgb;

    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    let x: f32 = sin(0.3 * time + uv.y * 21.0 ) * sin(0.7 * time + uv.y * 29.0) * 
                    sin(0.3 + 0.33 * time + uv.y * 31.0) * 0.0017;

    col.g = textureSample(screen_texture, texture_sampler, vec2<f32>(x + uv.x + 0.000, uv.y - 0.002)).y + 0.05;
    col.r = textureSample(screen_texture, texture_sampler, vec2<f32>(x + uv.x + 0.001, uv.y + 0.001)).x + 0.05;
    col.b = textureSample(screen_texture, texture_sampler, vec2<f32>(x + uv.x - 0.002, uv.y + 0.000)).z + 0.05;
    col.r += 0.08 * textureSample(screen_texture, texture_sampler, 0.75 * vec2<f32>(x + 0.025, -0.027) + vec2<f32>(uv.x + 0.001, uv.y + 0.001)).x;
    col.g += 0.05 * textureSample(screen_texture, texture_sampler, 0.75 * vec2<f32>(x +- 0.022, -0.02) + vec2<f32>(uv.x + 0.000, uv.y - 0.002)).y;
    col.b += 0.08 * textureSample(screen_texture, texture_sampler, 0.75 * vec2<f32>(x +- 0.02, -0.018) + vec2<f32>(uv.x - 0.002, uv.y + 0.000)).z;

    col.x = clamp(col.x * 0.6 + 0.4 * col.x * col.x * 1.0, 0.0, 1.0);
    col.y = clamp(col.y * 0.6 + 0.4 * col.y * col.y * 1.0, 0.0, 1.0);
    col.z = clamp(col.z * 0.6 + 0.4 * col.z * col.z * 1.0, 0.0, 1.0);

    let vig: f32 = (0.0 + 1.0 * 16.0 * uv.x * uv.y * (1.0 - uv.x) * (1.0 - uv.y));

	col *= vec3(pow(vig, 0.3));

    col *= vec3(0.95, 1.05, 0.95);
	col *= 2.8;

    let scans: f32 = clamp(0.35 + 0.35 * sin(3.5 * time + uv.y * resolution.y * 1.5), 0.0, 1.0);
	
    let s: f32 = pow(scans, 1.7);
	// float s = pow(scans, 1.7);

	col = col * vec3( 0.4 + 0.7 * s) ;

    col *= 1.0 + 0.01 * sin(110.0 * time);
	if uv.x < 0.0 || uv.x > 1.0 {
        col *= 0.0;
    }

	if uv.y < 0.0 || uv.y > 1.0 {
        col *= 0.0;
    }
		
	col *= 1.0 - 0.65 * vec3(clamp(((in.uv.x % 2.0) - 1.0) * 2.0, 0.0, 1.0));
	
    let comp: f32 = smoothstep(0.1, 0.9, sin(time));
 
	// Remove the next line to stop cross-fade between original and postprocess
    col = mix(col, originalcol, comp);

    // return vec4(in.position);
    return vec4(originalcol, 0.3);
}