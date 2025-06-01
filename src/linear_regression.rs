use crate::Regression;

pub struct LinearRegression {

}

pub struct LinearRegressionParams {
    epoch: i32
}

impl Regression<LinearRegressionParams> for LinearRegression {
    fn new(_: LinearRegressionParams) -> Self {
        Self {}
    }
}