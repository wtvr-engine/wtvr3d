#define USE_TANGENT

attribute vec4 a_position;
attribute vec3 a_normal;
attribute vec2 a_tex_coordinates;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;
uniform mat4 u_world_transform;
uniform mat4 u_transpose_inverse;

varying vec2 v_tex_coordinates;
varying vec3 v_normal;

#ifdef USE_TANGENT
varying vec3 v_tangeant;
varying vec3 v_bitangeant;
#endif

void main() {
    mat4 view_model_matrix = (u_view_matrix * u_world_transform);
    gl_Position = (u_projection_matrix * view_model_matrix) * a_position;
    v_tex_coordinates = a_tex_coordinates;
    v_normal = vec3(u_world_transform * vec4(a_normal,1.0));
}
