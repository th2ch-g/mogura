
struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;


struct Light {
    specular_color: vec4<f32>,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
}

@group(0) @binding(1)
var<uniform> light: Light;


struct Input {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
}

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_normal: vec4<f32>,
    @location(2) v_color: vec4<f32>,
}

@vertex
fn vs_main(in: Input) -> Output {
    var out: Output;
    out.position = camera.view_proj * vec4<f32>(in.position, 1.0);
    out.v_position = vec4<f32>(in.position, 1.0);
    out.v_normal = vec4<f32>(in.normal, 1.0);
    // out.v_normal = camera.view_proj * vec4<f32>(in.normal, 1.0);
    out.v_color = vec4<f32>(in.color, 1.0);
    return out;
}


@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    let N: vec3<f32> = normalize(in.v_normal.xyz);
    let L: vec3<f32> = normalize(camera.view_pos.xyz - in.v_position.xyz); // camera and light have same position
    let V: vec3<f32> = normalize(camera.view_pos.xyz - in.v_position.xyz); // light position and eye position also same
    let H: vec3<f32> = normalize(L + V);

    var diffuse: f32 = light.diffuse_intensity * max(dot(N, L), 0.0);
    var specular: f32 = light.specular_intensity * pow(max(dot(N, H), 0.0), light.specular_shininess);

    diffuse = diffuse + light.diffuse_intensity * max(dot(-N, L), 0.0);
    specular = specular + light.specular_intensity * pow(max(dot(-N, H), 0.0), light.specular_shininess);

    let ambient: f32 = light.ambient_intensity;
    let final_color: vec3<f32> = in.v_color.xyz * (ambient + diffuse) + light.specular_color.xyz * specular;
    return vec4<f32>(final_color, 1.0);
}
