struct View {
    scale: vec2<f32>,
    port: vec4<f32>,
}

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    
    @location(2) offset: vec2<f32>,
    @location(3) rotation: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> view: View;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    
    var prerot: f32 = atan2(model.position.y-model.rotation.y, model.position.x-model.rotation.x) + model.rotation.z;
    var premag: f32 = sqrt(pow(model.position.x-model.rotation.x,2)+pow(model.position.y-model.rotation.y,2));
    var rotpos: vec2<f32> = vec2<f32>(cos(prerot), sin(prerot));
    var prepos: vec2<f32> = model.rotation.xy + (rotpos*premag);
    
    
    var pos: vec2<f32> = prepos + model.offset + view.port.xy;
    pos *= 0.05;
    pos *= view.scale;
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color);
}