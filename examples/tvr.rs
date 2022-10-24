extern crate dvos3binding;
use nalgebra::{Isometry3};
use cxx::{let_cxx_string, CxxVector};

use opencv::prelude::*;
use opencv::types::VectorOfKeyPoint;

use std::pin::Pin;

fn main() {
    let tvr = dvos3binding::ffi::new_two_view_reconstruction(1.0, 1.0, 1.0, 1.0, 1.0, 200);
    
    let mut kps_cv1 = opencv::types::VectorOfKeyPoint::default();
    let mut kps_cv2 = opencv::types::VectorOfKeyPoint::default();

    for i in 1..10
    {
        kps_cv1.push(opencv::core::KeyPoint::new_coords(i as f32 *5.0, 1.0/i as f32 *3.0, 5.0, 1.4, 1.1, 3, 0).unwrap());

        kps_cv2.push(opencv::core::KeyPoint::new_coords(i as f32 *5.0, 1.0/i as f32 *3.0, 5.0, 1.4, 1.1, 3, 0).unwrap());
    }


    // T21: &mut Pose,
    // vP3D: Pin<&mut CxxVector<DVPoint3f>>,
    // vbTriangulated: Pin<&mut CxxVector<DVbool>>


    unsafe{
        let vMatches12 = vec![0, 2,4].as_ptr() as *const CxxVector<i32>; 

        let mut pose = dvos3binding::ffi::Pose{pose : [[0.0;4];4]};
        let mut vP3D = Vec::new().as_mut_ptr() as *mut dvos3binding::ffi::VectorOfDVPoint3f;  


        let mut vbTriangulated =  Vec::new().as_mut_ptr() as *mut dvos3binding::ffi::VectorOfDVBool; 
        
        let  kps1cv = kps_cv1.into_raw() as *const CxxVector<dvos3binding::ffi::DVKeyPoint>;
        let  kps2cv = kps_cv2.into_raw() as *const CxxVector<dvos3binding::ffi::DVKeyPoint>;
        // tvr.Reconstruct_2(&*kps1cv, &*kps2cv, &*vMatches12, &mut pose,
        // Box::pin(vP3D1).as_mut(),
        //  Box::pin(*vbTriangulated).as_mut()
        // );
        
        tvr.Reconstruct_2(&*kps1cv, &*kps2cv, &*vMatches12, &mut pose,
            &mut *vP3D,
             &mut *vbTriangulated
            );
    }


}
