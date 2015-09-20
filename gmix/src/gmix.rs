use gauss2::{Gauss2};

enum SimpleModel {
    Gauss,
    Exp,
    Dev,
    Turb,
}

pub struct SimplePars {
    pub model: SimpleModel;
    pub model_name: String;
    //pub data: vec<f64>;
    pub data: [f64, ..6];
}


type GMix = vec<Gauss2>;

// do we even need SimplePars?
// we can just have these and use polymorphism
// the raw GMix would be used by em but the
// fitters would just deal with a pars vector


struct GMixModel {
    pub model: SimpleModel;
    pub model_name: String;
    //pub pars: vec<f64>;
    pub pars: [f64, ..6];
    pub gm: GMix;
}

struct GMixCoellip {
    pub ngauss: Int;
    // will have size 4 + 2*ngauss for
    // [row,col,g1,g2,T1,...Tn,F1,...Fn]
    pub pars: vec<f64>;
    pub gm: GMix;
}

struct GMixCM {
    pub fracdev: f64;
    pub TdByTe: f64;
    //pub pars: vec<f64>;
    pub pars: [f64, ..6];
    pub gm: GMix;
}
