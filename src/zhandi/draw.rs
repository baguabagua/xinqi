use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};

#[derive(Resource)]
pub struct ZhandiTextureAssets {
    pub background: Handle<Image>,
    pub black_piece: Handle<Image>,
    pub white_piece: Handle<Image>,
}

// 在直线上插值点
fn interpolate_line(p1: (f32, f32), p2: (f32, f32), t: f32) -> (f32, f32) {
    let x = p1.0 + (p2.0 - p1.0) * t;
    let y = p1.1 + (p2.1 - p1.1) * t;
    (x, y)
}

// 绘制直线的函数（Bresenham算法）
fn draw_line(image: &mut Image, (x1, y1): (f32, f32), (x2, y2): (f32, f32)) {
    let mut x1 = x1.round() as i32;
    let mut y1 = y1.round() as i32;
    let x2 = x2.round() as i32;
    let y2 = y2.round() as i32;
    
    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;
    
    loop {
        // 检查坐标是否在图像范围内
        let _ = image.set_color_at(x1 as u32, y1 as u32, Color::BLACK);
        
        if x1 == x2 && y1 == y2 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x1 += sx;
        }
        if e2 <= dx {
            err += dx;
            y1 += sy;
        }
    }
}

// 绘制纹理储存在 ZhandiTextureAssets 中
pub fn zhandi_setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let mut background = Image::new_fill(
        Extent3d { width: 600, height: 533, depth_or_array_layers: 1 }, 
        TextureDimension::D2, 
        &[255, 255, 255, 255], 
        TextureFormat::Rgba8UnormSrgb, 
        RenderAssetUsages::all(), 
    );
    let width = 600;
    let height = 533;
    let hex_radius = 250.0;
    let (center_x, center_y) = ((width / 2) as f32, (height / 2) as f32);

    let vertices = vec![
        (center_x + hex_radius, center_y), // 0: 正右
        (center_x + hex_radius / 2.0, center_y - hex_radius * 3.0_f32.sqrt() / 2.0), // 1: 右上
        (center_x - hex_radius / 2.0, center_y - hex_radius * 3.0_f32.sqrt() / 2.0), // 2: 左上
        (center_x - hex_radius, center_y), // 3: 正左
        (center_x - hex_radius / 2.0, center_y + hex_radius * 3.0_f32.sqrt() / 2.0), // 4: 左下
        (center_x + hex_radius / 2.0, center_y + hex_radius * 3.0_f32.sqrt() / 2.0), // 5: 右下
    ];

    for i in 0..6 {
        let p1 = vertices[i];
        let p2 = vertices[(i+1)%6];
        let p3 = vertices[(i+2)%6];
        let p4 = vertices[(i+3)%6];
        for j in 0..5 {
            let q1 = interpolate_line(p1, p2, 0.25 * j as f32);
            let q2 = interpolate_line(p4, p3, 0.25 * j as f32);
            draw_line(&mut background, q1, q2);
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

    let textures = ZhandiTextureAssets {
        background: images.add(background),
        black_piece: images.add(black_piece),
        white_piece: images.add(white_piece),
    };
    commands.insert_resource(textures);
}