mod linear_regression;
mod gradient_descent;

trait Regression<Params> {
    fn new(params: Params) -> Self;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}



#[cfg(test)]
mod tests {
    use crate::linear_regression::LinearRegression;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    

}
