use crate::*;

// 以下Component 由实体持有
// TestTransformImpact
component_struct!(
    TestTransformImpact {
        impact_point : f32,
    }
);

// GLSL Program Key
component_struct!(
    ShaderComponent {
        program_key : String,
    }
);
// Render 渲染参数
component_struct!(
    RenderComponent {
        vertex_key : String,
        primitive_type : PrimitiveType,
        render_type : Option<RenderType>,
        indices : Option<Vec<u16>>,
    }
);
// 变形矩阵
component_struct!(
    TransformComponent {
        matrix : glam::Mat4,
    }
);
// Sprite Texture Key
component_struct!(
    SpriteComponent {
        texture_key : String,
    }
);
// Uniforms!
component_struct!(
    ColorBlendComponent {
        blend_color : glam::Vec4,
    }
);
