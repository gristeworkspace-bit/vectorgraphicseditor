use wasm_bindgen::prelude::*;
use rustybuzz::{Face as RbFace, UnicodeBuffer, shape};
use ttf_parser::{Face as TtfFace, OutlineBuilder, GlyphId};
use std::fmt::Write;

#[wasm_bindgen]
pub fn convert_text_to_svg(font_data: &[u8], text: &str) -> String {
    // 1. Rustybuzzで配置計算
    let rb_face = RbFace::from_slice(font_data, 0).expect("フォントの読み込みに失敗しました");
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);

    let glyph_buffer = shape(&rb_face, &[], buffer);

    // 2. ttf-parserで形状抽出の準備
    let ttf_face = TtfFace::parse(font_data, 0).expect("ttf-parserでの読み込みに失敗しました");
    
    let mut path_data = String::new();
    
    // ★追加: 現在の描画位置（カーソル位置）
    let mut current_x = 0.0;
    let mut current_y = 0.0;

    // グリフごとに処理
    for (i, info) in glyph_buffer.glyph_infos().iter().enumerate() {
        let pos = glyph_buffer.glyph_positions()[i];
        let glyph_id = GlyphId(info.glyph_id as u16);

        let mut builder = SvgPathBuilder {
            path_data: String::new(),
            // ★変更: カーソル位置(current_x)を加算する
            offset_x: current_x + (pos.x_offset as f32),
            offset_y: current_y + (pos.y_offset as f32),
        };

        if let Some(_) = ttf_face.outline_glyph(glyph_id, &mut builder) {
            write!(&mut path_data, "{} ", builder.path_data).unwrap();
        }

        // ★追加: 次の文字のためにカーソルを進める
        current_x += pos.x_advance as f32;
        current_y += pos.y_advance as f32;
    }

    path_data
}

// --- OutlineBuilder の実装 (変更なし) ---
struct SvgPathBuilder {
    path_data: String,
    offset_x: f32,
    offset_y: f32,
}

impl OutlineBuilder for SvgPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path_data.push_str(&format!("M {} {} ", x + self.offset_x, -y + self.offset_y));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.path_data.push_str(&format!("L {} {} ", x + self.offset_x, -y + self.offset_y));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path_data.push_str(&format!("Q {} {} {} {} ", x1 + self.offset_x, -y1 + self.offset_y, x + self.offset_x, -y + self.offset_y));
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path_data.push_str(&format!("C {} {} {} {} {} {} ", x1 + self.offset_x, -y1 + self.offset_y, x2 + self.offset_x, -y2 + self.offset_y, x + self.offset_x, -y + self.offset_y));
    }
    fn close(&mut self) {
        self.path_data.push_str("Z ");
    }
}