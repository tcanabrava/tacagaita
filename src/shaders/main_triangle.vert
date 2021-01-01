#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

out vec3 ourColor;

void main()
{
    gl_Position = vec4(Position.x, -1*Position.y, Position.z, 1.0);
    ourColor = Color;
}