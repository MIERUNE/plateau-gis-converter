//! Picks a Minecraft block from the colour of a polygon's texture.

use std::sync::LazyLock;

use image::RgbImage;

/// CIELab colour under the D65 white point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

/// Pixels whose channels are all at or below this are the atlas background.
const BACKGROUND_MAX: u8 = 8;
/// Below this many pixels after trimming, the median uses every non-background pixel.
const MIN_TRIMMED_PIXELS: usize = 20;

/// Blocks to choose from, with approximate average colours of their textures entered by hand.
const TEXTURE_PALETTE: &[(&str, [u8; 3])] = &[
    ("white_concrete", [207, 213, 214]),
    ("light_gray_concrete", [125, 125, 115]),
    ("gray_concrete", [55, 58, 62]),
    ("black_concrete", [8, 10, 15]),
    ("brown_concrete", [96, 60, 32]),
    ("red_concrete", [142, 33, 33]),
    ("orange_concrete", [224, 97, 1]),
    ("yellow_concrete", [241, 175, 21]),
    ("lime_concrete", [94, 169, 24]),
    ("green_concrete", [73, 91, 36]),
    ("cyan_concrete", [21, 119, 136]),
    ("light_blue_concrete", [36, 137, 199]),
    ("blue_concrete", [45, 47, 143]),
    ("purple_concrete", [100, 32, 156]),
    ("magenta_concrete", [169, 48, 159]),
    ("pink_concrete", [214, 101, 143]),
    ("white_terracotta", [210, 178, 161]),
    ("light_gray_terracotta", [135, 107, 98]),
    ("gray_terracotta", [58, 42, 36]),
    ("brown_terracotta", [77, 51, 36]),
    ("red_terracotta", [143, 61, 47]),
    ("orange_terracotta", [162, 84, 38]),
    ("yellow_terracotta", [186, 133, 35]),
    ("terracotta", [152, 94, 68]),
    ("cyan_terracotta", [87, 91, 91]),
    ("light_blue_terracotta", [114, 109, 138]),
    ("stone", [126, 126, 126]),
    ("smooth_stone", [159, 159, 159]),
    ("andesite", [136, 136, 137]),
    ("polished_andesite", [132, 135, 134]),
    ("diorite", [189, 188, 189]),
    ("quartz_block", [236, 230, 223]),
    ("sandstone", [216, 203, 156]),
    ("smooth_sandstone", [224, 214, 170]),
    ("bricks", [151, 98, 83]),
    ("stone_bricks", [122, 121, 122]),
    ("deepslate_tiles", [55, 55, 55]),
    ("mud_bricks", [137, 104, 79]),
    ("calcite", [223, 224, 220]),
    ("tuff", [108, 109, 103]),
];

static PALETTE_LAB: LazyLock<Vec<(&'static str, Lab)>> = LazyLock::new(|| {
    TEXTURE_PALETTE
        .iter()
        .map(|&(name, rgb)| (name, srgb_to_lab(rgb)))
        .collect()
});

pub fn srgb_to_lab(rgb: [u8; 3]) -> Lab {
    let [r, g, b] = rgb.map(|c| {
        let c = c as f32 / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    });
    let x = (0.4124564 * r + 0.3575761 * g + 0.1804375 * b) / 0.95047;
    let y = 0.2126729 * r + 0.7151522 * g + 0.072175 * b;
    let z = (0.0193339 * r + 0.119192 * g + 0.9503041 * b) / 1.08883;
    let (fx, fy, fz) = (lab_f(x), lab_f(y), lab_f(z));
    Lab {
        l: 116.0 * fy - 16.0,
        a: 500.0 * (fx - fy),
        b: 200.0 * (fy - fz),
    }
}

fn lab_f(t: f32) -> f32 {
    const DELTA: f32 = 6.0 / 29.0;
    if t > DELTA * DELTA * DELTA {
        t.cbrt()
    } else {
        t / (3.0 * DELTA * DELTA) + 4.0 / 29.0
    }
}

