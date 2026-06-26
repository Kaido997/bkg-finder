use std::fmt;
pub const BKG_SET_TYPE: usize = 81; // BKG = Block gauge
static UPPER_BOUND_RECURSION_LIMIT: u64 = 100000000;

const EPSILON: f64 = 0.00001;

#[derive(Debug, Clone)]
pub enum ErrorType {
    LimitReached,
    None,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::LimitReached => write!(
                f,
                "Reached upperbound recursion limit. Try to lower max combinations"
            ),
            ErrorType::None => write!(f, ""),
        }
    }
}

pub fn f_comp(x: f64, y: f64) -> bool 
{
    (x - y).abs() < EPSILON
}

fn push_range(target: &mut [f64], from: f64, to: f64, step: f64, offset: usize) 
{
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
}

impl Runtime {
    fn new() -> Self {
        Self {
            bkg_set: vec![0.0; BKG_SET_TYPE],
            upperbound_count: 0,
            error: ErrorType::None,
        }
    }

    fn bkg_find_rbt(
        &mut self,
        target: f64,
        current_sum: f64,
        mut index: usize,
        sub_sets: &mut [f64],
        sub_sets_size: usize,
        exclusions: &mut [f64],
        ex_size: usize,
    ) -> bool {
        self.upperbound_count += 1;

        if self.upperbound_count >= UPPER_BOUND_RECURSION_LIMIT {
            self.error = ErrorType::LimitReached;
            return true;
        }

        if current_sum > target {
            return false;
        }

        if index > 0 {
            index -= 1;
        } else {
            return false;
        }

        for j in 0..ex_size {
            if f_comp(self.bkg_set[index], exclusions[j]) {
                return self.bkg_find_rbt(
                    target,
                    current_sum,
                    index,
                    sub_sets,
                    sub_sets_size,
                    exclusions,
                    ex_size,
                );
            }
        }


        sub_sets[sub_sets_size] = self.bkg_set[index];

        if f_comp(current_sum, target) {
            sub_sets[sub_sets_size] = 0.0;
            return true;
        }

        if self.bkg_find_rbt(
            target,
            current_sum + self.bkg_set[index],
            index,
            sub_sets,
            sub_sets_size + 1,
            exclusions,
            ex_size,
        ) {
            return true;
        }

        if self.bkg_find_rbt(
            target,
            current_sum,
            index,
            sub_sets,
            sub_sets_size,
            exclusions,
            ex_size,
        ) {
            return true;
        }
        false
    }

    pub fn find_combination(
        &mut self,
        measure: f64,
        max_comb: usize,
        exclusions: &mut [f64],
        mut ex_size: usize,
    ) -> Result<Vec<Vec<f64>>, ErrorType> {
        let mut combinations: Vec<Vec<f64>> = vec![vec![0.0; 16]; max_comb];
        self.error = ErrorType::None;
        self.upperbound_count = 0;

        for item in combinations.iter_mut().take(max_comb) {
            let mut sub_sets: Vec<f64> = vec![0.0; 16];

            if self.bkg_find_rbt(
                measure,
                0.0,
                80,
                &mut sub_sets,
                0,
                exclusions,
                ex_size,
            ) {
                let mut counter: usize = 0;
                while sub_sets[counter] > EPSILON {
                    let v: f64 = sub_sets[counter];
                    exclusions[ex_size + counter] = v;
                    item[counter] = v;
                    counter += 1;
                }
                ex_size += counter;
            }
        }

        match self.error {
            ErrorType::LimitReached => Err(ErrorType::LimitReached),
            ErrorType::None => Ok(combinations),
        }
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
