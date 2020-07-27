#version 450 core

layout (location = 0) in vec3 vertexPosition;
layout (location = 1) in vec4 vertexColor;

out vec4 color;

uniform mat4 view_matrix;

void main() {
  gl_Position = view_matrix * vec4(vertexPosition, 1.0);
  color = vertexColor;
}
