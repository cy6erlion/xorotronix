pub mod theory;

pub struct Resistance;

impl Resistance {
    /// A circuit with a number of resistances connected in series,
    /// the equivalent resistance of the circuit is the sum of the
    /// individual resistances.
    pub fn series(resistors: Vec<f64>) -> f64 {
        resistors.iter().sum()
    }

    /// By putting resistors in parallel, you always get a smaller
    /// resistor.
    pub fn parallel(resistors: Vec<f64>) -> f64 {
        let mut total_resistance = 0.0;

        for r in resistors {
            total_resistance += 1.0 / r;
        }
        total_resistance.powi(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resistance_series() {
        let resistance_values = vec![5.0, 10.0];
        let total_resistance = Resistance::series(resistance_values);
        assert_eq!(total_resistance, 15.0);
    }

    #[test]
    fn test_resistance_parallel() {
        let resistance_values = vec![5.0, 20.0, 8.0];
        let total_resistance = Resistance::parallel(resistance_values);
        assert_eq!(total_resistance, 2.6666666666666665);
    }
}
