#version 450 core

layout (location = 0) in vec3 vertexPosition;
layout (location = 1) in vec2 texCoord;
layout (location = 2) in vec4 vertexColor;

out vec4 color;
out vec2 textureCoord;

uniform mat4 view_matrix;
uniform mat4 projection_matrix;

void main() {
  gl_Position = projection_matrix * view_matrix * vec4(vertexPosition, 1.0);
  color = vertexColor;
  textureCoord = texCoord;
}