/// Colour difference ΔE00 (Sharma, Wu and Dalal, 2005).
pub fn ciede2000(c1: Lab, c2: Lab) -> f32 {
    let (l1, a1, b1) = (c1.l as f64, c1.a as f64, c1.b as f64);
    let (l2, a2, b2) = (c2.l as f64, c2.a as f64, c2.b as f64);
    let pow25_7 = 25f64.powi(7);

    let c_bar = (a1.hypot(b1) + a2.hypot(b2)) / 2.0;
    let g = 0.5 * (1.0 - (c_bar.powi(7) / (c_bar.powi(7) + pow25_7)).sqrt());
    let (a1p, a2p) = ((1.0 + g) * a1, (1.0 + g) * a2);
    let (c1p, c2p) = (a1p.hypot(b1), a2p.hypot(b2));
    let hue = |b: f64, ap: f64| {
        if b == 0.0 && ap == 0.0 {
            0.0
        } else {
            b.atan2(ap).to_degrees().rem_euclid(360.0)
        }
    };
    let (h1p, h2p) = (hue(b1, a1p), hue(b2, a2p));

    let delta_lp = l2 - l1;
    let delta_cp = c2p - c1p;
    let chroma_product = c1p * c2p;
    let delta_hp = if chroma_product == 0.0 {
        0.0
    } else if (h2p - h1p).abs() <= 180.0 {
        h2p - h1p
    } else if h2p - h1p > 180.0 {
        h2p - h1p - 360.0
    } else {
        h2p - h1p + 360.0
    };
    let delta_big_hp = 2.0 * chroma_product.sqrt() * (delta_hp.to_radians() / 2.0).sin();

    let l_bar_p = (l1 + l2) / 2.0;
    let c_bar_p = (c1p + c2p) / 2.0;
    let h_bar_p = if chroma_product == 0.0 {
        h1p + h2p
    } else if (h1p - h2p).abs() <= 180.0 {
        (h1p + h2p) / 2.0
    } else if h1p + h2p < 360.0 {
        (h1p + h2p + 360.0) / 2.0
    } else {
        (h1p + h2p - 360.0) / 2.0
    };

    let cos_deg = |deg: f64| deg.to_radians().cos();
    let t = 1.0 - 0.17 * cos_deg(h_bar_p - 30.0)
        + 0.24 * cos_deg(2.0 * h_bar_p)
        + 0.32 * cos_deg(3.0 * h_bar_p + 6.0)
        - 0.20 * cos_deg(4.0 * h_bar_p - 63.0);
    let delta_theta = 30.0 * (-((h_bar_p - 275.0) / 25.0).powi(2)).exp();
    let r_c = 2.0 * (c_bar_p.powi(7) / (c_bar_p.powi(7) + pow25_7)).sqrt();
    let l_dist = (l_bar_p - 50.0).powi(2);
    let s_l = 1.0 + 0.015 * l_dist / (20.0 + l_dist).sqrt();
    let s_c = 1.0 + 0.045 * c_bar_p;
    let s_h = 1.0 + 0.015 * c_bar_p * t;
    let r_t = -(2.0 * delta_theta).to_radians().sin() * r_c;

    let (dl, dc, dh) = (delta_lp / s_l, delta_cp / s_c, delta_big_hp / s_h);
    (dl * dl + dc * dc + dh * dh + r_t * dc * dh).sqrt() as f32
}

