use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};

#[derive(Resource)]
pub struct XingxiangTextureAssets {
    pub background: Handle<Image>,
    pub black_piece: Handle<Image>,
    pub white_piece: Handle<Image>,
    pub black_king: Handle<Image>,
    pub white_king: Handle<Image>,
    pub black_knight: Handle<Image>,
    pub white_knight: Handle<Image>,
    pub black_rook: Handle<Image>,
    pub white_rook: Handle<Image>,
    pub black_bishop: Handle<Image>,
    pub white_bishop: Handle<Image>,
}

const BLACKCELL_COLOR: Color = Color::srgb(181.0/256.0, 136.0/256.0, 99.0/256.0);
const WHITECELL_COLOR: Color = Color::srgb(240.0/256.0, 217.0/256.0, 181.0/256.0);

pub fn xingxiang_setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>, 
) {
    let mut background = Image::new_fill(
        Extent3d { width: 1000, height: 1000, depth_or_array_layers: 1 }, 
        TextureDimension::D2, 
        &[255, 255, 255, 255], 
        TextureFormat::Rgba8UnormSrgb, 
        RenderAssetUsages::all(), 
    );
    for x in 0..8 {
        for y in 0..8 {
            let color = if (x+y) % 2 == 0 { WHITECELL_COLOR } else { BLACKCELL_COLOR };
            for nx in 0..125 {
                for ny in 0..125 {
                    let _ = background.set_color_at(x * 125 + nx, y * 125 + ny, color);
                }
            }
        }
    }

    let black_piece = {
        let size = 50;
        let radius = 20.0;
        let center = (size as f32 / 2.0, size as f32 / 2.0);

        let mut pixels = Vec::with_capacity(size * size * 4);

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center.0;
                let dy = y as f32 - center.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= radius {
                    // 圆形内部：黑色 (0, 0, 0, 255)
                    pixels.extend_from_slice(&[0, 0, 0, 255]);
                } else {
                    // 圆形外部：完全透明 (0, 0, 0, 0)
                    pixels.extend_from_slice(&[0, 0, 0, 0]);
                }
            }
        }

        Image::new(
            Extent3d {
                width: size as u32,
                height: size as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            pixels,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::all(), 
        )
    };
    let white_piece = {
        let size = 50;
        let radius = 20.0;
        let border_width = 2.0;
        let center = (size as f32 / 2.0, size as f32 / 2.0);

        let mut pixels = Vec::with_capacity(size * size * 4);

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center.0;
                let dy = y as f32 - center.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= radius - border_width {
                    // 圆形内部（去掉边框区域）：白色 (255, 255, 255, 255)
                    pixels.extend_from_slice(&[255, 255, 255, 255]);
                } else if distance <= radius {
                    // 边框区域：黑色 (0, 0, 0, 255)
                    pixels.extend_from_slice(&[0, 0, 0, 255]);
                } else {
                    // 圆形外部：完全透明 (0, 0, 0, 0)
                    pixels.extend_from_slice(&[0, 0, 0, 0]);
                }
            }
        }

        Image::new(
            Extent3d {
                width: size as u32,
                height: size as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            pixels,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::all(), 
        )
    };

    let textures = XingxiangTextureAssets {
        background: images.add(background),
        black_piece: images.add(black_piece),
        white_piece: images.add(white_piece),
        black_king: asset_server.load("chess_pieces/BKing.png"),
        white_king: asset_server.load("chess_pieces/WKing.png"),
        black_knight: asset_server.load("chess_pieces/BKnight.png"),
        white_knight: asset_server.load("chess_pieces/WKnight.png"),
        black_rook: asset_server.load("chess_pieces/BRook.png"),
        white_rook: asset_server.load("chess_pieces/WRook.png"),
        black_bishop: asset_server.load("chess_pieces/BBishop.png"),
        white_bishop: asset_server.load("chess_pieces/WBishop.png"),
    };
    commands.insert_resource(textures);
}