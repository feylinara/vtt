#version 330
uniform vec2 size;
uniform sampler2D tilesheet;
in vec2 texpos;
flat in float fragtile;
uniform float ntiles;
flat in int iid;

void main() {
  gl_FragColor = texture2D(tilesheet, vec2((texpos.x + fragtile) / ntiles, 1 - texpos.y));
//   gl_FragColor = vec4(float(iid) / 2500.0, 1.0 - float(iid) / 2500.0, 0.5, 1.0);
}