/// Colours of the pixels whose centres fall inside a UV ring.
///
/// A UV maps to pixel `(u * width, (1 - v) * height)`, as v runs up from the bottom of the image.
pub fn polygon_pixels(img: &RgbImage, uv_ring: &[[f64; 2]]) -> Vec<[u8; 3]> {
    let (width, height) = (img.width() as f64, img.height() as f64);
    let ring: Vec<[f64; 2]> = uv_ring
        .iter()
        .map(|&[u, v]| [u * width, (1.0 - v) * height])
        .collect();

    let (mut min, mut max) = ([f64::MAX; 2], [f64::MIN; 2]);
    for p in &ring {
        for i in 0..2 {
            min[i] = min[i].min(p[i]);
            max[i] = max[i].max(p[i]);
        }
    }
    let (x0, x1) = (
        min[0].floor().clamp(0.0, width) as u32,
        max[0].ceil().clamp(0.0, width) as u32,
    );
    let (y0, y1) = (
        min[1].floor().clamp(0.0, height) as u32,
        max[1].ceil().clamp(0.0, height) as u32,
    );

    let mut pixels = Vec::new();
    for y in y0..y1 {
        for x in x0..x1 {
            if ring_contains(&ring, [x as f64 + 0.5, y as f64 + 0.5]) {
                pixels.push(img.get_pixel(x, y).0);
            }
        }
    }
    pixels
}

/// Even-odd test; the ring may or may not repeat its first point at the end.
fn ring_contains(ring: &[[f64; 2]], [x, y]: [f64; 2]) -> bool {
    let mut inside = false;
    for (a, b) in ring.iter().zip(ring.iter().cycle().skip(1)) {
        if (a[1] > y) != (b[1] > y) && x < (b[0] - a[0]) * (y - a[1]) / (b[1] - a[1]) + a[0] {
            inside = !inside;
        }
    }
    inside
}

/// Per-channel Lab median of the pixels, ignoring the black atlas background, the darkest 30%
/// (shadows) and the brightest 3% (blown highlights). `None` when every pixel is background.
pub fn representative_color(pixels: &[[u8; 3]]) -> Option<Lab> {
    let mut colors: Vec<Lab> = pixels
        .iter()
        .filter(|p| p.iter().any(|&c| c > BACKGROUND_MAX))
        .map(|&p| srgb_to_lab(p))
        .collect();
    if colors.is_empty() {
        return None;
    }

    colors.sort_unstable_by(|p, q| p.l.total_cmp(&q.l));
    let n = colors.len();
    let trimmed = &colors[n * 30 / 100..n - n * 3 / 100];
    let kept = if trimmed.len() >= MIN_TRIMMED_PIXELS {
        trimmed
    } else {
        &colors[..]
    };

    Some(Lab {
        l: median(kept.iter().map(|c| c.l)),
        a: median(kept.iter().map(|c| c.a)),
        b: median(kept.iter().map(|c| c.b)),
    })
}

fn median(values: impl Iterator<Item = f32>) -> f32 {
    let mut values: Vec<f32> = values.collect();
    let mid = values.len() / 2;
    *values.select_nth_unstable_by(mid, f32::total_cmp).1
}

