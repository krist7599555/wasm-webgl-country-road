precision mediump float;
 
uniform sampler2D u_texture;
varying vec2 v_texcoord;
 
void main() {
  gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
  gl_FragColor = texture2D(u_texture, v_texcoord);
}