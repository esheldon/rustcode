extern crate gmix;

use gmix::point2::{Point2};
use gmix::cov2::{Cov2};
use gmix::gauss2::{Gauss2};

#[test]
fn test_cov2() {
    let ixx=3.5;
    let ixy=0.2;
    let iyy=1.1;
    let det = ixx*iyy - ixy*ixy;
    let mut cov = Cov2::new(ixx, ixy, iyy);

    match cov.set_det() {
        Ok(_) => {},
        Err(_) => panic!("should not happen!"),
    }

    assert_eq!(cov.det, det);

}

// could not get [should_fail] to work, so just inverted
#[test]
fn test_cov2_bad() {
    let ixx=1.0;
    let ixy=1.0;
    let iyy=1.0;
    let mut cov = Cov2::new(ixx, ixy, iyy);

    assert!( match cov.set_det() {
        Err(_) => true,
        Ok(_) => false,
    });
}


#[test]
fn test_gauss2() {
    let x=1.5;
    let y=2.5;
    let cen = Point2 {x: x, y: y};

    let ixx=3.5;
    let ixy=0.2;
    let iyy=1.1;
    let det = ixx*iyy - ixy*ixy;
    let idet=1.0/det;
    let cov = Cov2::new(ixx, ixy, iyy);

    let p=100.0;

    // a copy of cen and cov will be made
    // make mutable
    let mut g = Gauss2::new(p, &cen, &cov);

    assert_eq!(g.cen.x, x);
    assert_eq!(g.cen.y, y);
    assert_eq!(g.cov.ixx, ixx);
    assert_eq!(g.cov.ixy, ixy);
    assert_eq!(g.cov.iyy, iyy);

    match g.cov.set_det() {
        Err(_) => panic!("should not happen"),
        Ok(_) => {},
    }


    assert_eq!(g.cov.det, det);
    assert_eq!(g.cov.dxx, ixx*idet);
    assert_eq!(g.cov.dxy, ixy*idet);
    assert_eq!(g.cov.dyy, iyy*idet);

}
