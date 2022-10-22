// use opencv::{
//     prelude::*, core::*,
//     types::{VectorOfKeyPoint, VectorOfPoint3f},
// };
use cxx::{CxxVector};



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
    

    pub struct Pose{
    
        pub pose: [[f64;4];4]
    }




    unsafe extern "C++" {
        // Opaque types which both languages can pass around
        // but only C++ can see the fields.

        include!("orb_slam3/src/TwoViewReconstruction.h");

        
        type TwoViewReconstruction;
      
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


        fn Reconstruct_1(
            self: &TwoViewReconstruction,
            vKeys1: &CxxVector<MyKeyPoint>,
        )-> bool;
        
    }
}



// bool (orb_slam3::TwoViewReconstruction::*)(const std::vector<orb_slam3::DVKeyPoint>&) const
// bool (orb_slam3::TwoViewReconstruction::*)(const std::vector<orb_slam3::MyKeyPoint>&) const