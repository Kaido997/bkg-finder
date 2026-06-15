use bkg_finder::bkg;

const EPSILON: f64 = 0.00001;

fn sum_combination(combo: &[f64]) -> f64 {
    combo.iter().take_while(|&&x| x > EPSILON).sum()
}

fn check_measure(measure: f64) {
    let mut rt = bkg::init_bkg_set();
    let mut exclusions = vec![0.0; 64];
    let combinations = rt
        .find_combination(measure, 1, &mut exclusions, 0)
        .expect("find_combination failed");

    let sum = sum_combination(&combinations[0]);
    let diff = (sum - measure).abs();
    assert!(
        diff < EPSILON,
        "measure={measure:.4} sum={sum:.4} diff={diff:.6} blocks={:?}",
        &combinations[0][..combinations[0].iter().take_while(|&&x| x > EPSILON).count()]
    );
}

#[test]
fn test_measure_4_0000() {
    check_measure(4.0000);
}

#[test]
fn test_measure_3_0000() {
    check_measure(3.0000);
}

#[test]
fn test_measure_0_2500() {
    check_measure(0.2500);
}

#[test]
fn test_measure_0_8000() {
    check_measure(0.8000);
}

#[test]
fn test_measure_1_1001() {
    check_measure(1.1001);
}
