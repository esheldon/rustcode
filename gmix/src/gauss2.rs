use std::f64::consts::{PI};

use point2::{Point2};
use cov2::{Cov2,Cov2Error};

// math functions are implemented *methods* on the numbers, so
// we can't just write equations.  Define these functions so that we can

#[inline(always)]
fn exp(x: f64) -> f64 { x.exp() }

#[inline(always)]
fn sqrt(x: f64) -> f64 { x.sqrt() }

#[derive(Copy,Clone)]
pub struct Gauss2 {
    pub p:  f64,
    pub cen: Point2,
    pub cov: Cov2,

    pub norm_is_set: bool,
    pub norm: f64,
    pub pnorm: f64,
}

impl Gauss2 {
    pub fn new(p:f64, cen: &Point2, cov: &Cov2) -> Gauss2 {
        Gauss2 {
            p: p,
            cen: *cen,
            cov: *cov,

            norm_is_set: false,
            norm: -9999.0,
            pnorm: -9999.0,
        }
    }
    pub fn set_norm(&mut self) -> Result<(), Cov2Error> {
        if !self.cov.det_is_set {
            try!(self.cov.set_det());
        }

        let det = self.cov.det;
        self.norm = 1./( 2.0*PI*sqrt(det) );
        self.pnorm = self.p*self.norm;
        self.norm_is_set=true;

        Ok( () )
    }

    pub fn eval(&self, pt: &Point2) -> f64 {

        // we cannot evaluate until we have set the cov
        // determinant and d* convenience fields, as well
        // as the norm/pnorm fields. But we don't want
        // this evaluation to return a Result or Option
        
        assert!(self.norm_is_set);

        let xd = pt.x-self.cen.x;
        let yd = pt.y-self.cen.y;

        let chi2 =        self.cov.dxx*yd*yd
                    +     self.cov.dyy*xd*xd
                    - 2.0*self.cov.dxy*yd*xd ;

        let val = match chi2 > 0.0 {
            true => {
                self.pnorm*exp(-0.5*chi2)
            },
            false => 0.0,
        };

        val
    }

}
