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
    out.v_color = vec4<f32>(in.color, 1.0);
    return out;
}

// @fragment
// fn fs_main(in: Output) -> @location(0) vec4<f32> {
//     let N: vec3<f32> = normalize(in.v_normal.xyz);
//     let L: vec3<f32> = normalize(camera.view_pos.xyz - in.v_position.xyz); // camera and light have same position
//     let V: vec3<f32> = normalize(in.v_position.xyz - camera.view_pos.xyz); // correct view vector
//     let H: vec3<f32> = normalize(L + V); // correct half vector
//
//     var diffuse: f32 = light.diffuse_intensity * max(dot(N, L), 0.0); // correct diffuse calculation
//     var specular: f32 = light.specular_intensity * pow(max(dot(N, H), 0.0), light.specular_shininess); // correct specular calculation
//
//     let ambient: f32 = light.ambient_intensity;
//
//     let final_color: vec3<f32> = in.v_color.xyz * (ambient + diffuse) + light.specular_color.xyz * specular;
//
//     return vec4<f32>(final_color, 1.0);
// }

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    let N: vec3<f32> = normalize(in.v_normal.xyz);
    let V: vec3<f32> = normalize(in.v_position.xyz - camera.view_pos.xyz);

    let light_dir_0 = vec3<f32>(0.0, 1.0, 0.0);
    let light_dir_1 = vec3<f32>(0.0, -1.0, 0.0);
    let light_dir_2 = vec3<f32>(0.0, 0.0, 1.0);
    let light_dir_3 = vec3<f32>(0.0, 0.0, -1.0);
    let light_dir_4 = vec3<f32>(1.0, 0.0, 0.0);
    let light_dir_5 = vec3<f32>(-1.0, 0.0, 0.0);

    var diffuse_sum: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var specular_sum: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    let L_0: vec3<f32> = -light_dir_0;
    let H_0: vec3<f32> = normalize(L_0 + V);
    diffuse_sum += light.diffuse_intensity * max(dot(N, L_0), 0.0) * in.v_color.xyz;
    specular_sum += light.specular_intensity * pow(max(dot(N, H_0), 0.0), light.specular_shininess) * light.specular_color.xyz;

    let L_1: vec3<f32> = -light_dir_1;
    let H_1: vec3<f32> = normalize(L_1 + V);
    diffuse_sum += light.diffuse_intensity * max(dot(N, L_1), 0.0) * in.v_color.xyz;
    specular_sum += light.specular_intensity * pow(max(dot(N, H_1), 0.0), light.specular_shininess) * light.specular_color.xyz;

    let L_2: vec3<f32> = -light_dir_2;
    let H_2: vec3<f32> = normalize(L_2 + V);
    diffuse_sum += light.diffuse_intensity * max(dot(N, L_2), 0.0) * in.v_color.xyz;
    specular_sum += light.specular_intensity * pow(max(dot(N, H_2), 0.0), light.specular_shininess) * light.specular_color.xyz;

    let L_3: vec3<f32> = -light_dir_3;
    let H_3: vec3<f32> = normalize(L_3 + V);
    diffuse_sum += light.diffuse_intensity * max(dot(N, L_3), 0.0) * in.v_color.xyz;
    specular_sum += light.specular_intensity * pow(max(dot(N, H_3), 0.0), light.specular_shininess) * light.specular_color.xyz;

    let L_4: vec3<f32> = -light_dir_4;
    let H_4: vec3<f32> = normalize(L_4 + V);
    diffuse_sum += light.diffuse_intensity * max(dot(N, L_4), 0.0) * in.v_color.xyz;
    specular_sum += light.specular_intensity * pow(max(dot(N, H_4), 0.0), light.specular_shininess) * light.specular_color.xyz;

    let L_5: vec3<f32> = -light_dir_5;
    let H_5: vec3<f32> = normalize(L_5 + V);
    diffuse_sum += light.diffuse_intensity * max(dot(N, L_5), 0.0) * in.v_color.xyz;
    specular_sum += light.specular_intensity * pow(max(dot(N, H_5), 0.0), light.specular_shininess) * light.specular_color.xyz;

    let ambient: vec3<f32> = light.ambient_intensity * in.v_color.xyz;
    let final_color: vec3<f32> = ambient + diffuse_sum + specular_sum;

    return vec4<f32>(final_color, 1.0);
}
