#version 330

uniform sampler2D token;

in vec2 texpos;

out vec4 color;

void main() {
    color = texture2D(token, texpos);
    // color = vec4(1.0, 0.3, 0.8, 1.0);
}