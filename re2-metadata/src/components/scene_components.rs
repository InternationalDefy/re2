use crate::*;

// 以下Component 由场景持有
// GLSL Program Data
component_struct!(
    ShaderDataComponent {
        program_key : String,
        fshader_name : String,
        vshader_name : String,
    }
);
// Vertex Buffer Data
component_struct!(
    VertexDataComponent {
        vertex_key : String,
        vertex : Vec<Vertex>,
    }
);
// Sprite Texture Data
component_struct!(
    SpriteDataComponent {
        texture_key : String,
        image_path : &'static str,
    }
);
