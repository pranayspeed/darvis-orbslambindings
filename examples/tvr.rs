extern crate dvos3binding;
use nalgebra::{Isometry3};
use cxx::{let_cxx_string, CxxVector};

use opencv::prelude::*;
use opencv::types::VectorOfKeyPoint;

fn main() {
    let tvr = dvos3binding::ffi::new_two_view_reconstruction(1.0, 1.0, 1.0, 1.0, 1.0, 200);
    
    //let pts3d = dvos3binding::ffi::Points3f{pts : [0.0,0.1,0.1]};
    //let kps = dvos3binding::ffi::VecOfKeypoint::new(vec){vec : vec![pts3d]};

    let mut kps_cv = opencv::types::VectorOfKeyPoint::default();
    kps_cv.push(opencv::core::KeyPoint::new_coords(5.0, 3.0, 5.0, 1.4, 1.1, 3, 0).unwrap());

    let mut kps1cv = kps_cv.clone().into_raw() as *const CxxVector<dvos3binding::ffi::MyKeyPoint> as *mut CxxVector<dvos3binding::ffi::MyKeyPoint>;

    unsafe{
    tvr.Reconstruct_3(&*kps1cv);
    }
    let kps = dvos3binding::MyVectorOfKeyPoint::new(&mut kps_cv);

    unsafe{
        tvr.Reconstruct_2(&kps);
        let kps_from_raw = VectorOfKeyPoint::from_raw(kps.get_ptr());
        println!("{:?}", &kps_from_raw);

        
    }

    //let kps = dvos3binding::ffi::new_vec_keypoint(&kps);

    // let kp1 = dvos3binding::MyDVKeyPoint::new(&opencv::core::KeyPoint::new_coords(5.0, 3.0, 5.0, 1.4, 1.1, 3, 0).unwrap());

    // let kps1  = vec![kp1];

    
    //let kps1_1 : &CxxVector<opencv::core::KeyPoint> = &kps1;
    //cxx::CxxVector::<dvos3binding::MyDVKeyPoint>::
    // tvr.Reconstruct_2(&kps);
    // let test = dvos3binding::ffi::test();
    //let test2 = dvos3binding::ffi::test2();

    // let pose = Pose::default();
    // let_cxx_string!(vertex_name = "VertexSE3Expmap");

    // g2orust::ffi::create_frame_vertex(
    //     1, &vertex_name, 
    //     pose.t_to_array(), pose.r_quaternion_to_array(), 
    //     optimizer_ptr
    // );

}
