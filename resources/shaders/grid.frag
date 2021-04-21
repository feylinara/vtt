#version 330
uniform vec2 size;
uniform sampler2D tilesheet;
in vec2 texpos;
flat in float fragtile;
uniform float ntiles;

void main() {
  gl_FragColor = texture2D(tilesheet, vec2((texpos.x + fragtile) / ntiles, 1 - texpos.y));
}