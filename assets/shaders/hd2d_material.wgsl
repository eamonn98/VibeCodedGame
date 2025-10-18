#import bevy_sprite::mesh2d_vertex_output::VertexOutput;

struct Hd2dMaterialUniform {
    base_color: vec4<f32>,
    ambient_color: vec4<f32>,
    directional_color: vec4<f32>,
    directional_dir: vec2<f32>,
    normal_strength: f32,
    point_light_count: u32,
    point_positions: array<vec4<f32>, 4>,
    point_colors: array<vec4<f32>, 4>,
};

@group(2) @binding(0) var<uniform> material: Hd2dMaterialUniform;

fn apply_directional(normal: vec3<f32>) -> vec3<f32> {
    let light_dir = normalize(vec3(material.directional_dir, 0.0));
    let ndotl = max(dot(normal, -light_dir), 0.0);
    return material.directional_color.rgb * ndotl * material.directional_color.w;
}

fn apply_point_lights(normal: vec3<f32>, world_pos: vec3<f32>) -> vec3<f32> {
    var total = vec3<f32>(0.0);
    let count = min(material.point_light_count, 4u);
    for (var i: u32 = 0u; i < count; i = i + 1u) {
        let point = material.point_positions[i];
        let color = material.point_colors[i];
        let radius = max(point.z, 0.001);
        let to_light = vec3(point.xy - world_pos.xy, 0.0);
        let distance = length(to_light);
        if (distance < radius) {
            let attenuation = 1.0 - distance / radius;
            let dir = normalize(to_light);
            let diffuse = max(dot(normal, dir), 0.0);
            total = total + color.rgb * diffuse * attenuation * attenuation * color.w;
        }
    }
    return total;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var albedo = material.base_color;
#ifdef VERTEX_COLORS
    albedo = albedo * mesh.color;
#endif

    let normal = normalize(mesh.world_normal * vec3(material.normal_strength, material.normal_strength, 1.0));
    let world_pos = mesh.world_position.xyz;

    var lighting = material.ambient_color.rgb * material.ambient_color.w;
    lighting = lighting + apply_directional(normal);
    lighting = lighting + apply_point_lights(normal, world_pos);

    let shaded = albedo.rgb * lighting;
    return vec4<f32>(shaded, albedo.a);
}
