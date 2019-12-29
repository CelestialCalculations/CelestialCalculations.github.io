pub trait CanConvertUnit {
    type Unit;

    fn convert<'a>(
        &self,
        to_unit: Self::Unit,
    ) -> Result<Box<dyn HasConvertableUnit<Unit = Self::Unit>>, &'a str>;
}

pub trait HasConvertableUnit {
    type Unit;

    fn scalar(&self) -> f64;
    fn unit(&self) -> &Self::Unit;
    fn convert_scalar<'a>(&self, to_unit: &Self::Unit) -> Result<f64, &'a str>;
}

pub trait MetricScale<T: std::ops::Mul> {
    fn pow10(&self, power: i32) -> T;
}

impl MetricScale<f64> for f64 {
    fn pow10(&self, n: i32) -> f64 {
        let scale = (10.0 as f64).powi(n);
        self * scale
    }
}
