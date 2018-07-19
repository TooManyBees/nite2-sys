#![allow(non_camel_case_types, non_snake_case)]

// bindgen ./Include/NiteCAPI.h -o ~/code/rust/nite2-sys.rs --whitelist-function nite.* --rustified-enum Nite.* --opaque-type Oni.* -- -x c++ -I../OpenNI-MacOSX-x64-2.2/Include

extern crate openni2_sys;
use openni2_sys::{OniDeviceHandle, OniFrame, OniGeneralCallback};

use std::os::raw::{c_short, c_int, c_ulonglong, c_void};
#[cfg(test)] use std::mem::{size_of, align_of};
#[cfg(test)] use std::ptr;

#[test]
fn test_oni_general_callback_size() {
    // Without having onicapi.h for reference, bindgen thought
    // OniGeneralCallback (a function pointer) should be a u64
    assert_eq!(size_of::<u64>(), size_of::<OniGeneralCallback>());
}

/// Available joints in skeleton
pub type NiteJointType = c_int;
pub const NITE_JOINT_HEAD: NiteJointType = 0;
pub const NITE_JOINT_NECK: NiteJointType = 1;
pub const NITE_JOINT_LEFT_SHOULDER: NiteJointType = 2;
pub const NITE_JOINT_RIGHT_SHOULDER: NiteJointType = 3;
pub const NITE_JOINT_LEFT_ELBOW: NiteJointType = 4;
pub const NITE_JOINT_RIGHT_ELBOW: NiteJointType = 5;
pub const NITE_JOINT_LEFT_HAND: NiteJointType = 6;
pub const NITE_JOINT_RIGHT_HAND: NiteJointType = 7;
pub const NITE_JOINT_TORSO: NiteJointType = 8;
pub const NITE_JOINT_LEFT_HIP: NiteJointType = 9;
pub const NITE_JOINT_RIGHT_HIP: NiteJointType = 10;
pub const NITE_JOINT_LEFT_KNEE: NiteJointType = 11;
pub const NITE_JOINT_RIGHT_KNEE: NiteJointType = 12;
pub const NITE_JOINT_LEFT_FOOT: NiteJointType = 13;
pub const NITE_JOINT_RIGHT_FOOT: NiteJointType = 14;

/// Possible states of the skeleton
pub type NiteSkeletonState = c_int;
/// No skeleton - skeleton was not requested
pub const NITE_SKELETON_NONE: NiteSkeletonState = 0;
/// Skeleton requested, but still unavailable
pub const NITE_SKELETON_CALIBRATING: NiteSkeletonState = 1;
/// Skeleton available
pub const NITE_SKELETON_TRACKED: NiteSkeletonState = 2;
/// Possible reasons as to why skeleton is unavailable
pub const NITE_SKELETON_CALIBRATION_ERROR_NOT_IN_POSE: NiteSkeletonState = 3;
/// Possible reasons as to why skeleton is unavailable
pub const NITE_SKELETON_CALIBRATION_ERROR_HANDS: NiteSkeletonState = 4;
/// Possible reasons as to why skeleton is unavailable
pub const NITE_SKELETON_CALIBRATION_ERROR_HEAD: NiteSkeletonState = 5;
/// Possible reasons as to why skeleton is unavailable
pub const NITE_SKELETON_CALIBRATION_ERROR_LEGS: NiteSkeletonState = 6;
/// Possible reasons as to why skeleton is unavailable
pub const NITE_SKELETON_CALIBRATION_ERROR_TORSO: NiteSkeletonState = 7;

/// Possible failure values
pub type NiteStatus = c_int;
pub const NITE_STATUS_OK: NiteStatus = 0;
pub const NITE_STATUS_ERROR: NiteStatus = 1;
pub const NITE_STATUS_BAD_USER_ID: NiteStatus = 2;
pub const NITE_STATUS_OUT_OF_FLOW: NiteStatus = 3;

