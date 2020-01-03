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

// Textures
uniform sampler2D u_tex_diffuse;

// Lights

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

vec4 dir_light_value(Light light, vec3 normal) {
    return vec4(light.color,max(dot(- light.position_or_direction,normal)*light.intensity,0.0));
}

void main() {
    vec3 normal = normalize(v_normal);
    vec4 diffuse = texture2D(u_tex_diffuse, vec2(v_tex_coordinates.x, 1.0 - v_tex_coordinates.y));
    vec3 computed_light_color = u_ambiant_light.rgb*u_ambiant_light.a;
    float total_intensity = u_ambiant_light.a;
#if NUM_DIR_LIGHTS > 0
    for(int i = 0; i < NUM_DIR_LIGHTS; i++){
        vec4 dir_light = dir_light_value(u_dir_lights[i],normal);
        computed_light_color += dir_light.rgb*dir_light.a;
        total_intensity += dir_light.a;
    }
#endif
    computed_light_color /= total_intensity;
    gl_FragColor = vec4(diffuse.rgb*computed_light_color,diffuse.a);
}