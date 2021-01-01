#version 330 core
out vec4 FragColor;

in vec3 ourColor;
uniform vec3 current_red;
uniform ivec2 some_int;

void main()
{
    FragColor = vec4(ourColor, 1.0f);
}
