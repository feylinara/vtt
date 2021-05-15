#version 330

uniform sampler2D token;
uniform uint renderpass;

in vec2 texpos;
flat in uint frag_token_name;

layout(location=0) out vec4 color;
layout(location=1) out uvec3 click;

void main() {
    color = texture2D(token, texpos);
    if (texture2D(token, texpos).a > 0.0) {
        click = uvec3(renderpass, (frag_token_name >> 8) & uint(0xff), frag_token_name & uint(0xff));
    }
}
