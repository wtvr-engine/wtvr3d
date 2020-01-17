precision mediump float;

#define NUM_DIR_LIGHTS 0
#define NUM_POINT_LIGHTS 0

// Typedefs
struct Light {
    vec3 position_or_direction;
    float intensity;
    vec3 color;
    float attenuation;
};

// User-defined uniforms
uniform sampler2D u_tex_diffuse;
uniform sampler2D u_tex_normal;
uniform float u_roughness;

// Lights

uniform vec3 u_camera_position;

#if NUM_DIR_LIGHTS > 0
uniform Light u_dir_lights[NUM_DIR_LIGHTS];
#endif

#if NUM_POINT_LIGHTS > 0
uniform Light u_point_lights[NUM_POINT_LIGHTS];
#endif

uniform vec4 u_ambiant_light;

// Varyings
varying vec2 v_tex_coordinates;
varying vec3 v_normal;
varying vec3 v_position;
varying mat3 v_tbn_matrix;

#pragma glslify: lambert = require(glsl-diffuse-lambert) 
#pragma glslify: beckmann = require(glsl-specular-beckmann) 


vec4 light_value(vec3 light_direction, vec3 light_color, float light_intensity, vec3 normal, vec3 view_direction) {
    vec3 light_dir = normalize(-light_direction);
    float power = lambert(light_dir,normal) + beckmann(light_dir,view_direction,normal,u_roughness);
    return vec4(light_color*light_intensity,power);
}

vec3 get_normal(){
    vec3 normal = texture2D(u_tex_normal,vec2(v_tex_coordinates.x, 1.0 - v_tex_coordinates.y)).rgb;
    normal = normalize(normal * 2.0 - 1.0);
    return normalize(v_tbn_matrix * normal);
}

void main() {
    vec3 normal = get_normal();
    vec3 view_direction = normalize(u_camera_position - v_position);
    vec4 diffuse = texture2D(u_tex_diffuse, vec2(v_tex_coordinates.x, 1.0 - v_tex_coordinates.y));
    vec3 computed_light_color = u_ambiant_light.rgb*u_ambiant_light.a;
    float total_intensity = u_ambiant_light.a;
#if NUM_DIR_LIGHTS > 0
    for(int i = 0; i < NUM_DIR_LIGHTS; i++){
        vec4 dir_light = light_value(u_dir_lights[i].position_or_direction, u_dir_lights[i].color, u_dir_lights[i].intensity,normal,view_direction);
        computed_light_color += dir_light.rgb*dir_light.a;
    }
#endif
#if NUM_POINT_LIGHTS > 0
    for(int i = 0; i < NUM_POINT_LIGHTS; i++){
        vec3 direction = v_position - u_point_lights[i].position_or_direction;
        vec4 point_light = light_value(direction, u_point_lights[i].color, u_point_lights[i].intensity,normal,view_direction);
        computed_light_color += point_light.rgb*point_light.a;
    }
#endif
    gl_FragColor = vec4(diffuse.rgb*computed_light_color,diffuse.a);
}