/// The palette block whose colour is closest to `color` by CIEDE2000.
pub fn nearest_block(color: Lab) -> &'static str {
    PALETTE_LAB
        .iter()
        .map(|&(name, block)| (name, ciede2000(color, block)))
        .min_by(|(_, d1), (_, d2)| d1.total_cmp(d2))
        .map(|(name, _)| name)
        .expect("the palette is not empty")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lab(l: f32, a: f32, b: f32) -> Lab {
        Lab { l, a, b }
    }

    fn repeat(color: [u8; 3], count: usize) -> Vec<[u8; 3]> {
        vec![color; count]
    }

    #[test]
    fn ciede2000_matches_published_pairs() {
        // Sharma, Wu and Dalal (2005), Table 1: pairs 1, 4, 7, 9, 12, 17 and 25.
        let pairs = [
            (
                lab(50.0, 2.6772, -79.7751),
                lab(50.0, 0.0, -82.7485),
                2.0425,
            ),
            (
                lab(50.0, -1.3802, -84.2814),
                lab(50.0, 0.0, -82.7485),
                1.0000,
            ),
            (lab(50.0, 0.0, 0.0), lab(50.0, -1.0, 2.0), 2.3669),
            (lab(50.0, 2.49, -0.001), lab(50.0, -2.49, 0.0009), 7.1792),
            (lab(50.0, 2.49, -0.001), lab(50.0, -2.49, 0.0012), 7.2195),
            (lab(50.0, 2.5, 0.0), lab(73.0, 25.0, -18.0), 27.1492),
            (
                lab(60.2574, -34.0099, 36.2677),
                lab(60.4626, -34.1751, 39.4387),
                1.2644,
            ),
        ];
        for (c1, c2, expected) in pairs {
            let actual = ciede2000(c1, c2);
            assert!(
                (actual - expected).abs() < 1e-4,
                "{c1:?} {c2:?}: {actual} != {expected}"
            );
            assert!((ciede2000(c2, c1) - expected).abs() < 1e-4);
        }
    }

    #[test]
    fn srgb_to_lab_matches_reference_values() {
        let white = srgb_to_lab([255, 255, 255]);
        assert!((white.l - 100.0).abs() < 0.01 && white.a.abs() < 0.01 && white.b.abs() < 0.01);
        // sRGB red is L* 53.24, a* 80.09, b* 67.20.
        let red = srgb_to_lab([255, 0, 0]);
        assert!((red.l - 53.24).abs() < 0.05);
        assert!((red.a - 80.09).abs() < 0.05);
        assert!((red.b - 67.20).abs() < 0.05);
    }

    #[test]
    fn colour_near_a_palette_entry_selects_that_block() {
        assert_eq!(nearest_block(srgb_to_lab([150, 97, 82])), "bricks");
        assert_eq!(
            nearest_block(srgb_to_lab([205, 212, 214])),
            "white_concrete"
        );
        assert_eq!(
            nearest_block(srgb_to_lab([240, 176, 20])),
            "yellow_concrete"
        );
    }

    #[test]
    fn background_pixels_are_ignored() {
        let facade = [151, 98, 83];
        let mut pixels = repeat([0, 0, 0], 300);
        pixels.extend(repeat([8, 8, 8], 300));
        pixels.extend(repeat(facade, 100));

        let color = representative_color(&pixels).unwrap();
        assert!(ciede2000(color, srgb_to_lab(facade)) < 0.5);
        assert_eq!(nearest_block(color), "bricks");
    }

    #[test]
    fn only_background_pixels_give_no_colour() {
        assert_eq!(representative_color(&repeat([0, 0, 0], 50)), None);
        assert_eq!(representative_color(&[]), None);
    }

    #[test]
    fn shadows_and_highlights_do_not_move_the_colour() {
        // Shadows outnumber the facade, so a plain median would land on a shadow.
        let facade = [143, 61, 47];
        let mut pixels = repeat([20, 20, 20], 110);
        pixels.extend(repeat(facade, 86));
        pixels.extend(repeat([255, 255, 255], 4));

        let color = representative_color(&pixels).unwrap();
        assert!(ciede2000(color, srgb_to_lab(facade)) < 0.5);
        assert_eq!(nearest_block(color), "red_terracotta");
    }

    #[test]
    fn polygon_pixels_selects_pixels_inside_the_uv_triangle() {
        // Each pixel encodes its own position, so the result names the pixels it took.
        let img = RgbImage::from_fn(4, 4, |x, y| image::Rgb([x as u8, y as u8, 1]));
        // In image space (v flipped) the corners are (0, 4), (2, 4) and (0, 0), and the
        // hypotenuse x = y / 2 passes no pixel centre.
        let triangle = [[0.0, 0.0], [0.5, 0.0], [0.0, 1.0], [0.0, 0.0]];

        let mut taken: Vec<(u8, u8)> = polygon_pixels(&img, &triangle)
            .into_iter()
            .map(|[x, y, _]| (x, y))
            .collect();
        taken.sort();

        assert_eq!(taken, vec![(0, 1), (0, 2), (0, 3), (1, 3)]);
    }

    #[test]
    fn polygon_pixels_clamps_uvs_outside_the_image() {
        let img = RgbImage::from_pixel(2, 2, image::Rgb([9, 9, 9]));
        let square = [[-1.0, -1.0], [2.0, -1.0], [2.0, 2.0], [-1.0, 2.0]];
        assert_eq!(polygon_pixels(&img, &square).len(), 4);
    }
}
