precision mediump float;

uniform sampler2D u_tex_diffuse;
varying vec2 v_tex_coordinates;

void main() {
    gl_FragColor = texture2D(u_tex_diffuse, vec2(v_tex_coordinates.x, 1.0 - v_tex_coordinates.y));;
}