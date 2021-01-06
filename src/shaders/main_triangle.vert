#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
layout (location = 2) in vec2 TextCoord;

out vec3 ourColor;
out vec2 ourTextCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = model * vec4(Position, 1.0);
    ourColor = Color;
    ourTextCoord = TextCoord;
}