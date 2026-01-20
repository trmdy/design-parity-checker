use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, GrayImage};

#[derive(Debug, Clone, Copy)]
pub struct ImageAlignmentOptions {
    pub enabled: bool,
    pub max_shift: u32,
    pub downscale_max_dim: u32,
}

impl Default for ImageAlignmentOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            max_shift: 16,
            downscale_max_dim: 256,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlignmentOffset {
    pub dx: i32,
    pub dy: i32,
}

pub fn align_implementation(
    reference: &DynamicImage,
    implementation: &DynamicImage,
    options: ImageAlignmentOptions,
) -> (DynamicImage, Option<AlignmentOffset>) {
    if !options.enabled || options.max_shift == 0 {
        return (implementation.clone(), None);
    }

    let (ref_w, ref_h) = reference.dimensions();
    if ref_w == 0 || ref_h == 0 {
        return (implementation.clone(), None);
    }

    let mut impl_img = implementation.clone();
    if impl_img.dimensions() != (ref_w, ref_h) {
        impl_img = impl_img.resize_exact(ref_w, ref_h, FilterType::Lanczos3);
    }

    let (dx, dy) = find_best_offset(reference, &impl_img, options);
    if dx == 0 && dy == 0 {
        return (impl_img, None);
    }

    let aligned = apply_shift(reference, &impl_img, dx, dy);
    (
        aligned,
        Some(AlignmentOffset {
            dx,
            dy,
        }),
    )
}

fn find_best_offset(
    reference: &DynamicImage,
    implementation: &DynamicImage,
    options: ImageAlignmentOptions,
) -> (i32, i32) {
    let (w, h) = reference.dimensions();
    let max_dim = options.downscale_max_dim.max(1);
    let max_src_dim = w.max(h);
    let scale = if max_src_dim > max_dim {
        max_dim as f64 / max_src_dim as f64
    } else {
        1.0
    };

    let (ref_small, impl_small) = if scale < 1.0 {
        let new_w = ((w as f64 * scale).round() as u32).max(1);
        let new_h = ((h as f64 * scale).round() as u32).max(1);
        (
            reference.resize_exact(new_w, new_h, FilterType::Triangle),
            implementation.resize_exact(new_w, new_h, FilterType::Triangle),
        )
    } else {
        (reference.clone(), implementation.clone())
    };

    let mut max_shift_scaled = ((options.max_shift as f64) * scale).ceil() as i32;
    if max_shift_scaled < 1 {
        max_shift_scaled = 1;
    }

    let (dx_small, dy_small) =
        best_shift_luma(&ref_small.to_luma8(), &impl_small.to_luma8(), max_shift_scaled);

    let dx = ((dx_small as f64) / scale).round() as i32;
    let dy = ((dy_small as f64) / scale).round() as i32;
    let max_shift = options.max_shift as i32;
    (dx.clamp(-max_shift, max_shift), dy.clamp(-max_shift, max_shift))
}

fn best_shift_luma(ref_luma: &GrayImage, impl_luma: &GrayImage, max_shift: i32) -> (i32, i32) {
    let (w, h) = ref_luma.dimensions();
    let w = w as i32;
    let h = h as i32;
    if w == 0 || h == 0 {
        return (0, 0);
    }

    let ref_buf = ref_luma.as_raw();
    let impl_buf = impl_luma.as_raw();

    let mut best_score = f64::INFINITY;
    let mut best: (i32, i32) = (0, 0);

    for dy in -max_shift..=max_shift {
        for dx in -max_shift..=max_shift {
            let ref_x0 = dx.max(0);
            let ref_y0 = dy.max(0);
            let impl_x0 = (-dx).max(0);
            let impl_y0 = (-dy).max(0);
            let width = w - ref_x0 - impl_x0;
            let height = h - ref_y0 - impl_y0;
            if width <= 0 || height <= 0 {
                continue;
            }

            let mut sum = 0u64;
            for y in 0..height {
                let ref_row = (ref_y0 + y) * w + ref_x0;
                let impl_row = (impl_y0 + y) * w + impl_x0;
                for x in 0..width {
                    let ref_idx = (ref_row + x) as usize;
                    let impl_idx = (impl_row + x) as usize;
                    let diff = ref_buf[ref_idx] as i16 - impl_buf[impl_idx] as i16;
                    sum += diff.unsigned_abs() as u64;
                }
            }

            let avg = sum as f64 / (width * height) as f64;
            let best_dist = best.0.abs() + best.1.abs();
            let dist = dx.abs() + dy.abs();
            if avg + f64::EPSILON < best_score
                || ((avg - best_score).abs() <= f64::EPSILON && dist < best_dist)
            {
                best_score = avg;
                best = (dx, dy);
            }
        }
    }

    best
}

fn apply_shift(
    reference: &DynamicImage,
    implementation: &DynamicImage,
    dx: i32,
    dy: i32,
) -> DynamicImage {
    let (w, h) = reference.dimensions();
    let w = w as i32;
    let h = h as i32;

    let mut canvas = reference.to_rgba8();
    let impl_rgba = implementation.to_rgba8();

    let dst_x0 = dx.max(0);
    let dst_y0 = dy.max(0);
    let src_x0 = (-dx).max(0);
    let src_y0 = (-dy).max(0);
    let width = w - dst_x0 - src_x0;
    let height = h - dst_y0 - src_y0;

    if width <= 0 || height <= 0 {
        return implementation.clone();
    }

    for y in 0..height {
        let src_y = (src_y0 + y) as u32;
        let dst_y = (dst_y0 + y) as u32;
        for x in 0..width {
            let src_x = (src_x0 + x) as u32;
            let dst_x = (dst_x0 + x) as u32;
            let pixel = impl_rgba.get_pixel(src_x, src_y);
            canvas.put_pixel(dst_x, dst_y, *pixel);
        }
    }

    DynamicImage::ImageRgba8(canvas)
}
