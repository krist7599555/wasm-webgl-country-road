attribute vec4 position;
attribute vec2 texcoord;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

varying vec2 v_texcoord;

void main() {
   gl_Position = projection * view * model * position;
   v_texcoord = vec2(texcoord.x, 1.0 - texcoord.y);
}