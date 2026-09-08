const FALLBACK_POINTS: f64 = 0.5;

pub fn calculate_drink_points(volume_ml: Option<f64>, abv: Option<f64>, multiplier: f64) -> f64 {
    let (Some(volume_ml), Some(abv)) = (volume_ml, abv) else {
        return FALLBACK_POINTS;
    };

    if !volume_ml.is_finite()
        || !abv.is_finite()
        || !multiplier.is_finite()
        || volume_ml <= 0.0
        || abv <= 0.0
    {
        return FALLBACK_POINTS;
    }

    let score = (volume_ml / 1000.0) * (abv / 100.0) * 0.789 * 1000.0 / 10.0 * multiplier;

    if score.is_finite() {
        round_to_one_decimal(score)
    } else {
        FALLBACK_POINTS
    }
}

pub fn round_to_one_decimal(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use super::calculate_drink_points;

    fn assert_points(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn calculates_points_for_a_half_liter_beer() {
        assert_points(calculate_drink_points(Some(500.0), Some(5.0), 1.0), 2.0);
    }

    #[test]
    fn applies_the_drink_type_multiplier() {
        assert_points(calculate_drink_points(Some(40.0), Some(40.0), 2.0), 2.5);
    }

    #[test]
    fn uses_fallback_points_for_missing_or_invalid_values() {
        assert_points(calculate_drink_points(None, Some(5.0), 1.0), 0.5);
        assert_points(calculate_drink_points(Some(500.0), None, 1.0), 0.5);
        assert_points(calculate_drink_points(Some(0.0), Some(5.0), 1.0), 0.5);
        assert_points(calculate_drink_points(Some(f64::NAN), Some(5.0), 1.0), 0.5);
    }
}
