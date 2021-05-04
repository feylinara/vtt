#version 330

uniform sampler2D texture;

in vec2 texpos;

out vec4 color;

void main() {
    color = texture2D(texture, texpos);
}
