use std::f64::consts::{PI};

use point2::{Point2};
use cov2::{Cov2};

#[derive(Copy,Clone)]
pub struct Gauss2 {
    pub p:  f64,
    pub cen: Point2,
    pub cov: Cov2,
    pub norm: f64,
    pub pnorm: f64,
}

impl Gauss2 {
    pub fn new(p:f64, cen:&Point2, cov:&Cov2) {
        let norm = 1./( 2.0*PI*cov.det.sqrt() );
        let pnorm = p*norm;

        Gauss2 {
            p: p,
            cen: cen.clone(),
            cov: cov.clone(),
            norm: norm,
            pnorm: pnorm,
        }
    }
}
