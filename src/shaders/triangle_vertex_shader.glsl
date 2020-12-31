#version 330 core

layout (location = 0) in vec3 Position;

out vec4 vertexColor;

void main()
{
    gl_Position = vec4(Position, 1.0);
    vertexColor = vec4(0.0f, 1.0f, 0.0f, 1.0f);
}