#version 330

uniform vec2 dimensions;
uniform vec2 offset;
uniform mat4 projection;

layout(location = 0) in vec2 pos;

out vec2 texpos;

void main() {
    gl_Position = projection * vec4(offset + pos * dimensions, 1.0, 1.0);
    texpos = vec2(pos.x, pos.y);
}
