#[derive(Debug,Copy,Clone)]
pub struct Cov2 {
    pub ixx: f64,
    pub ixy: f64,
    pub iyy: f64,

    pub det: f64,

    pub dxx: f64,
    pub dxy: f64,
    pub dyy: f64,
}

impl Cov2 {
    pub fn new(ixx: f64, ixy: f64, iyy: f64) -> Option<Cov2> {
        let det = ixx*iyy - ixy*ixy;
        
        if det == 0.0 {
            return None;
        }

        let idet = 1.0/det;
        let dxx = ixx*idet;
        let dxy = ixy*idet;
        let dyy = iyy*idet;

        Some(
            Cov2 {
                ixx: ixx,
                ixy: ixy,
                iyy: iyy,
                det: det,
                dxx: dxx,
                dxy: dxy,
                dyy: dyy,
            }
        )
    }
}
