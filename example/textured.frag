#version 330 core
uniform mat4 transform; 
uniform sampler2D texture_0; 

in vec2 uv; 

out vec4 result; 

#define SMOOTH false 
#define KERNEL_SIZE 0 

void main() {
 
    result = texture(texture_0, uv);

}

