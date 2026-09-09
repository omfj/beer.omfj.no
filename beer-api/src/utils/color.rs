use rand::RngExt;

pub(crate) fn generate_soft_color() -> String {
    let mut rng = rand::rng();
    hsl_to_hex(
        f64::from(rng.random_range(0..360)),
        f64::from(rng.random_range(30..60)),
        f64::from(rng.random_range(75..95)),
    )
}

fn hsl_to_hex(hue: f64, saturation: f64, lightness: f64) -> String {
    let saturation = saturation / 100.0;
    let lightness = lightness / 100.0;
    let a = saturation * lightness.min(1.0 - lightness);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let component = |n: f64| {
        let k = (n + hue / 30.0) % 12.0;
        (255.0 * (lightness - a * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0))).round() as u8
    };
    format!(
        "#{:02x}{:02x}{:02x}",
        component(0.0),
        component(8.0),
        component(4.0)
    )
}
