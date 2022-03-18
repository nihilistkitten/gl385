#version 300 es

uniform mat4 matrix;
in vec4 position;
in vec4 color;
out vec4 vColor;

void main() {
  gl_Position = matrix * position;
	vColor = color;
}
