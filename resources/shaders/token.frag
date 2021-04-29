#version 330

uniform sampler2D token;

in vec2 texpos;

out vec4 color;

void main() {
    color = texture2D(token, texpos);
}
