extern crate dvos3binding;
use nalgebra::{Isometry3};
use cxx::{let_cxx_string, CxxVector};

use opencv::prelude::*;
use opencv::types::VectorOfKeyPoint;

fn main() {
    let tvr = dvos3binding::ffi::new_two_view_reconstruction(1.0, 1.0, 1.0, 1.0, 1.0, 200);
    
    let mut kps_cv = opencv::types::VectorOfKeyPoint::default();

    for i in 1..10
    {
        kps_cv.push(opencv::core::KeyPoint::new_coords(i as f32 *5.0, 1.0/i as f32 *3.0, 5.0, 1.4, 1.1, 3, 0).unwrap());
    }




    unsafe{
        let  kps1cv = kps_cv.into_raw() as *const CxxVector<dvos3binding::ffi::MyKeyPoint>;

        tvr.Reconstruct_1(&*kps1cv);
        
    }


}
