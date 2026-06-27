use bkg_finder::bkg;

const EPSILON: f64 = 0.00001;

fn sum_combination(combo: &[f64]) -> f64 {
    combo.iter().sum()
}

fn check_measure(measure: f64) {
    let mut rt = bkg::init_bkg_set();
    let combinations = rt.find_combination(measure, 1);

    assert_eq!(combinations.len(), 1);
    assert_eq!(combinations[0].0, bkg::ErrorType::None);

    let sum = sum_combination(&combinations[0].1);
    assert!(
        (sum - measure).abs() < EPSILON,
        "measure={measure:.4} sum={sum:.4} diff={:.6}",
        (sum - measure).abs()
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

#[test]
fn test_measure_0_1001() {
    check_measure(0.1001);
}

#[test]
fn test_measure_0_1020() {
    check_measure(0.1020);
}

#[test]
fn test_max_returns_multiple_combinations() {
    let mut runtime = bkg::init_bkg_set();
    let combinations = runtime.find_combination(4.0000, 2);

    assert_eq!(combinations.len(), 2);

    for (error, combination) in combinations {
        assert_eq!(error, bkg::ErrorType::None);

        let sum: f64 = combination.iter().sum();
        assert!(
            (sum - 4.0000).abs() < EPSILON,
            "sum={sum:.4} combination={combination:?}"
        );
    }
}

#[test]
fn test_excludes_requested_block() {
    let mut runtime = bkg::init_bkg_set();
    runtime.set_exclusions(vec![4.0000]);

    let combinations = runtime.find_combination(4.0000, 1);

    assert_eq!(combinations.len(), 1);
    assert_eq!(combinations[0].0, bkg::ErrorType::None);
    assert!(
        !combinations[0]
            .1
            .iter()
            .any(|block| (*block - 4.0000).abs() < EPSILON),
        "combination should not contain excluded block: {:?}",
        combinations[0].1
    );

    let sum: f64 = combinations[0].1.iter().sum();
    assert!((sum - 4.0000).abs() < EPSILON, "sum={sum:.4}");
}

#[test]
fn test_failed_run_reports_error() {
    let mut runtime = bkg::init_bkg_set();
    let combinations = runtime.find_combination(0.0001, 1);

    assert_eq!(combinations.len(), 1);
    assert_eq!(combinations[0].0, bkg::ErrorType::FoundNothing);
    assert!(combinations[0].1.is_empty());
}
#[test]
fn test_repeated_searches_do_not_reuse_generated_exclusions() {
    let mut runtime = bkg::init_bkg_set();

    let first = runtime.find_combination(4.0000, 1);
    let second = runtime.find_combination(4.0000, 1);

    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 1);
    assert_eq!(first[0].0, bkg::ErrorType::None);
    assert_eq!(second[0].0, bkg::ErrorType::None);
    assert_eq!(first[0].1, second[0].1);
    assert!((sum_combination(&second[0].1) - 4.0000).abs() < EPSILON);
}
