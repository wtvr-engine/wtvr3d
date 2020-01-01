attribute vec4 a_position;
attribute vec4 a_normal;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;
uniform mat4 u_world_transform;
uniform mat4 u_transpose_inverse;


void main() {
    mat4 view_model_matrix = (u_view_matrix * u_world_transform);
    gl_Position = (u_projection_matrix * view_model_matrix) * a_position;
}