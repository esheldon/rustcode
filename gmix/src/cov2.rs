
#[derive(Debug,Copy,Clone)]
pub struct Cov2 {
    pub ixx: f64,
    pub ixy: f64,
    pub iyy: f64,

    // det may be zero, but we may only be adding this covariance
    // matrix to another during convolution, in which case it is OK
    // so we delay setting it
    pub det_is_set: i32,
    pub det: f64,

    pub dxx: f64,
    pub dxy: f64,
    pub dyy: f64,
}

pub enum Cov2Error {SingularMatrix}

impl Cov2 {
    pub fn new(ixx: f64, ixy: f64, iyy: f64) -> Cov2 {
        Cov2 {
            ixx: ixx,
            ixy: ixy,
            iyy: iyy,
            det_is_set: 0,
            det: 0.0,
            dxx: 0.0,
            dxy: 0.0,
            dyy: 0.0,
        }
    }

    pub fn set_det(&mut self) -> Result<(), Cov2Error> {
        let det = self.ixx*self.iyy - self.ixy*self.ixy;
        
        if det == 0.0 {
            return Err(Cov2Error::SingularMatrix);
        }

        self.det = det;

        let idet = 1.0/self.det;
        self.dxx = self.ixx*idet;
        self.dxy = self.ixy*idet;
        self.dyy = self.iyy*idet;

        self.det_is_set=1;

        Ok( () )
    }
}
