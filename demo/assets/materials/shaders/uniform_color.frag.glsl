precision mediump float;

uniform sampler2D u_tex_diffuse;
varying vec2 v_tex_coordinates;

void main() {
    gl_FragColor = texture2D(u_tex_diffuse, v_tex_coordinates);;
}