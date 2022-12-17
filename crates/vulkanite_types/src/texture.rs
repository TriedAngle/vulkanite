use crate::{
    Extent3D, ImageSubresourceRange, SharingMode, TextureDimension, TextureFormat, TextureUsages,
    TextureViewDimension,
};

pub struct TextureInfo {
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub size: Extent3D,
    pub mip_levels: u32,
    pub samples: u32,
    pub usage: TextureUsages,
    pub sharing: SharingMode,
}

pub struct TextureViewInfo {
    pub dimension: TextureViewDimension,
    pub format: TextureFormat,
    pub range: ImageSubresourceRange,
}
