#[derive(Debug)]
pub struct TVM<Double> {
    pub i: Option<Double>,
    pub n: Option<Double>,
    pub t: Option<Double>,
    pub pv: Option<Double>,
    pub fv: Option<Double>,
}
    
impl<Double: std::fmt::Debug> TVM<Double> {
    pub fn pv(&self) {
        self.pv = Some(self.fv/(1+(self.i/self.n).pow(self.n * self.t)));
        println!("pv = {}", self.pv);
    }

    pub fn fetch(&self) {
        println!("{:#?}", self);
    }
}
