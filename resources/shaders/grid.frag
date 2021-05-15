#version 330

uniform vec2 size;
uniform sampler2D tilesheet;

in vec2 texpos;

flat in float fragtile;
uniform float ntiles;
uniform uint renderpass;

layout(location=0) out vec4 color;
layout(location=1) out uvec3 click;

void main() {
  color = texture2D(tilesheet, vec2((texpos.x + fragtile) / ntiles, 1 - texpos.y));
  click = uvec3(renderpass, 0, 0);
}