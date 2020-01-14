/// Name for the view matrix uniform
pub const VIEW_MATRIX_NAME: &str = "u_view_matrix";

/// Name for the view matrix uniform
pub const CAMERA_POSITION_NAME: &str = "u_camera_position";

/// Name for the view matrix uniform
pub const PROJECTION_MATRIX_NAME: &str = "u_projection_matrix";

/// Name for the world transform (model) matrix uniform
pub const WORLD_TRANSFORM_NAME: &str = "u_world_transform";

/// Name for the ambiant light uniform
pub const AMBIANT_LIGHT_NAME: &str = "u_ambiant_light";

/// Name for the point lights array uniform
pub const POINT_LIGHTS_NAME: &str = "u_point_lights";

/// Name for the directional lights array uniform
pub const DIRECTIONAL_LIGHTS_NAME: &str = "u_dir_lights";

/// Name for the color field in the Light GLSL struct
pub const LIGHT_COLOR_NAME: &str = "color";

/// Name for the intensity field in the Light GLSL struct
pub const LIGHT_INTENSITY_NAME: &str = "intensity";

/// Name for the attenuation field in the Light GLSL struct
pub const LIGHT_ATTENUATION_NAME: &str = "attenuation";

/// Name for the direction/position field in the Light GLSL struct
pub const LIGHT_POSITION_DIRECTION_NAME: &str = "position_or_direction";

/// Vertex (positions) buffer name used in shaders
pub const VERTEX_BUFFER_NAME: &str = "a_position";

/// Normal buffer name used in shaders
pub const NORMAL_BUFFER_NAME: &str = "a_normal";

/// UV (texture coordinates) buffer name used in shaders
pub const UV_BUFFER_NAME: &str = "a_tex_coordinates";
