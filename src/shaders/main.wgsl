struct View {
    time: vec2<f32>,
    scale: vec2<f32>,
    position: vec2<f32>,
    rotation: vec2<f32>,
}

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    
    @location(2) offset: vec2<f32>,
    @location(3) center: vec2<f32>,
    @location(4) rotation: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> view: View;

fn rotate(center: vec2<f32>, position: vec2<f32>, rotation: f32) -> vec2<f32> {
    var prerot: f32 = atan2(position.y-center.y, position.x-center.x) + rotation;
    var premag: f32 = sqrt(pow(position.x-center.x,2)+pow(position.y-center.y,2));
    var rotpos: vec2<f32> = vec2<f32>(cos(prerot), sin(prerot));
    var prepos: vec2<f32> = center + (rotpos*premag);
    return prepos;
} 

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    
    // var prerot: f32 = atan2(model.position.y-model.center.y, model.position.x-model.center.x) + model.rotation.x;
    // var premag: f32 = sqrt(pow(model.position.x-model.center.x,2)+pow(model.position.y-model.center.y,2));
    // var rotpos: vec2<f32> = vec2<f32>(cos(prerot), sin(prerot));
    // var prepos: vec2<f32> = model.center + (rotpos*premag);
    
    var prepos: vec2<f32> = rotate(model.center, model.position, model.rotation.x);
    prepos = rotate(-view.position, prepos, view.rotation.x);
    
    var pos: vec2<f32> = prepos + model.offset - view.position;
    pos *= 0.05;
    // pos *= view.scale;
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color);
}