
use cxx::{CxxVector, SharedPtr};


#[cxx::bridge(namespace = "orb_slam3")]
pub mod ffi {
    // Shared structs with fields visible to both languages.
    #[derive(Debug, Clone)]
    pub struct DVPoints2f
    {
        pub x : f32,
        pub y : f32
    }
    #[derive(Debug, Clone)]
    pub struct DVKeyPoint {
        /// coordinates of the keypoints
        pub pt: DVPoints2f,
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

    #[derive(Debug, Clone)]
    pub struct DVPoint3f {
        pub x : f32,
        pub y : f32,
        pub z: f32
    }

    pub struct Pose{
    
        pub pose: [[f64;4];4]
    }
    #[derive(Debug, Clone)]
    pub struct DVbool{
    
        pub val: bool
    }

    pub struct VectorOfDVPoint3f
    {
        pub vec: Vec<DVPoint3f>
    }
    pub struct VectorOfDVBool
    {
        pub vec: Vec<bool>
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
            vKeys1: &CxxVector<DVKeyPoint>,
        )-> bool;
        
        
        // fn Reconstruct_2(
        //     self: &TwoViewReconstruction,
        //     vKeys1: &CxxVector<DVKeyPoint>,
        //     vKeys2:  &CxxVector<DVKeyPoint>,
        //     vMatches12: &CxxVector<i32>,
        //     T21: &mut Pose,
        //     vP3D: Pin<&mut CxxVector<DVPoint3f>>,
        //     vbTriangulated: Pin<&mut CxxVector<DVbool>>
        // )-> bool;

        fn Reconstruct_2(
            self: &TwoViewReconstruction,
            vKeys1: &CxxVector<DVKeyPoint>,
            vKeys2:  &CxxVector<DVKeyPoint>,
            vMatches12: &CxxVector<i32>,
            T21: &mut Pose,
            vP3D: &mut VectorOfDVPoint3f,
            vbTriangulated: &mut VectorOfDVBool
        )-> bool;
    }
}




