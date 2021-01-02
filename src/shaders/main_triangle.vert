#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
layout (location = 2) in vec2 TextCoord;

out vec3 ourColor;
out vec2 ourTextCoord;

uniform float h_offset;

void main()
{
    gl_Position = vec4(Position.x + h_offset, Position.y, Position.z, 1.0);
    ourColor = Color;
    ourTextCoord = TextCoord;
}