#![allow(dead_code)]

use ash;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Device lost")]
pub struct DeviceLost;

impl DeviceLost {
    pub(crate) fn from_vk_result(result: ash::vk::Result) -> Self {
        match result {
            ash::vk::Result::ERROR_DEVICE_LOST => DeviceLost,
            _ => panic!("Unexpected result value"),
        }
    }
}

#[derive(Error, Debug)]
#[error("Surface lost")]
pub struct SurfaceLost;

impl SurfaceLost {
    pub(crate) fn from_vk_result(result: ash::vk::Result) -> Self {
        match result {
            ash::vk::Result::ERROR_SURFACE_LOST_KHR => SurfaceLost,
            _ => panic!("Unexpected result value"),
        }
    }
}

/// Out of memory error.
#[derive(Error, Debug)]
pub enum OomError {
    /// Host memory exhausted.
    #[error("Out of host memory")]
    OutOfHostMemory,

    /// Device memory exhausted.
    #[error("Out of device memory")]
    OutOfDeviceMemory,
}

impl OomError {
    pub(crate) fn from_vk_result(result: ash::vk::Result) -> Self {
        match result {
            ash::vk::Result::ERROR_OUT_OF_HOST_MEMORY => OomError::OutOfHostMemory,
            ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => OomError::OutOfDeviceMemory,
            _ => panic!("Unexpected result value"),
        }
    }
}

/// Possible errors returned by `Instance` and `PhysicalDevice`.
#[derive(Error, Debug)]
pub enum InstanceError {
    #[error("Failed to load Vulkan library {}", _0)]
    LibraryLoadError(String),

    #[error("Failed to load functions {:?}", _0)]
    LoadError(Vec<&'static str>),

    #[error(transparent)]
    OomError(#[from] OomError),

    #[error("Initialization failed")]
    InitializationFailed,

    #[error("Layer not present")]
    LayerNotPresent,

    #[error("Extension not present")]
    ExtensionNotPresent,

    #[error("Incompatible driver")]
    IncompatibleDriver,
}

impl InstanceError {
    pub(crate) fn from_loading_error(error: ash::LoadingError) -> Self {
        InstanceError::LibraryLoadError(format!("{}", error))
    }

    pub(crate) fn from_instance_error(error: ash::InstanceError) -> Self {
        match error {
            ash::InstanceError::LoadError(names) => InstanceError::LoadError(names),
            ash::InstanceError::VkError(result) => InstanceError::from_vk_result(result),
        }
    }

    pub(crate) fn from_vk_result(result: ash::vk::Result) -> Self {
        match result {
            ash::vk::Result::ERROR_OUT_OF_HOST_MEMORY => {
                InstanceError::OomError(OomError::OutOfHostMemory)
            }
            ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => {
                InstanceError::OomError(OomError::OutOfDeviceMemory)
            }
            ash::vk::Result::ERROR_INITIALIZATION_FAILED => InstanceError::InitializationFailed,
            ash::vk::Result::ERROR_LAYER_NOT_PRESENT => InstanceError::LayerNotPresent,
            ash::vk::Result::ERROR_EXTENSION_NOT_PRESENT => InstanceError::ExtensionNotPresent,
            ash::vk::Result::ERROR_INCOMPATIBLE_DRIVER => InstanceError::IncompatibleDriver,
            _ => panic!("Unexpected error value"),
        }
    }
}

/// Possible errors returned by `Device`.
#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Failed to load device functions {:?}", _0)]
    LoadError(Vec<&'static str>),

    #[error(transparent)]
    OomError(#[from] OomError),

    #[error(transparent)]
    DeviceLost(#[from] DeviceLost),

    #[error("Initialization failed")]
    InitializationFailed,

    #[error("Extension not present")]
    ExtensionNotPresent,

    #[error("Feature not present")]
    FeatureNotPresent,

    #[error("Too many objects")]
    TooManyObjects,
}

impl DeviceError {
    pub(crate) fn from_device_error(error: ash::InstanceError) -> Self {
        match error {
            ash::InstanceError::LoadError(names) => DeviceError::LoadError(names),
            ash::InstanceError::VkError(result) => DeviceError::from_vk_result(result),
        }
    }

    pub(crate) fn from_vk_result(result: ash::vk::Result) -> Self {
        match result {
            ash::vk::Result::ERROR_OUT_OF_HOST_MEMORY => {
                DeviceError::OomError(OomError::OutOfHostMemory)
            }
            ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => {
                DeviceError::OomError(OomError::OutOfDeviceMemory)
            }
            ash::vk::Result::ERROR_DEVICE_LOST => DeviceError::DeviceLost(DeviceLost),
            ash::vk::Result::ERROR_INITIALIZATION_FAILED => DeviceError::InitializationFailed,
            ash::vk::Result::ERROR_EXTENSION_NOT_PRESENT => DeviceError::ExtensionNotPresent,
            ash::vk::Result::ERROR_FEATURE_NOT_PRESENT => DeviceError::FeatureNotPresent,
            ash::vk::Result::ERROR_TOO_MANY_OBJECTS => DeviceError::TooManyObjects,
            _ => panic!("Unexpected result value"),
        }
    }
}

#[derive(Error, Debug)]
pub enum SurfaceError {
    #[error(transparent)]
    OomError(#[from] OomError),

    #[error(transparent)]
    DeviceLost(#[from] DeviceLost),

    #[error(transparent)]
    SurfaceLost(#[from] SurfaceLost),

    #[error("Native window in use")]
    WindowInUse,
}

impl SurfaceError {
    pub(crate) fn from_vk_result(result: ash::vk::Result) -> Self {
        match result {
            ash::vk::Result::ERROR_OUT_OF_HOST_MEMORY => {
                SurfaceError::OomError(OomError::OutOfHostMemory)
            }
            ash::vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => {
                SurfaceError::OomError(OomError::OutOfDeviceMemory)
            }
            ash::vk::Result::ERROR_DEVICE_LOST => SurfaceError::DeviceLost(DeviceLost),
            ash::vk::Result::ERROR_SURFACE_LOST_KHR => SurfaceError::SurfaceLost(SurfaceLost),
            ash::vk::Result::ERROR_NATIVE_WINDOW_IN_USE_KHR => SurfaceError::WindowInUse,
            _ => panic!("Unexpected result value"),
        }
    }
}