pub type NitePoseType = c_int;
pub const NITE_POSE_PSI: NitePoseType = 0;
pub const NITE_POSE_CROSSED_HANDS: NitePoseType = 1;

/// Available gestures types
pub type NiteGestureType = c_int;
pub const NITE_GESTURE_WAVE: NiteGestureType = 0;
pub const NITE_GESTURE_CLICK: NiteGestureType = 1;
pub const NITE_GESTURE_HAND_RAISE: NiteGestureType = 2;

pub type NiteUserId = c_short;
#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteUserTracker {
    _unused: [u8; 0],
}
pub type NiteUserTrackerHandle = *mut NiteUserTracker;

/// 3D Point
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NitePoint3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[test]
fn bindgen_test_layout_NitePoint3f() {
    assert_eq!(size_of::<NitePoint3f>(),
               12usize,
               concat!("Size of: ", stringify!(NitePoint3f)));
    assert_eq!(align_of::<NitePoint3f>(),
               4usize,
               concat!("Alignment of ", stringify!(NitePoint3f)));
    assert_eq!(unsafe { &(*(ptr::null::<NitePoint3f>())).x as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NitePoint3f),
                       "::",
                       stringify!(x)));
    assert_eq!(unsafe { &(*(ptr::null::<NitePoint3f>())).y as *const _ as usize },
               4usize,
               concat!("Offset of field: ",
                       stringify!(NitePoint3f),
                       "::",
                       stringify!(y)));
    assert_eq!(unsafe { &(*(ptr::null::<NitePoint3f>())).z as *const _ as usize },
               8usize,
               concat!("Offset of field: ",
                       stringify!(NitePoint3f),
                       "::",
                       stringify!(z)));
}
/// Quaternion
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteQuaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[test]
fn bindgen_test_layout_NiteQuaternion() {
    assert_eq!(size_of::<NiteQuaternion>(),
               16usize,
               concat!("Size of: ", stringify!(NiteQuaternion)));
    assert_eq!(align_of::<NiteQuaternion>(),
               4usize,
               concat!("Alignment of ", stringify!(NiteQuaternion)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteQuaternion>())).x as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteQuaternion),
                       "::",
                       stringify!(x)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteQuaternion>())).y as *const _ as usize },
               4usize,
               concat!("Offset of field: ",
                       stringify!(NiteQuaternion),
                       "::",
                       stringify!(y)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteQuaternion>())).z as *const _ as usize },
               8usize,
               concat!("Offset of field: ",
                       stringify!(NiteQuaternion),
                       "::",
                       stringify!(z)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteQuaternion>())).w as *const _ as usize },
               12usize,
               concat!("Offset of field: ",
                       stringify!(NiteQuaternion),
                       "::",
                       stringify!(w)));
}
/// Single joint of a skeleton
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteSkeletonJoint {
    /// Type of the joint
    pub jointType: NiteJointType,
    /// Position of the joint - in real world coordinates
    pub position: NitePoint3f,
    pub positionConfidence: f32,
    /// Orientation of the joint
    pub orientation: NiteQuaternion,
    pub orientationConfidence: f32,
}
#[test]
fn bindgen_test_layout_NiteSkeletonJoint() {
    assert_eq!(size_of::<NiteSkeletonJoint>(),
               40usize,
               concat!("Size of: ", stringify!(NiteSkeletonJoint)));
    assert_eq!(align_of::<NiteSkeletonJoint>(),
               4usize,
               concat!("Alignment of ", stringify!(NiteSkeletonJoint)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteSkeletonJoint>())).jointType as *const _ as usize
               },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteSkeletonJoint),
                       "::",
                       stringify!(jointType)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteSkeletonJoint>())).position as *const _ as usize
               },
               4usize,
               concat!("Offset of field: ",
                       stringify!(NiteSkeletonJoint),
                       "::",
                       stringify!(position)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteSkeletonJoint>())).positionConfidence as *const _ as
                   usize
               },
               16usize,
               concat!("Offset of field: ",
                       stringify!(NiteSkeletonJoint),
                       "::",
                       stringify!(positionConfidence)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteSkeletonJoint>())).orientation as *const _ as usize
               },
               20usize,
               concat!("Offset of field: ",
                       stringify!(NiteSkeletonJoint),
                       "::",
                       stringify!(orientation)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteSkeletonJoint>())).orientationConfidence as
                   *const _ as usize
               },
               36usize,
               concat!("Offset of field: ",
                       stringify!(NiteSkeletonJoint),
                       "::",
                       stringify!(orientationConfidence)));
}
/// 3D Box
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteBoundingBox {
    pub min: NitePoint3f,
    pub max: NitePoint3f,
}
#[test]
fn bindgen_test_layout_NiteBoundingBox() {
    assert_eq!(size_of::<NiteBoundingBox>(),
               24usize,
               concat!("Size of: ", stringify!(NiteBoundingBox)));
    assert_eq!(align_of::<NiteBoundingBox>(),
               4usize,
               concat!("Alignment of ", stringify!(NiteBoundingBox)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteBoundingBox>())).min as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteBoundingBox),
                       "::",
                       stringify!(min)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteBoundingBox>())).max as *const _ as usize },
               12usize,
               concat!("Offset of field: ",
                       stringify!(NiteBoundingBox),
                       "::",
                       stringify!(max)));
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NitePoseData {
    pub type_: NitePoseType,
    pub state: c_int,
}
#[test]
fn bindgen_test_layout_NitePoseData() {
    assert_eq!(size_of::<NitePoseData>(),
               8usize,
               concat!("Size of: ", stringify!(NitePoseData)));
    assert_eq!(align_of::<NitePoseData>(),
               4usize,
               concat!("Alignment of ", stringify!(NitePoseData)));
    assert_eq!(unsafe { &(*(ptr::null::<NitePoseData>())).type_ as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NitePoseData),
                       "::",
                       stringify!(type_)));
    assert_eq!(unsafe { &(*(ptr::null::<NitePoseData>())).state as *const _ as usize },
               4usize,
               concat!("Offset of field: ",
                       stringify!(NitePoseData),
                       "::",
                       stringify!(state)));
}
/// Skeleton - a set of joints
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteSkeleton {
    pub joints: [NiteSkeletonJoint; 15usize],
    pub state: NiteSkeletonState,
}
#[test]
fn bindgen_test_layout_NiteSkeleton() {
    assert_eq!(size_of::<NiteSkeleton>(),
               604usize,
               concat!("Size of: ", stringify!(NiteSkeleton)));
    assert_eq!(align_of::<NiteSkeleton>(),
               4usize,
               concat!("Alignment of ", stringify!(NiteSkeleton)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteSkeleton>())).joints as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteSkeleton),
                       "::",
                       stringify!(joints)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteSkeleton>())).state as *const _ as usize },
               600usize,
               concat!("Offset of field: ",
                       stringify!(NiteSkeleton),
                       "::",
                       stringify!(state)));
}
/// Snapshot of a specific user
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteUserData {
    pub id: NiteUserId,
    pub boundingBox: NiteBoundingBox,
    pub centerOfMass: NitePoint3f,
    pub state: c_int,
    pub skeleton: NiteSkeleton,
    pub poses: [NitePoseData; 2usize],
}
#[test]
fn bindgen_test_layout_NiteUserData() {
    assert_eq!(size_of::<NiteUserData>(),
               664usize,
               concat!("Size of: ", stringify!(NiteUserData)));
    assert_eq!(align_of::<NiteUserData>(),
               4usize,
               concat!("Alignment of ", stringify!(NiteUserData)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteUserData>())).id as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserData),
                       "::",
                       stringify!(id)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteUserData>())).boundingBox as *const _ as usize
               },
               4usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserData),
                       "::",
                       stringify!(boundingBox)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteUserData>())).centerOfMass as *const _ as usize
               },
               28usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserData),
                       "::",
                       stringify!(centerOfMass)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteUserData>())).state as *const _ as usize },
               40usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserData),
                       "::",
                       stringify!(state)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteUserData>())).skeleton as *const _ as usize },
               44usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserData),
                       "::",
                       stringify!(skeleton)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteUserData>())).poses as *const _ as usize },
               648usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserData),
                       "::",
                       stringify!(poses)));
}
/// Snapshot of the scene segmentation
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteUserMap {
    pub pixels: *mut NiteUserId,
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
}
#[test]
fn bindgen_test_layout_NiteUserMap() {
    assert_eq!(size_of::<NiteUserMap>(),
               24usize,
               concat!("Size of: ", stringify!(NiteUserMap)));
    assert_eq!(align_of::<NiteUserMap>(),
               8usize,
               concat!("Alignment of ", stringify!(NiteUserMap)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteUserMap>())).pixels as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserMap),
                       "::",
                       stringify!(pixels)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteUserMap>())).width as *const _ as usize },
               8usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserMap),
                       "::",
                       stringify!(width)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteUserMap>())).height as *const _ as usize },
               12usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserMap),
                       "::",
                       stringify!(height)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteUserMap>())).stride as *const _ as usize },
               16usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserMap),
                       "::",
                       stringify!(stride)));
}
/// 3D Plane
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NitePlane {
    pub point: NitePoint3f,
    pub normal: NitePoint3f,
}
#[test]
fn bindgen_test_layout_NitePlane() {
    assert_eq!(size_of::<NitePlane>(),
               24usize,
               concat!("Size of: ", stringify!(NitePlane)));
    assert_eq!(align_of::<NitePlane>(),
               4usize,
               concat!("Alignment of ", stringify!(NitePlane)));
    assert_eq!(unsafe { &(*(ptr::null::<NitePlane>())).point as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NitePlane),
                       "::",
                       stringify!(point)));
    assert_eq!(unsafe { &(*(ptr::null::<NitePlane>())).normal as *const _ as usize },
               12usize,
               concat!("Offset of field: ",
                       stringify!(NitePlane),
                       "::",
                       stringify!(normal)));
}
/// Output snapshot of the User Tracker algorithm
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteUserTrackerFrame {
    /// Number of users
    pub userCount: c_int,
    /// List of users
    pub pUser: *mut NiteUserData,
    /// Scene segmentation map
    pub userMap: NiteUserMap,
    /// The depth frame from which this data was learned
    pub pDepthFrame: *mut OniFrame,
    pub timestamp: c_ulonglong,
    pub frameIndex: c_int,
    /// Confidence of the floor plane
    pub floorConfidence: f32,
    /// Floor plane
    pub floor: NitePlane,
}
#[test]
fn bindgen_test_layout_NiteUserTrackerFrame() {
    assert_eq!(size_of::<NiteUserTrackerFrame>(),
               88usize,
               concat!("Size of: ", stringify!(NiteUserTrackerFrame)));
    assert_eq!(align_of::<NiteUserTrackerFrame>(),
               8usize,
               concat!("Alignment of ", stringify!(NiteUserTrackerFrame)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteUserTrackerFrame>())).userCount as *const _ as usize
               },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserTrackerFrame),
                       "::",
                       stringify!(userCount)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteUserTrackerFrame>())).pUser as *const _ as usize
               },
               8usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserTrackerFrame),
                       "::",
                       stringify!(pUser)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteUserTrackerFrame>())).userMap as *const _ as usize
               },
               16usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserTrackerFrame),
                       "::",
                       stringify!(userMap)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteUserTrackerFrame>())).pDepthFrame as *const _ as usize
               },
               40usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserTrackerFrame),
                       "::",
                       stringify!(pDepthFrame)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteUserTrackerFrame>())).timestamp as *const _ as usize
               },
               48usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserTrackerFrame),
                       "::",
                       stringify!(timestamp)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteUserTrackerFrame>())).frameIndex as *const _ as usize
               },
               56usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserTrackerFrame),
                       "::",
                       stringify!(frameIndex)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteUserTrackerFrame>())).floorConfidence as *const _ as
                   usize
               },
               60usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserTrackerFrame),
                       "::",
                       stringify!(floorConfidence)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteUserTrackerFrame>())).floor as *const _ as usize
               },
               64usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserTrackerFrame),
                       "::",
                       stringify!(floor)));
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteUserTrackerCallbacks {
    pub readyForNextFrame: OniGeneralCallback,
}
#[test]
fn bindgen_test_layout_NiteUserTrackerCallbacks() {
    assert_eq!(size_of::<NiteUserTrackerCallbacks>(),
               8usize,
               concat!("Size of: ", stringify!(NiteUserTrackerCallbacks)));
    assert_eq!(align_of::<NiteUserTrackerCallbacks>(),
               8usize,
               concat!("Alignment of ", stringify!(NiteUserTrackerCallbacks)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteUserTrackerCallbacks>())).readyForNextFrame as
                   *const _ as usize
               },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteUserTrackerCallbacks),
                       "::",
                       stringify!(readyForNextFrame)));
}
pub type NiteHandId = c_short;
/// A snapshot of a specific hand
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteHandData {
    pub id: NiteHandId,
    pub position: NitePoint3f,
    pub state: c_int,
}
#[test]
fn bindgen_test_layout_NiteHandData() {
    assert_eq!(size_of::<NiteHandData>(),
               20usize,
               concat!("Size of: ", stringify!(NiteHandData)));
    assert_eq!(align_of::<NiteHandData>(),
               4usize,
               concat!("Alignment of ", stringify!(NiteHandData)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteHandData>())).id as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteHandData),
                       "::",
                       stringify!(id)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteHandData>())).position as *const _ as usize },
               4usize,
               concat!("Offset of field: ",
                       stringify!(NiteHandData),
                       "::",
                       stringify!(position)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteHandData>())).state as *const _ as usize },
               16usize,
               concat!("Offset of field: ",
                       stringify!(NiteHandData),
                       "::",
                       stringify!(state)));
}
/// A snapshot of a specific gesture
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteGestureData {
    pub type_: NiteGestureType,
    pub currentPosition: NitePoint3f,
    pub state: c_int,
}
#[test]
fn bindgen_test_layout_NiteGestureData() {
    assert_eq!(size_of::<NiteGestureData>(),
               20usize,
               concat!("Size of: ", stringify!(NiteGestureData)));
    assert_eq!(align_of::<NiteGestureData>(),
               4usize,
               concat!("Alignment of ", stringify!(NiteGestureData)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteGestureData>())).type_ as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteGestureData),
                       "::",
                       stringify!(type_)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteGestureData>())).currentPosition as *const _ as usize
               },
               4usize,
               concat!("Offset of field: ",
                       stringify!(NiteGestureData),
                       "::",
                       stringify!(currentPosition)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteGestureData>())).state as *const _ as usize },
               16usize,
               concat!("Offset of field: ",
                       stringify!(NiteGestureData),
                       "::",
                       stringify!(state)));
}
/// Output snapshot of the Hand Tracker algorithm
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteHandTrackerFrame {
    /// Number of hands
    pub handCount: c_int,
    /// List of hands
    pub pHands: *mut NiteHandData,
    /// Number of gestures
    pub gestureCount: c_int,
    /// List of gestures
    pub pGestures: *mut NiteGestureData,
    /// The depth frame from which this data was learned
    pub pDepthFrame: *mut OniFrame,
    pub timestamp: c_ulonglong,
    pub frameIndex: c_int,
}
#[test]
fn bindgen_test_layout_NiteHandTrackerFrame() {
    assert_eq!(size_of::<NiteHandTrackerFrame>(),
               56usize,
               concat!("Size of: ", stringify!(NiteHandTrackerFrame)));
    assert_eq!(align_of::<NiteHandTrackerFrame>(),
               8usize,
               concat!("Alignment of ", stringify!(NiteHandTrackerFrame)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteHandTrackerFrame>())).handCount as *const _ as usize
               },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteHandTrackerFrame),
                       "::",
                       stringify!(handCount)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteHandTrackerFrame>())).pHands as *const _ as usize
               },
               8usize,
               concat!("Offset of field: ",
                       stringify!(NiteHandTrackerFrame),
                       "::",
                       stringify!(pHands)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteHandTrackerFrame>())).gestureCount as *const _ as
                   usize
               },
               16usize,
               concat!("Offset of field: ",
                       stringify!(NiteHandTrackerFrame),
                       "::",
                       stringify!(gestureCount)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteHandTrackerFrame>())).pGestures as *const _ as usize
               },
               24usize,
               concat!("Offset of field: ",
                       stringify!(NiteHandTrackerFrame),
                       "::",
                       stringify!(pGestures)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteHandTrackerFrame>())).pDepthFrame as *const _ as usize
               },
               32usize,
               concat!("Offset of field: ",
                       stringify!(NiteHandTrackerFrame),
                       "::",
                       stringify!(pDepthFrame)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteHandTrackerFrame>())).timestamp as *const _ as usize
               },
               40usize,
               concat!("Offset of field: ",
                       stringify!(NiteHandTrackerFrame),
                       "::",
                       stringify!(timestamp)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteHandTrackerFrame>())).frameIndex as *const _ as usize
               },
               48usize,
               concat!("Offset of field: ",
                       stringify!(NiteHandTrackerFrame),
                       "::",
                       stringify!(frameIndex)));
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteHandTrackerCallbacks {
    pub readyForNextFrame: OniGeneralCallback,
}
#[test]
fn bindgen_test_layout_NiteHandTrackerCallbacks() {
    assert_eq!(size_of::<NiteHandTrackerCallbacks>(),
               8usize,
               concat!("Size of: ", stringify!(NiteHandTrackerCallbacks)));
    assert_eq!(align_of::<NiteHandTrackerCallbacks>(),
               8usize,
               concat!("Alignment of ", stringify!(NiteHandTrackerCallbacks)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<NiteHandTrackerCallbacks>())).readyForNextFrame as
                   *const _ as usize
               },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteHandTrackerCallbacks),
                       "::",
                       stringify!(readyForNextFrame)));
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteVersion {
    pub major: c_int,
    pub minor: c_int,
    pub maintenance: c_int,
    pub build: c_int,
}
#[test]
fn bindgen_test_layout_NiteVersion() {
    assert_eq!(size_of::<NiteVersion>(),
               16usize,
               concat!("Size of: ", stringify!(NiteVersion)));
    assert_eq!(align_of::<NiteVersion>(),
               4usize,
               concat!("Alignment of ", stringify!(NiteVersion)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteVersion>())).major as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(NiteVersion),
                       "::",
                       stringify!(major)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteVersion>())).minor as *const _ as usize },
               4usize,
               concat!("Offset of field: ",
                       stringify!(NiteVersion),
                       "::",
                       stringify!(minor)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteVersion>())).maintenance as *const _ as usize },
               8usize,
               concat!("Offset of field: ",
                       stringify!(NiteVersion),
                       "::",
                       stringify!(maintenance)));
    assert_eq!(unsafe { &(*(ptr::null::<NiteVersion>())).build as *const _ as usize },
               12usize,
               concat!("Offset of field: ",
                       stringify!(NiteVersion),
                       "::",
                       stringify!(build)));
}
#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NiteHandTracker {
    _unused: [u8; 0],
}
pub type NiteHandTrackerHandle = *mut NiteHandTracker;
extern "C" {
    pub fn niteInitialize() -> NiteStatus;
}
extern "C" {
    pub fn niteShutdown();
}
extern "C" {
    pub fn niteGetVersion() -> NiteVersion;
}
extern "C" {
    pub fn niteInitializeUserTracker(pUserTrackerHandle: *mut NiteUserTrackerHandle) -> NiteStatus;
}
extern "C" {
    pub fn niteInitializeUserTrackerByDevice(deviceHandle: OniDeviceHandle,
                                             pUserTrackerHandle: *mut NiteUserTrackerHandle)
                                             -> NiteStatus;
}
extern "C" {
    pub fn niteShutdownUserTracker(userTrackerHandle: NiteUserTrackerHandle) -> NiteStatus;
}
extern "C" {
    pub fn niteStartSkeletonTracking(userTrackerHandle: NiteUserTrackerHandle, userId: NiteUserId) -> NiteStatus;
}
extern "C" {
    pub fn niteStopSkeletonTracking(userTrackerHandle: NiteUserTrackerHandle, userId: NiteUserId);
}
extern "C" {
    pub fn niteIsSkeletonTracking(userTrackerHandle: NiteUserTrackerHandle, userId: NiteUserId) -> bool;
}
extern "C" {
    pub fn niteSetSkeletonSmoothing(userTrackerHandle: NiteUserTrackerHandle, smoothing: f32) -> NiteStatus;
}
extern "C" {
    pub fn niteGetSkeletonSmoothing(userTrackerHandle: NiteUserTrackerHandle, pSmoothing: *mut f32) -> NiteStatus;
}
extern "C" {
    pub fn niteStartPoseDetection(userTrackerHandle: NiteUserTrackerHandle,
                                  userId: NiteUserId,
                                  poseType: NitePoseType)
                                  -> NiteStatus;
}
extern "C" {
    pub fn niteStopPoseDetection(userTrackerHandle: NiteUserTrackerHandle,
                                 userId: NiteUserId,
                                 poseType: NitePoseType);
}
extern "C" {
    pub fn niteStopAllPoseDetection(userTrackerHandle: NiteUserTrackerHandle, userId: NiteUserId);
}
extern "C" {
    pub fn niteRegisterUserTrackerCallbacks(userTrackerHandle: NiteUserTrackerHandle,
                                            pUserTrackerCallbacks: *mut NiteUserTrackerCallbacks,
                                            pCookie: *mut c_void)
                                            -> NiteStatus;
}
extern "C" {
    pub fn niteUnregisterUserTrackerCallbacks(userTrackerHandle: NiteUserTrackerHandle,
                                              pUserTrackerCallbacks: *mut NiteUserTrackerCallbacks);
}
extern "C" {
    pub fn niteReadUserTrackerFrame(userTrackerHandle: NiteUserTrackerHandle,
                                    frame: *mut *mut NiteUserTrackerFrame)
                                    -> NiteStatus;
}
extern "C" {
    pub fn niteUserTrackerFrameAddRef(userTrackerHandle: NiteUserTrackerHandle,
                                      frame: *mut NiteUserTrackerFrame)
                                      -> NiteStatus;
}
extern "C" {
    pub fn niteUserTrackerFrameRelease(userTrackerHandle: NiteUserTrackerHandle,
                                       frame: *mut NiteUserTrackerFrame)
                                       -> NiteStatus;
}
extern "C" {
    pub fn niteInitializeHandTracker(pHandTrackerHandle: *mut NiteHandTrackerHandle) -> NiteStatus;
}
extern "C" {
    pub fn niteInitializeHandTrackerByDevice(deviceHandle: OniDeviceHandle,
                                             pHandTrackerHandle: *mut NiteHandTrackerHandle)
                                             -> NiteStatus;
}
extern "C" {
    pub fn niteShutdownHandTracker(handTrackerHandle: NiteHandTrackerHandle) -> NiteStatus;
}
extern "C" {
    pub fn niteStartHandTracking(handTrackerHandle: NiteHandTrackerHandle,
                                 pPoint: *const NitePoint3f,
                                 pNewHandId: *mut NiteHandId)
                                 -> NiteStatus;
}
extern "C" {
    pub fn niteStopHandTracking(handTrackerHandle: NiteHandTrackerHandle, handId: NiteHandId);
}
extern "C" {
    pub fn niteStopAllHandTracking(handTrackerHandle: NiteHandTrackerHandle);
}
extern "C" {
    pub fn niteSetHandSmoothingFactor(handTrackerHandle: NiteHandTrackerHandle, smoothing: f32) -> NiteStatus;
}
extern "C" {
    pub fn niteGetHandSmoothingFactor(handTrackerHandle: NiteHandTrackerHandle, pSmoothing: *mut f32) -> NiteStatus;
}
extern "C" {
    pub fn niteRegisterHandTrackerCallbacks(handTrackerHandle: NiteHandTrackerHandle,
                                            pHandTrackerCallbacks: *mut NiteHandTrackerCallbacks,
                                            pCookie: *mut c_void)
                                            -> NiteStatus;
}
extern "C" {
    pub fn niteUnregisterHandTrackerCallbacks(handTrackerHandle: NiteHandTrackerHandle,
                                              pHandTrackerCallbacks: *mut NiteHandTrackerCallbacks);
}
extern "C" {
    pub fn niteReadHandTrackerFrame(handTrackerHandle: NiteHandTrackerHandle,
                                    frame: *mut *mut NiteHandTrackerFrame)
                                    -> NiteStatus;
}
extern "C" {
    pub fn niteHandTrackerFrameAddRef(handTrackerHandle: NiteHandTrackerHandle,
                                      pFrame: *mut NiteHandTrackerFrame)
                                      -> NiteStatus;
}
extern "C" {
    pub fn niteHandTrackerFrameRelease(handTrackerHandle: NiteHandTrackerHandle,
                                       pFrame: *mut NiteHandTrackerFrame)
                                       -> NiteStatus;
}
extern "C" {
    pub fn niteStartGestureDetection(handTrackerHandle: NiteHandTrackerHandle,
                                     gestureType: NiteGestureType)
                                     -> NiteStatus;
}
extern "C" {
    pub fn niteStopGestureDetection(handTrackerHandle: NiteHandTrackerHandle, gestureType: NiteGestureType);
}
extern "C" {
    pub fn niteStopAllGestureDetection(handTrackerHandle: NiteHandTrackerHandle);
}
extern "C" {
    pub fn niteConvertJointCoordinatesToDepth(userTracker: NiteUserTrackerHandle,
                                              x: f32,
                                              y: f32,
                                              z: f32,
                                              pX: *mut f32,
                                              pY: *mut f32)
                                              -> NiteStatus;
}
extern "C" {
    pub fn niteConvertDepthCoordinatesToJoint(userTracker: NiteUserTrackerHandle,
                                              x: c_int,
                                              y: c_int,
                                              z: c_int,
                                              pX: *mut f32,
                                              pY: *mut f32)
                                              -> NiteStatus;
}
extern "C" {
    pub fn niteConvertHandCoordinatesToDepth(handTracker: NiteHandTrackerHandle,
                                             x: f32,
                                             y: f32,
                                             z: f32,
                                             pX: *mut f32,
                                             pY: *mut f32)
                                             -> NiteStatus;
}
extern "C" {
    pub fn niteConvertDepthCoordinatesToHand(handTracker: NiteHandTrackerHandle,
                                             x: c_int,
                                             y: c_int,
                                             z: c_int,
                                             pX: *mut f32,
                                             pY: *mut f32)
                                             -> NiteStatus;
}
