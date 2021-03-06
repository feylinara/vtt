#version 330
layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 offset;
layout(location = 2) in float tile;

uniform vec2 size;
uniform mat4 projection;

out vec2 texpos;
flat out float fragtile;
flat out int iid;

void main() {
    gl_Position = projection * vec4(offset + pos * size, 0.5, 1.0);
    texpos = pos;
    fragtile = tile;
    iid = gl_InstanceID;
}
