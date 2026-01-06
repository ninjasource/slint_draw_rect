use i_slint_core::software_renderer::{
    self, DrawRectangleArgs, DrawTextureArgs, PhysicalRegion, Rgb565Pixel,
};
use log::info;

use crate::slint_backend::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub struct Dma2DBuffer<'a> {
    data: &'a mut [software_renderer::Rgb565Pixel],
    pixel_stride: usize,
}

fn rgb24_to_rgb565(r: u8, g: u8, b: u8) -> u16 {
    let r5 = (r as u16 >> 3) & 0x1F; // 5 bits
    let g6 = (g as u16 >> 2) & 0x3F; // 6 bits
    let b5 = (b as u16 >> 3) & 0x1F; // 5 bits

    (r5 << 11) | (g6 << 5) | b5
}

impl<'a> i_slint_core::software_renderer::TargetPixelBuffer for Dma2DBuffer<'a> {
    type TargetPixel = software_renderer::Rgb565Pixel;

    fn line_slice(&mut self, line_number: usize) -> &mut [Self::TargetPixel] {
        let offset = line_number * self.pixel_stride;
        &mut self.data[offset..offset + self.pixel_stride]
    }

    fn num_lines(&self) -> usize {
        self.data.len() / self.pixel_stride
    }

    fn fill_background(&mut self, brush: &slint::Brush, region: &PhysicalRegion) -> bool {
        //info!(
        //    "fill_background: x {} y {} width {} height {}",
        //    region.bounding_box_origin().x,
        //    region.bounding_box_origin().y,
        //    region.bounding_box_size().width,
        //    region.bounding_box_size().height
        //);

        /*
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::RED)
            .stroke_width(3)
            .fill_color(Rgb565::GREEN)
            .build();

        Rectangle::new(Point::new(50, 20), Size::new(60, 35))
            .into_styled(style)
            .draw(self.data)
            .unwrap();
        */

        let colour = match brush {
            slint::Brush::SolidColor(c) => rgb24_to_rgb565(c.red(), c.green(), c.blue()),
            _ => panic!("brush not supported"),
        };
        let left = (region.bounding_box_origin().x as usize).max(0);
        let right = (left + region.bounding_box_size().width as usize).min(DISPLAY_WIDTH);
        let top = (region.bounding_box_origin().y as usize).max(0);
        let bottom = (top + region.bounding_box_size().height as usize).min(DISPLAY_HEIGHT);

        info!("fill_background: left {left} right {right} top {top} bottom {bottom}",);

        for x in left..right {
            for y in top..bottom {
                self.data[x + y * DISPLAY_WIDTH] = Rgb565Pixel(colour)
            }
        }

        return true;
    }

    fn draw_rectangle(&mut self, rect_args: &DrawRectangleArgs, _clip: &PhysicalRegion) -> bool {
        let colour = match rect_args.background {
            slint::Brush::SolidColor(c) => rgb24_to_rgb565(c.red(), c.green(), c.blue()),
            _ => panic!("brush not supported"),
        };

        let left = (rect_args.x as usize).max(0);
        let right = (left + rect_args.width as usize).min(DISPLAY_WIDTH);
        let top = (rect_args.y as usize).max(0);
        let bottom = (top + rect_args.height as usize).min(DISPLAY_HEIGHT);

        info!("draw_rectangle: left {left} right {right} top {top} bottom {bottom}",);

        for x in left..right {
            for y in top..bottom {
                self.data[x + y * DISPLAY_WIDTH] = Rgb565Pixel(colour)
            }
        }

        return true;
    }

    fn draw_texture(&mut self, texture: &DrawTextureArgs, _clip: &PhysicalRegion) -> bool {
        let colour = rgb24_to_rgb565(0, 0, 0);
        let left = (texture.dst_x as usize).max(0);
        let right = (left + texture.dst_width as usize).min(DISPLAY_WIDTH);
        let top = (texture.dst_y as usize).max(0);
        let bottom = (top + texture.dst_height as usize).min(DISPLAY_HEIGHT);

        info!("draw_texture: left {left} right {right} top {top} bottom {bottom}",);

        for x in left..right {
            for y in top..bottom {
                self.data[x + y * DISPLAY_WIDTH] = Rgb565Pixel(colour)
            }
        }

        return true;
    }
}

impl<'a> Dma2DBuffer<'a> {
    pub fn new(data: &'a mut [software_renderer::Rgb565Pixel], pixel_stride: usize) -> Self {
        Self { data, pixel_stride }
    }
}
