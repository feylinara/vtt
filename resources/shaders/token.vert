#version 330

uniform vec2 dimensions;

layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 offset;
layout(location = 2) in uint token_name;

uniform mat4 projection;

out vec2 texpos;
flat out uint frag_token_name;

void main() {
    gl_Position = projection * vec4(offset + pos * dimensions, 1.0, 1.0);
    texpos = vec2(pos.x, 1 - pos.y);
    frag_token_name = token_name;
}
