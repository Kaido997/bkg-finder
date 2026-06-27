use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::bkg;

#[derive(Serialize)]
struct CombinationResult {
    error: String,
    blocks: Vec<f64>,
    sum: f64,
    delta: f64,
}

#[derive(Serialize)]
struct SearchResult {
    target: f64,
    max_combinations: u8,
    exclusions: Vec<f64>,
    combinations: Vec<CombinationResult>,
}

#[wasm_bindgen]
pub fn find_blocks(
    target: f64,
    max_combinations: u8,
    exclusions: &str,
) -> Result<JsValue, JsValue> {
    if !target.is_finite() || target <= 0.0 {
        return Err(JsValue::from_str("Measurement must be a positive number."));
    }

    if max_combinations == 0 {
        return Err(JsValue::from_str("Result count must be at least 1."));
    }

    let exclusions = parse_exclusions(exclusions)?;
    let mut runtime = bkg::init_bkg_set();
    runtime.set_exclusions(exclusions.clone());

    let combinations = runtime
        .find_combination(target, usize::from(max_combinations))
        .into_iter()
        .map(|(error, blocks)| {
            let sum = blocks.iter().sum::<f64>();
            CombinationResult {
                error: error.to_string(),
                blocks,
                sum,
                delta: sum - target,
            }
        })
        .collect::<Vec<_>>();

    serde_wasm_bindgen::to_value(&SearchResult {
        target,
        max_combinations,
        exclusions,
        combinations,
    })
    .map_err(|error| JsValue::from_str(&error.to_string()))
}

fn parse_exclusions(raw: &str) -> Result<Vec<f64>, JsValue> {
    raw.split(|c: char| c == ',' || c == ';' || c.is_whitespace())
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.parse::<f64>()
                .map_err(|_| JsValue::from_str(&format!("Invalid exclusion: {part}")))
        })
        .collect()
}
