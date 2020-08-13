#version 450 core

layout (location = 0) in vec3 vertexPosition;
layout (location = 1) in vec4 vertexColor;
layout (location = 2) in mat4 instanceMatrix;
layout (location = 6) in float v_index;

out vec4 color;
out vec2 textureCoord;

uniform samplerBuffer texture_tbo;
uniform mat4 view_matrix;
uniform mat4 projection_matrix;

vec2 get_texture_coordinates(int index) {
   vec4 t_coord;

   if (index == 0) {
     t_coord = texelFetch(texture_tbo, gl_InstanceID * 4);

     return vec2(t_coord.x, t_coord.y);
  } else if (index == 1) {
     t_coord = texelFetch(texture_tbo, gl_InstanceID * 4 + 1);

     return vec2(t_coord.x, t_coord.y);
  } else if (index == 2) {
     t_coord = texelFetch(texture_tbo, gl_InstanceID * 4 + 2);

     return vec2(t_coord.x, t_coord.y);
  } else if (index == 3) {
     t_coord = texelFetch(texture_tbo, gl_InstanceID * 4 + 3);

     return vec2(t_coord.x, t_coord.y);
  }  
}

void main() {
  gl_Position = projection_matrix * view_matrix * instanceMatrix * vec4(vertexPosition, 1.0);
  color = vertexColor;
  textureCoord = get_texture_coordinates(gl_VertexID);
}
