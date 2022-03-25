use crate::*;
// 以下为数据结构
// 片元类型
#[derive(Debug, Clone)]
pub enum PrimitiveType { 
    LineStrip, 
    LinesList,
    TrianglesList,
    TriangleStrpi,
    Points,
}

// 顶点类型
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position : [f32; 2],
    pub tex_coords : [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

// 绘制管线方案
#[derive(Debug, Clone)]
pub enum RenderType {
    Default,
    ColorBlend,
}
