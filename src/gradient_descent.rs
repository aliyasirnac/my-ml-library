pub struct GradientDescent {
    w: f64,
    b: f64,
    a: f64
}

impl GradientDescent {
    pub fn new(w: f64, b: f64, a:f64) -> Self {
        Self { w,b, a }
    }

    /// # Params
    /// -   `self`: struct params 
    /// 
    /// -  epoch
    /// 
    /// -  `f_d`: derivative of loss function, you need to pass
    fn update_params(&mut self,epoch: i32, f_d: fn(w:f64,b:f64)->f64) { // TODO: weights must be vector, we need to update this later
        self.w = self.w - self.a*f_d(self.w,self.b);
        self.b = self.b - self.a*f_d(self.w,self.b);
    }    
}
