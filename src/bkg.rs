use std::fmt;
pub const BKG_SET_TYPE: usize = 81; // BKG = Block gauge
static UPPER_BOUND_RECURSION_LIMIT: u64 = 10_000_000;

const EPSILON: f64 = 0.00001;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    LimitReached,
    FoundNothing,
    BlockSetIndexNegative,
    None,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::LimitReached => write!(
                f,
                "ERROR: Reached upperbound recursion limit. Try to lower max combinations"
            ),
            ErrorType::FoundNothing => write!(f, "ERROR: Search ended with no result"),
            ErrorType::BlockSetIndexNegative => write!(f, ""),
            ErrorType::None => write!(f, ""),
        }
    }
}

pub fn f_comp(x: f64, y: f64) -> bool {
    (x - y).abs() < EPSILON
}

fn push_range(target: &mut [f64], from: f64, to: f64, step: f64, offset: usize) {
    let mut idx: usize = offset;
    let mut i = from;
    while i < to || f_comp(i, to) {
        target[idx] = i;
        i += step;
        idx += 1;
    }
}

#[derive(Debug)]
pub struct Runtime {
    bkg_set: Vec<f64>,
    upperbound_count: u64,
    error: ErrorType,
    _sub_sets: Vec<f64>,
    _exclusions: Vec<f64>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            bkg_set: vec![0.0; BKG_SET_TYPE],
            upperbound_count: 0,
            error: ErrorType::None,
            _sub_sets: Vec::new(),
            _exclusions: Vec::new(),
        }
    }

    pub fn set_exclusions(&mut self, v: Vec<f64>) {
        self._exclusions = v;
    }

    fn bkg_find_rbt(&mut self, target: f64, current_sum: f64, mut bkg_set_idx: usize) -> bool {
        self.upperbound_count += 1;

        if self.upperbound_count >= UPPER_BOUND_RECURSION_LIMIT {
            self.error = ErrorType::LimitReached;
            return false;
        }

        if self.error != ErrorType::None {
            return false;
        }

        if f_comp(current_sum, target) {
            return true;
        }

        if current_sum > target {
            return false;
        }

        let Some(prev_bkg_set_idx) = bkg_set_idx.checked_sub(1) else {
            return false;
        };

        bkg_set_idx = prev_bkg_set_idx;

        for (j, _) in self._exclusions.iter().enumerate() {
            if f_comp(self.bkg_set[bkg_set_idx], self._exclusions[j]) {
                return self.bkg_find_rbt(target, current_sum, bkg_set_idx);
            }
        }

        self._sub_sets.push(self.bkg_set[bkg_set_idx]);
        if self.bkg_find_rbt(target, current_sum + self.bkg_set[bkg_set_idx], bkg_set_idx) {
            return true;
        }

        self._sub_sets.pop();
        self.bkg_find_rbt(target, current_sum, bkg_set_idx)
    }

    pub fn find_combination(
        &mut self,
        measure: f64,
        max_comb: usize,
    ) -> Vec<(ErrorType, Vec<f64>)> {
        let mut combinations = Vec::with_capacity(max_comb);

        for _ in 0..max_comb {
            self.error = ErrorType::None;
            self.upperbound_count = 0;
            self._sub_sets.clear();

            if self.bkg_find_rbt(measure, 0.0, 81) {
                combinations.push((self.error.clone(), self._sub_sets.clone()));
                self._exclusions.extend(&self._sub_sets);
            } else {
                let error = if self.error == ErrorType::None {
                    ErrorType::FoundNothing
                } else {
                    self.error.clone()
                };
                combinations.push((error, Vec::new()));
            }
        }

        combinations
    }
}

pub fn init_bkg_set() -> Runtime {
    let mut rnt = Runtime::new();
    push_range(&mut rnt.bkg_set, 0.1001, 0.1009, 0.0001, 0);
    push_range(&mut rnt.bkg_set, 0.1010, 0.1490, 0.0010, 9);
    push_range(&mut rnt.bkg_set, 0.0500, 0.9500, 0.0500, 58);
    push_range(&mut rnt.bkg_set, 1.0000, 4.0000, 1.0000, 77);
    rnt
}
