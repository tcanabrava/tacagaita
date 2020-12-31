#version 330 core
out vec4 FragColor;

in vec3 SomeColor;

void main()
{
    FragColor = vec4(SomeColor, 1.0f);
}
