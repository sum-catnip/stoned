#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct K {
    time: f32,
    intensity: f32,
    base_level: f32,
    peak_frequency: f32,
    peak_sharpness: f32,
    peak_intensity: f32,
    wave_frequency: f32,
    wave_intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: vec2<f32>,
#endif
}

@group(0) @binding(2) var<uniform> settings: K;

const PI: f32 = 3.14159265359;
const TAU: f32 = 6.28318530718;

fn hash(p: vec2<f32>) -> f32 {
    let h = dot(p, vec2<f32>(127.1, 311.7));
    return fract(sin(h) * 43758.5453123);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash(i + vec2<f32>(0.0, 0.0)), hash(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y
    );
}

fn fbm(p: vec2<f32>) -> f32 {
    var value = 0.0;
    var amp = 0.5;
    var pos = p;
    for (var i = 0; i < 5; i++) {
        value += amp * noise(pos);
        amp *= 0.5;
        pos *= 2.0;
    }
    return value;
}

fn dynamic_intensity(time: f32) -> f32 {
    let base = settings.base_level;
    let peak_freq = settings.peak_frequency;
    let sharpness = settings.peak_sharpness;
    let peak_max = settings.peak_intensity;
    let wave_freq = settings.wave_frequency;
    let wave_amp = settings.wave_intensity;
    
    // waves
    let small_wave = sin(time * wave_freq) * 0.5 + 0.5;
    let ambient = base + small_wave * wave_amp;
    
    // sharp spikes
    let peak_phase = time * peak_freq * 0.1;
    let raw_peak = sin(peak_phase);
    let peak = pow(max(raw_peak, 0.0), sharpness);
    
    let peak_phase2 = time * peak_freq * 0.07 + 1.5;
    let raw_peak2 = sin(peak_phase2);
    let peak2 = pow(max(raw_peak2, 0.0), sharpness * 1.5) * 0.6;
    
    // k-hole
    let mega_phase = time * peak_freq * 0.02;
    let raw_mega = sin(mega_phase);
    let mega_peak = pow(max(raw_mega, 0.0), sharpness * 2.0) * 0.4;
    
    let combined_peaks = max(max(peak, peak2), mega_peak);
    let final_intensity = ambient + combined_peaks * (peak_max - base);
    
    return clamp(final_intensity, 0.0, 1.0);
}

fn stereographic_distort(uv: vec2<f32>, time: f32, intensity: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5);
    var p = uv - center;
    
    let r = length(p);
    let theta = atan2(p.y, p.x);
    
    let projected_r = tan(r * PI * 0.4 * intensity) * 0.3;
    let spiral = theta + r * 3.0 * sin(time * 0.3) * intensity;
    let breath = 1.0 + 0.1 * sin(time * 0.5) * intensity;
    
    return center + vec2<f32>(cos(spiral), sin(spiral)) * projected_r * breath;
}

fn hole(uv: vec2<f32>, time: f32, intensity: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5);
    var p = uv - center;
    
    let r = length(p);
    let theta = atan2(p.y, p.x);
    
    let tunnel_r = 0.1 / (r + 0.1) * intensity;
    let spiral_theta = theta + r * 5.0 - time * 0.5;
    let blend = smoothstep(0.0, 0.3, r);
    
    let tunnel_uv = center + vec2<f32>(cos(spiral_theta), sin(spiral_theta)) * tunnel_r;
    return mix(tunnel_uv, uv, 1.0 - intensity * (1.0 - blend));
}

fn kaleidoscope(uv: vec2<f32>, segments: f32, time: f32) -> vec2<f32> {
    let center = vec2<f32>(0.5);
    var p = uv - center;
    
    let angle = atan2(p.y, p.x) + time * 0.1;
    let r = length(p);
    let segment_angle = TAU / segments;
    let new_angle = abs(((angle % segment_angle) - segment_angle * 0.5));
    
    return center + vec2<f32>(cos(new_angle), sin(new_angle)) * r;
}

fn wave_distort(uv: vec2<f32>, time: f32, intensity: f32) -> vec2<f32> {
    var d = uv;
    d.x += sin(uv.y * 10.0 + time * 2.0) * 0.01 * intensity;
    d.y += sin(uv.x * 8.0 + time * 1.5) * 0.01 * intensity;
    d.x += sin(uv.y * 20.0 - time * 3.0) * 0.005 * intensity;
    d.y += cos(uv.x * 15.0 + time * 2.5) * 0.005 * intensity;
    return d;
}

