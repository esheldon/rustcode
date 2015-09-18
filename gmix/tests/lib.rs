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
    let covopt = Cov2::new(ixx, ixy, iyy);

    assert!( match covopt {
        None => false,
        Some(cov) => {
            assert_eq!(cov.ixx, ixx);
            assert_eq!(cov.det, det);
            true
        }
    });

}

// could not get [should_fail] to work, so just inverted
#[test]
fn test_cov2_bad() {
    let ixx=1.0;
    let ixy=1.0;
    let iyy=1.0;
    let covopt = Cov2::new(ixx, ixy, iyy);

    assert!( match covopt {
        None => true,
        Some(cov) => false,
    });
}


#[test]
fn test_gauss2() {
    let pt = Point2 {x: 1.5, y: 2.5};

    assert_eq!(pt.x, 1.5);
    assert_eq!(pt.y, 2.5);

    let ixx=3.5;
    let ixy=0.2;
    let iyy=1.1;
    let det = ixx*iyy - ixy*ixy;
    let cov = match Cov2::new(ixx, ixy, iyy) {
        None => panic!("bad cov"),
        Some(cov) => cov,
    };

    let p=100.0;

    let g = Gauss2::new(p, &cen, &cov);
}
