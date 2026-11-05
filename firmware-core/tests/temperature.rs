use firmware_core::temperature::estimate_c;

#[test]
fn midscale_is_about_twenty_five_celsius() {
    assert_eq!(estimate_c(2047), Some(25));
    assert_eq!(estimate_c(2048), Some(25));
}

#[test]
fn rails_and_out_of_range_counts_are_invalid() {
    for count in [0, 1, 4094, 4095, 4096, u16::MAX] {
        assert_eq!(estimate_c(count), None, "count {count}");
    }
}

#[test]
fn known_counts_follow_the_beta_curve() {
    assert_eq!(estimate_c(1000), Some(-2));
    assert_eq!(estimate_c(3000), Some(54));
}

#[test]
fn integer_approximation_stays_within_one_degree_of_the_beta_curve() {
    for count in 1..4095 {
        if let Some(estimate) = estimate_c(count) {
            let ratio = f64::from(4095 - count) / f64::from(count);
            let reference = 1.0 / (1.0 / 298.15 + ratio.ln() / 3380.0) - 273.15;
            assert!(
                (f64::from(estimate) - reference).abs() <= 1.0,
                "count {count}: {estimate} vs {reference}"
            );
        }
    }
}
