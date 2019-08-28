use cassowary::{Solver, Variable, Expression};
use cassowary::WeightedRelation::{GE, EQ};
use cassowary::strength::{REQUIRED, WEAK};
use std::collections::HashMap;

pub struct Langbardo(pub f64);

impl Langbardo {
    pub fn fit(xs: &[f64], ys:&[f64], quantile: f64) -> Option<Self> {
        assert!(quantile >= 0f64 && quantile <= 1f64);

        let mut solver = Solver::new();

        let mut names = HashMap::new();

        let k = Variable::new();
        let b = Variable::new();
        names.insert(k, "k".to_string());
        names.insert(b, "b".to_string());

        let mut biases = Expression::new(Vec::new(), 0f64);

        for (index, x) in xs.iter().enumerate() {
            let y = ys[index];
            let x = x.clone();

            let u_pos = Variable::new();
            let u_neg = Variable::new();
            names.insert(u_pos, format!("u_pos{}", index));
            names.insert(u_neg, format!("u_neg{}", index));

            solver.add_constraints(&[
                u_pos |GE(REQUIRED)| 0.0,
                u_neg |GE(REQUIRED)| 0.0,
                k * x + u_pos - u_neg |EQ(REQUIRED)| y,
            ]).unwrap();

            biases = biases + u_pos * quantile + (1.0-quantile) * u_neg;
        }

        let loss = Variable::new();
        names.insert(loss, "loss".to_string());
        solver.add_constraints(&[
            loss |EQ(REQUIRED)| biases,
            loss|EQ(WEAK)| 0.0,
        ]).unwrap();

        for &(ref var, ref val) in solver.fetch_changes() {
            if &names[var] == "k" {
                return Some(Langbardo(val.clone()))
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::abs;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn linear_fit() {
        let result = Langbardo::fit(&[1.0, 2.0, 3.0], &[2.0, 4.0, 6.0], 0.5);
        assert_eq!(result.unwrap().0, 2.0);
    }

    #[test]
    fn hand_calculated_data() {
        let result = Langbardo::fit(&[1.0, 2.0, 3.0], &[2.0, 4.0, 4.0], 0.5);
        assert_approx_eq!(result.unwrap().0, 4f64/3f64);

        let result = Langbardo::fit(&[1.0, 2.0, 3.0], &[2.0, 4.0, 4.0], 0.6);
        assert_eq!(result.unwrap().0, 2.0);
    }
}