fn chromatic_aberration(uv: vec2<f32>, intensity: f32) -> vec3<f32> {
    let center = vec2<f32>(0.5);
    let dir = normalize(uv - center);
    let dist = length(uv - center);
    let aberration = dist * 0.03 * intensity;
    
    let r = textureSample(screen_texture, texture_sampler, uv + dir * aberration).r;
    let g = textureSample(screen_texture, texture_sampler, uv).g;
    let b = textureSample(screen_texture, texture_sampler, uv - dir * aberration).b;
    
    return vec3<f32>(r, g, b);
}

fn shader_color_shift(color: vec3<f32>, time: f32, intensity: f32) -> vec3<f32> {
    let angle = time * 0.3 * intensity;
    let s = sin(angle);
    let c = cos(angle);
    let weights = vec3<f32>(0.213, 0.715, 0.072);
    
    var result: vec3<f32>;
    result.r = dot(color, vec3<f32>(
        weights.x + c * (1.0 - weights.x) - s * weights.x,
        weights.y - c * weights.y - s * 0.143,
        weights.z - c * weights.z + s * (1.0 - weights.z)
    ));
    result.g = dot(color, vec3<f32>(
        weights.x - c * weights.x + s * 0.14,
        weights.y + c * (1.0 - weights.y) + s * 0.14,
        weights.z - c * weights.z - s * 0.283
    ));
    result.b = dot(color, vec3<f32>(
        weights.x - c * weights.x - s * (1.0 - weights.x),
        weights.y - c * weights.y + s * weights.y,
        weights.z + c * (1.0 - weights.z) + s * weights.z
    ));
    
    // Pulsing saturation
    let gray = dot(result, vec3<f32>(0.299, 0.587, 0.114));
    let sat_pulse = 1.0 + 0.5 * sin(time * 2.0) * intensity;
    result = mix(vec3<f32>(gray), result, sat_pulse);
    
    // Posterization
    let posterize = 4.0 + 12.0 * (1.0 - intensity);
    result = floor(result * posterize) / posterize;
    
    return result;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;
    let time = settings.time;
    let master = settings.intensity;
    
    let shader_intensity = dynamic_intensity(time) * master;
    
    // thresholds
    let wave_active = smoothstep(0.15, 0.35, shader_intensity);
    let stereo_active = smoothstep(0.35, 0.55, shader_intensity);
    let kaleido_active = smoothstep(0.45, 0.65, shader_intensity);
    let khole_active = smoothstep(0.65, 0.85, shader_intensity);
    
    if (wave_active > 0.01) {
        uv = wave_distort(uv, time, wave_active * 0.8);
    }
    
    if (stereo_active > 0.01) {
        uv = mix(uv, stereographic_distort(uv, time, stereo_active), stereo_active * 0.5);
    }
    
    if (khole_active > 0.01) {
        uv = hole(uv, time, khole_active * 0.3);
    }
    
    if (kaleido_active > 0.01) {
        let segments = 6.0 + 2.0 * sin(time * 0.2);
        let kaleido_uv = kaleidoscope(uv, segments, time);
        uv = mix(uv, kaleido_uv, kaleido_active * 0.4);
    }
    
    uv = clamp(uv, vec2<f32>(0.001), vec2<f32>(0.999));
    
    var color = chromatic_aberration(uv, shader_intensity * 2.0);
    color = shader_color_shift(color, time, shader_intensity);
    
    // Fractal noise overlay
    let fractal = fbm(uv * 3.0 + vec2<f32>(time * 0.1, time * 0.05));
    color += (fractal - 0.5) * shader_intensity * 0.15;
    
    // vignette
    let dist_from_center = length(uv - vec2<f32>(0.5));
    let vignette_pulse = 1.0 + 0.1 * sin(time * 0.7);
    let vignette = 1.0 - smoothstep(0.3, 0.8 * vignette_pulse, dist_from_center) * (0.3 + shader_intensity * 0.4);
    color *= vignette;
    
    // grain
    let grain = (hash(uv * 100.0 + time * 100.0) - 0.5) * 0.05 * shader_intensity;
    color += grain;
    
    return vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.5)), 1.0);
}
