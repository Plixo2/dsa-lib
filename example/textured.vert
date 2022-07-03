#version 330 core
layout (location = 0) in vec3 a_pos; 
layout (location = 1) in vec2 a_uv; 

uniform mat4 transform; 
uniform sampler2D texture_0; 

out vec2 uv; 

#define tri_sin(a) sin(a) 

void main() {
 
    gl_Position = transform * vec4(a_pos, 1.0);
    uv = a_uv;

}

float r_sin(float value) {
    return sin(value);
}
 

