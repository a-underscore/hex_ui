use ab_glyph::{point, Font, FontRef, Glyph, Point, PxScale, ScaleFont};
use hex::{
    anyhow,
    assets::Texture,
    glium::{
        texture::{MipmapsOption, RawImage2d},
        uniforms::SamplerBehavior,
        Display,
    },
};
use image::{DynamicImage, Rgba};

pub trait Text
where
    Self: Sized,
{
    fn text<S>(
        display: &Display,
        content: S,
        font: FontRef<'_>,
        scale: f32,
        padding: u32,
        max_width: f32,
        color: [f32; 4],
        mipmaps_option: MipmapsOption,
        sampler_behavior: SamplerBehavior,
    ) -> anyhow::Result<Self>
    where
        S: AsRef<str>;
}

impl Text for Texture {
    fn text<S>(
        display: &Display,
        content: S,
        font: FontRef<'_>,
        scale: f32,
        padding: u32,
        max_width: f32,
        color: [f32; 4],
        mipmaps_option: MipmapsOption,
        sampler_behavior: SamplerBehavior,
    ) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
    {
        let scaled_font = font.as_scaled(PxScale::from(scale));
        let glyphs = layout_paragraph(content.as_ref(), scaled_font, max_width, point(0.0, 0.0));
        let glyphs_height = scaled_font.height().ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs.first().unwrap().position.x;
            let last_glyph = glyphs.last().unwrap();
            let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
            (max_x - min_x).ceil() as u32
        };
        let mut image =
            DynamicImage::new_rgba8(glyphs_width + padding, glyphs_height + padding).to_rgba8();

        for g in glyphs {
            if let Some(outlined) = scaled_font.outline_glyph(g) {
                let bounds = outlined.px_bounds();

                outlined.draw(|x, y, v| {
                    let x = x + bounds.min.x as u32 - 1;
                    let y = y + bounds.min.y as u32 - 1;
                    let px = image.get_pixel(x, y);

                    image.put_pixel(
                        x,
                        y,
                        Rgba([
                            (color[0] * 255.0) as u8,
                            (color[1] * 255.0) as u8,
                            (color[2] * 255.0) as u8,
                            px.0[3].saturating_add((v * 255.0) as u8),
                        ]),
                    );
                });
            }
        }

        Self::new(
            display,
            RawImage2d::from_raw_rgba_reversed(&image.clone().into_raw(), image.dimensions()),
            mipmaps_option,
            sampler_behavior,
        )
    }
}

pub fn layout_paragraph<S, F>(content: &str, font: S, max_width: f32, position: Point) -> Vec<Glyph>
where
    S: ScaleFont<F>,
    F: Font,
{
    let v_advance = font.height() + font.line_gap();
    let mut glyphs = Vec::new();
    let mut caret = position + point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;

    for c in content.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = point(position.x, caret.y + v_advance);
                last_glyph = None;
            }

            continue;
        }

        let mut glyph = font.scaled_glyph(c);

        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }

        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        glyphs.push(glyph);
    }

    glyphs
}
