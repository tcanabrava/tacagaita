#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 ourTextCoord;

uniform sampler2D texture_1;
uniform sampler2D texture_2;

void main()
{
    FragColor = mix(texture(texture_1, ourTextCoord), texture(texture_2, ourTextCoord), 0.5);
}
