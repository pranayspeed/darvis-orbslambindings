// use opencv::{
//     prelude::*, core::*,
//     types::{VectorOfKeyPoint, VectorOfPoint3f},
// };
use cxx::{let_cxx_string, CxxVector};

use std::ffi::c_void;

extern crate opencv;

use opencv::prelude::*;
// mod opencv;  // the bindgen-generated bindings
// #[derive(Clone, Debug)]
// pub struct MyVectorOfKeyPoint(opencv::types::VectorOfKeyPoint);

// impl MyVectorOfKeyPoint
// {
//     pub fn new( vec : &opencv::types::VectorOfKeyPoint ) -> Self
//     {
//         MyVectorOfKeyPoint
//         {
//             0 : vec.clone()
//         }
//     }
// }




#[derive(Debug)]
pub struct MyVectorOfKeyPoint(*mut c_void);
// {
//    // pub ptr: *mut c_void
// }

impl MyVectorOfKeyPoint
{
    pub fn new( vec : &mut opencv::types::VectorOfKeyPoint ) -> Self
    {
        MyVectorOfKeyPoint
        {
            0 : vec.as_raw_mut()
        }
    }

    pub fn get_ptr(&self ) -> *mut c_void
    {
        self.0.clone()
    }
}

#[derive(Clone, Debug)]
pub struct MyDVKeyPoint(opencv::core::KeyPoint);

impl MyDVKeyPoint
{
    pub fn new( kp : &opencv::core::KeyPoint ) -> Self
    {
        MyDVKeyPoint
        {
            0 : kp.clone()
        }
    }
}




use cxx::{type_id, ExternType};



unsafe impl ExternType for MyVectorOfKeyPoint {
    type Id = type_id!("orb_slam3::VecOfKeypoint");
    type Kind = cxx::kind::Opaque;
}


// unsafe impl ExternType for MyDVKeyPoint {
//     type Id = type_id!("orb_slam3::DVKeyPoint");
//     type Kind = cxx::kind::Trivial;
// }

// unsafe impl ExternType for opencv::types::VectorOfPoint3f {
//     type Id = type_id!("orb_slam3::types::VectorOfPoint3f");
//     type Kind = cxx::kind::Opaque;
// }


#[cxx::bridge(namespace = "orb_slam3")]
pub mod ffi {
    // Shared structs with fields visible to both languages.

    pub struct Points2f
    {
        pub x : f32,
        pub y : f32
    }

    pub struct MyKeyPoint {
        /// coordinates of the keypoints
        pub pt: Points2f,
        /// diameter of the meaningful keypoint neighborhood
        pub size: f32,
        /// computed orientation of the keypoint (-1 if not applicable);
        /// it's in [0,360) degrees and measured relative to
        /// image coordinate system, ie in clockwise.
        pub angle: f32,
        /// the response by which the most strong keypoints have been selected. Can be used for the further sorting or subsampling
        pub response: f32,
        /// octave (pyramid layer) from which the keypoint has been extracted
        pub octave: i32,
        /// object class (if the keypoints need to be clustered by an object they belong to)
        pub class_id: i32,
    }
    

    // pub struct MyVectorOfMyKeyPoint<'a>
    // {
    //     pub vec : &'a  CxxVector<MyKeyPoint>
    // }
    

    //pub struct MyVectorOfMyKeyPoint1(CxxVector<MyKeyPoint>);

    // pub struct VecOfKeypoint_new{
    
    //     //vec: Vec<Points3f>
    //     vec : MyVectorOfKeyPoint
    // }


    pub struct Pose{
    
        pub pose: [[f64;4];4]
    }




    unsafe extern "C++" {
        // Opaque types which both languages can pass around
        // but only C++ can see the fields.

        include!("orb_slam3/src/TwoViewReconstruction.h");
        //include!("orb_slam3/Sophus/sophus/se3.hpp");
        //include!("opencv2/core/core.hpp");
        
        
        type TwoViewReconstruction;
        type VecOfKeypoint = crate::MyVectorOfKeyPoint;

        //type DVKeyPoint = crate::MyDVKeyPoint;


        // type VectorOfPoint3f ;//=   crate::opencv::types::VectorOfPoint3f;// VectorOfPoint3f;
        //type VectorOfKeyPoint  = crate::opencv::opencv::VecOfKeypoint;//  = Vec<KeyPoint>;
        // type SE3f;// = Sophus::SE3f;
        
        //type VectorOfKeyPoint = Vec<cv::KeyPoint>;
        // type BridgeVecOfKeypoint;
        // fn new_vec_keypoint(
        //     vKeys1: &VecOfKeypoint,
        // ) -> UniquePtr<BridgeVecOfKeypoint>;
        
        fn new_two_view_reconstruction(
            fx: f32,
            cx: f32,
            fy: f32,
            cy: f32,
            sigma: f32,
            iterations: i32
        ) -> UniquePtr<TwoViewReconstruction>;

        // fn Reconstruct(
        //     self: &TwoViewReconstruction,
        //     vKeys1: UniquePtr<VectorOfKeyPoint>,
        //     vKeys2:  UniquePtr<VectorOfKeyPoint>,
        //     vMatches12: Vec<i32>,
        //     T21: UniquePtr< SE3f>,
        //     vP3D: UniquePtr<VectorOfPoint3f>,
        //     vbTriangulated: Vec<bool>
        // ) -> bool;

        // fn Reconstruct_1(
        //     self: &TwoViewReconstruction,
        //     vKeys1: UniquePtr<BridgeVecOfKeypoint>,
        // )-> bool;

        fn Reconstruct_2(
            self: &TwoViewReconstruction,
            vKeys1: &VecOfKeypoint,
        )-> bool;

        fn Reconstruct_3(
            self: &TwoViewReconstruction,
            vKeys1: &CxxVector<MyKeyPoint>,
        )-> bool;
        
    }
}



// bool (orb_slam3::TwoViewReconstruction::*)(const std::vector<orb_slam3::DVKeyPoint>&) const
// bool (orb_slam3::TwoViewReconstruction::*)(const std::vector<orb_slam3::MyKeyPoint>&) const