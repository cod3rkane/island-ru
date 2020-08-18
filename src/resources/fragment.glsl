#version 410 core

out vec4 FragColor;

in vec4 color;
in vec2 textureCoord;

uniform sampler2D texture1;

void main() {
  FragColor = texture(texture1, textureCoord);
}
