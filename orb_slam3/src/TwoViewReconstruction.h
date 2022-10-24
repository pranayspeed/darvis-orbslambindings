/**
* This file is part of ORB-SLAM3
*
* Copyright (C) 2017-2021 Carlos Campos, Richard Elvira, Juan J. Gómez Rodríguez, José M.M. Montiel and Juan D. Tardós, University of Zaragoza.
* Copyright (C) 2014-2016 Raúl Mur-Artal, José M.M. Montiel and Juan D. Tardós, University of Zaragoza.
*
* ORB-SLAM3 is free software: you can redistribute it and/or modify it under the terms of the GNU General Public
* License as published by the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* ORB-SLAM3 is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
* the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License along with ORB-SLAM3.
* If not, see <http://www.gnu.org/licenses/>.
*/

#ifndef TwoViewReconstruction_H
#define TwoViewReconstruction_H

#include <opencv2/opencv.hpp>
#include <Eigen/Core>
#include <unordered_set>

#include <sophus/se3.hpp>

#include <opencv2/core/types.hpp>

namespace orb_slam3
{

    struct DVKeyPoint;
    struct DVPoint3f;
    struct Pose;
    struct DVbool;
    struct VectorOfDVPoint3f;
    struct VectorOfDVBool;

    class TwoViewReconstruction
    {
        typedef std::pair<int,int> Match;
        
    public:
        EIGEN_MAKE_ALIGNED_OPERATOR_NEW
        // Fix the reference frame
        TwoViewReconstruction(float fx, float cx, float fy, float cy, float sigma, int iterations);

        // Computes in parallel a fundamental matrix and a homography
        // Selects a model and tries to recover the motion and the structure from motion
        bool Reconstruct(const std::vector<cv::KeyPoint>& vKeys1, const std::vector<cv::KeyPoint>& vKeys2, const std::vector<int> &vMatches12,
                          Sophus::SE3f &T21, std::vector<cv::Point3f> &vP3D, std::vector<bool> &vbTriangulated);


        bool Reconstruct_1(
            const std::vector<orb_slam3::DVKeyPoint> &vKeys1
        ) const;


        // bool Reconstruct_2(
        //     const std::vector<orb_slam3::DVKeyPoint> &vKeys1,
        //     const std::vector<orb_slam3::DVKeyPoint> &vKeys2,const std::vector<int> &vMatches12,
        //     orb_slam3::Pose &T21, 
        //     std::vector<orb_slam3::DVPoint3f> &vP3D, 
        //     std::vector<orb_slam3::DVbool> &vbTriangulated
        // )const ;

        bool Reconstruct_2(
            const std::vector<orb_slam3::DVKeyPoint> &vKeys1,
            const std::vector<orb_slam3::DVKeyPoint> &vKeys2,const std::vector<int> &vMatches12,
            orb_slam3::Pose &T21, 
            VectorOfDVPoint3f &vP3D, 
            VectorOfDVBool &vbTriangulated
        )const ;

    private:

        void FindHomography(std::vector<bool> &vbMatchesInliers, float &score, Eigen::Matrix3f &H21);
        void FindFundamental(std::vector<bool> &vbInliers, float &score, Eigen::Matrix3f &F21);

        Eigen::Matrix3f ComputeH21(const std::vector<cv::Point2f> &vP1, const std::vector<cv::Point2f> &vP2);
        Eigen::Matrix3f ComputeF21(const std::vector<cv::Point2f> &vP1, const std::vector<cv::Point2f> &vP2);

        float CheckHomography(const Eigen::Matrix3f &H21, const Eigen::Matrix3f &H12, std::vector<bool> &vbMatchesInliers, float sigma);

        float CheckFundamental(const Eigen::Matrix3f &F21, std::vector<bool> &vbMatchesInliers, float sigma);

        bool ReconstructF(std::vector<bool> &vbMatchesInliers, Eigen::Matrix3f &F21, Eigen::Matrix3f &K,
                          Sophus::SE3f &T21, std::vector<cv::Point3f> &vP3D, std::vector<bool> &vbTriangulated, float minParallax, int minTriangulated);

        bool ReconstructH(std::vector<bool> &vbMatchesInliers, Eigen::Matrix3f &H21, Eigen::Matrix3f &K,
                          Sophus::SE3f &T21, std::vector<cv::Point3f> &vP3D,std:: vector<bool> &vbTriangulated, float minParallax, int minTriangulated);

        void Normalize(const std::vector<cv::KeyPoint> &vKeys, std::vector<cv::Point2f> &vNormalizedPoints, Eigen::Matrix3f &T);


        int CheckRT(const Eigen::Matrix3f &R, const Eigen::Vector3f &t, const std::vector<cv::KeyPoint> &vKeys1, const std::vector<cv::KeyPoint> &vKeys2,
                    const std::vector<Match> &vMatches12, std::vector<bool> &vbMatchesInliers,
                    const Eigen::Matrix3f &K, std::vector<cv::Point3f> &vP3D, float th2, std::vector<bool> &vbGood, float &parallax);

        void DecomposeE(const Eigen::Matrix3f &E, Eigen::Matrix3f &R1, Eigen::Matrix3f &R2, Eigen::Vector3f &t);


        // Keypoints from Reference Frame (Frame 1)
        std::vector<cv::KeyPoint> mvKeys1;

        // Keypoints from Current Frame (Frame 2)
        std::vector<cv::KeyPoint> mvKeys2;

        // Current Matches from Reference to Current
        std::vector<Match> mvMatches12;
        std::vector<bool> mvbMatched1;

        // Calibration
        Eigen::Matrix3f mK;

        // Standard Deviation and Variance
        float mSigma, mSigma2;

        // Ransac max iterations
        int mMaxIterations;

        // Ransac sets
        std::vector<std::vector<size_t> > mvSets;

    };

    std::unique_ptr<TwoViewReconstruction> new_two_view_reconstruction(float fx, float cx, float fy, float cy, float sigma, int iterations);
    
    // std::unique_ptr<BridgeVecOfKeypoint> new_vec_keypoint(const VecOfKeypoint & vKeys1);
    void test();

} //namespace ORB_SLAM

#endif // TwoViewReconstruction_H
