use inline_spirv::inline_spirv;

pub const VERTEX_SHADER_CODE: &[u32] = inline_spirv!(
    r#"
#version 450

void main() {
    uint in_vertex_index = gl_VertexIndex;
    float x = float(int(in_vertex_index) - 1);
    float y = float(int(in_vertex_index & 1u) * 2 - 1);
    gl_Position = vec4(x, y, 0.0, 1.0);
}
"#,
    vert
);

pub const FRAGMENT_SHADER_CODE: &[u32] = inline_spirv!(
    r#"
#version 450

layout(location = 0) out vec4 fragment_color[4];

layout(push_constant) uniform PushConstants {
    uint surface_count;
};

void main() {
    //fragment_color[0] = vec4(1.0, 0.0, 0.0, 1.0);

    for(int i=0;i<surface_count;++i)
    {
        fragment_color[i] = vec4(1.0, 0.0, 0.0, 1.0);
    }
}
"#,
    frag
);

