#define USE_TANGENT

attribute vec4 a_position;
attribute vec3 a_normal;
attribute vec2 a_tex_coordinates;
attribute vec3 a_tangent;

uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;
uniform mat4 u_world_transform;
uniform mat4 u_transpose_inverse;

varying vec2 v_tex_coordinates;
varying vec3 v_normal;
varying vec3 v_position;

#ifdef USE_TANGENT
varying mat3 v_tbn_matrix;
#endif

mat3 compute_tbn_matrix(){
    vec3 t = normalize(vec3(u_world_transform * vec4(a_tangent, 0.0)));
    vec3 b = normalize(vec3(u_world_transform * vec4( cross(a_normal,a_tangent), 0.0)));
    vec3 n = normalize(vec3(u_world_transform * vec4(a_normal, 0.0)));
    return mat3(t,b,n);
}

void main() {
    mat4 view_model_matrix = (u_view_matrix * u_world_transform);
    gl_Position = (u_projection_matrix * view_model_matrix) * a_position;
    vec4 position = (u_world_transform * a_position);
    v_position = position.xyz/position.w;
    v_tex_coordinates = a_tex_coordinates;
    vec4 normal = u_world_transform * vec4(a_normal,1.0);
    v_normal = normal.xyz/normal.w;
    v_tbn_matrix = compute_tbn_matrix();
}
