#version 450 core

layout (location = 0) in vec3 vertexPosition;
layout (location = 1) in vec4 vertexColor;
layout (location = 2) in mat4 instanceMatrix;

out vec4 color;

uniform mat4 view_matrix;
uniform mat4 projection_matrix;

void main() {
  gl_Position = projection_matrix * view_matrix * instanceMatrix * vec4(vertexPosition, 1.0);
  color = vertexColor;
}
