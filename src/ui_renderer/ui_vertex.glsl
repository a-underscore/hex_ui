#version 330

in vec2 position;
in vec2 uv;

out vec2 tex_pos;

uniform float z;
uniform mat3 transform;
uniform mat4 camera_proj;

void main(void) {
        vec2 pos = (vec3(position, 1.0) * transform).xy;

        gl_Position = vec4(vec3(pos, z), 1.0) * camera_proj;

	tex_pos = uv;
}
