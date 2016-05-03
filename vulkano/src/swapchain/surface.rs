// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::error;
use std::fmt;
use std::mem;
use std::ops::Range;
use std::ptr;
use std::sync::Arc;

use format::Format;
use format::FormatDesc;
use image::Usage as ImageUsage;
use instance::Instance;
use instance::PhysicalDevice;
use instance::QueueFamily;
use swapchain::display::DisplayMode;
use swapchain::display::DisplayPlane;

use check_errors;
use Error;
use OomError;
use VulkanObject;
use VulkanPointers;
use vk;

/// Represents a surface on the screen.
///
/// Creating a `Surface` is platform-specific.
#[derive(Debug)]
pub struct Surface {
    instance: Arc<Instance>,
    surface: vk::SurfaceKHR,
}

impl Surface {
    /// Creates a `Surface` that covers a display mode.
    ///
    /// # Panic
    ///
    /// - Panicks if `display_mode` and `plane` don't belong to the same physical device.
    /// - Panicks if `plane` doesn't support the display of `display_mode`.
    ///
    pub fn from_display_mode(display_mode: &DisplayMode, plane: &DisplayPlane)
                             -> Result<Arc<Surface>, SurfaceCreationError>
    {
        unimplemented!()        // TODO:
        /*if !display_mode.display().physical_device().instance().loaded_extensions().khr_display {
            return Err(SurfaceCreationError::MissingExtension { name: "VK_KHR_display" });
        }

        assert_eq!(display_mode.display().physical_device().internal_object(),
                   plane.physical_device().internal_object());
        assert!(plane.supports(display_mode.display()));

        let instance = display_mode.display().physical_device().instance();
        let vk = instance.pointers();

        let surface = unsafe {
            let infos = vk::DisplaySurfaceCreateInfoKHR {
                sType: vk::STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR,
                pNext: ptr::null(),
                flags: 0,   // reserved
                displayMode: display_mode.internal_object(),
                planeIndex: plane.index,
                planeStackIndex: plane.properties.currentStackIndex,
                transform: vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR,      // TODO: let user choose
                globalAlpha: 0.0,       // TODO: let user choose
                alphaMode: vk::DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR,       // TODO: let user choose
                imageExtent: vk::Extent2D {     // TODO: let user choose
                    width: display_mode.parameters.visibleRegion.width,
                    height: display_mode.parameters.visibleRegion.height,
                },
            };

            let mut output = mem::uninitialized();
            try!(check_errors(vk.CreateDisplayPlaneSurfaceKHR(instance.internal_object(), &infos,
                                                              ptr::null(), &mut output)));
            output
        };

        Ok(Arc::new(Surface {
            instance: instance.clone(),
            surface: surface,
        }))*/
    }

    /// Creates a `Surface` from a Win32 window.
    ///
    /// The surface's min, max and current extent will always match the window's dimensions.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `hinstance` and the `hwnd` are both correct and stay
    /// alive for the entire lifetime of the surface.
    pub unsafe fn from_hwnd<T, U>(instance: &Arc<Instance>, hinstance: *const T, hwnd: *const U)
                                  -> Result<Arc<Surface>, SurfaceCreationError>
    {
        let vk = instance.pointers();

        if !instance.loaded_extensions().khr_win32_surface {
            return Err(SurfaceCreationError::MissingExtension { name: "VK_KHR_win32_surface" });
        }

        let surface = {
            let infos = vk::Win32SurfaceCreateInfoKHR {
                sType: vk::STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
                pNext: ptr::null(),
                flags: 0,   // reserved
                hinstance: hinstance as *mut _,
                hwnd: hwnd as *mut _,
            };

            let mut output = mem::uninitialized();
            try!(check_errors(vk.CreateWin32SurfaceKHR(instance.internal_object(), &infos,
                                                       ptr::null(), &mut output)));
            output
        };

        Ok(Arc::new(Surface {
            instance: instance.clone(),
            surface: surface,
        }))
    }

    /// Creates a `Surface` from an XCB window.
    ///
    /// The surface's min, max and current extent will always match the window's dimensions.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `connection` and the `window` are both correct and stay
    /// alive for the entire lifetime of the surface.
    pub unsafe fn from_xcb<C, W>(instance: &Arc<Instance>, connection: *const C, window: *const W)
                                 -> Result<Arc<Surface>, SurfaceCreationError>
    {
        let vk = instance.pointers();

        if !instance.loaded_extensions().khr_xcb_surface {
            return Err(SurfaceCreationError::MissingExtension { name: "VK_KHR_xcb_surface" });
        }

        let surface = {
            let infos = vk::XcbSurfaceCreateInfoKHR   {
                sType: vk::STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
                pNext: ptr::null(),
                flags: 0,   // reserved
                connection: connection as *mut _,
                window: window as *mut _,
            };

            let mut output = mem::uninitialized();
            try!(check_errors(vk.CreateXcbSurfaceKHR(instance.internal_object(), &infos,
                                                     ptr::null(), &mut output)));
            output
        };

        Ok(Arc::new(Surface {
            instance: instance.clone(),
            surface: surface,
        }))
    }

    /// Creates a `Surface` from an Xlib window.
    ///
    /// The surface's min, max and current extent will always match the window's dimensions.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `display` and the `window` are both correct and stay
    /// alive for the entire lifetime of the surface.
    pub unsafe fn from_xlib<D, W>(instance: &Arc<Instance>, display: *const D, window: *const W)
                                  -> Result<Arc<Surface>, SurfaceCreationError>
    {
        let vk = instance.pointers();

        if !instance.loaded_extensions().khr_xlib_surface {
            return Err(SurfaceCreationError::MissingExtension { name: "VK_KHR_xlib_surface" });
        }

        let surface = {
            let infos = vk::XlibSurfaceCreateInfoKHR  {
                sType: vk::STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
                pNext: ptr::null(),
                flags: 0,   // reserved
                dpy: display as *mut _,
                window: window as *mut _,
            };

            let mut output = mem::uninitialized();
            try!(check_errors(vk.CreateXlibSurfaceKHR(instance.internal_object(), &infos,
                                                      ptr::null(), &mut output)));
            output
        };

        Ok(Arc::new(Surface {
            instance: instance.clone(),
            surface: surface,
        }))
    }

    /// Creates a `Surface` from a Wayland window.
    ///
    /// The window's dimensions will be set to the size of the swapchain.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `display` and the `surface` are both correct and stay
    /// alive for the entire lifetime of the surface.
    pub unsafe fn from_wayland<D, S>(instance: &Arc<Instance>, display: *const D, surface: *const S)
                                     -> Result<Arc<Surface>, SurfaceCreationError>
    {
        let vk = instance.pointers();

        if !instance.loaded_extensions().khr_wayland_surface {
            return Err(SurfaceCreationError::MissingExtension { name: "VK_KHR_wayland_surface" });
        }

        let surface = {
            let infos = vk::WaylandSurfaceCreateInfoKHR {
                sType: vk::STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
                pNext: ptr::null(),
                flags: 0,   // reserved
                display: display as *mut _,
                surface: surface as *mut _,
            };

            let mut output = mem::uninitialized();
            try!(check_errors(vk.CreateWaylandSurfaceKHR(instance.internal_object(), &infos,
                                                         ptr::null(), &mut output)));
            output
        };

        Ok(Arc::new(Surface {
            instance: instance.clone(),
            surface: surface,
        }))
    }

    /// Creates a `Surface` from a MIR window.
    ///
    /// If the swapchain's dimensions does not match the window's dimensions, the image will
    /// automatically be scaled during presentation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `connection` and the `surface` are both correct and stay
    /// alive for the entire lifetime of the surface.
    pub unsafe fn from_mir<C, S>(instance: &Arc<Instance>, connection: *const C, surface: *const S)
                                 -> Result<Arc<Surface>, SurfaceCreationError>
    {
        let vk = instance.pointers();

        if !instance.loaded_extensions().khr_mir_surface {
            return Err(SurfaceCreationError::MissingExtension { name: "VK_KHR_mir_surface" });
        }

        let surface = {
            let infos = vk::MirSurfaceCreateInfoKHR  {
                sType: vk::STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR,
                pNext: ptr::null(),
                flags: 0,   // reserved
                connection: connection as *mut _,
                mirSurface: surface as *mut _,
            };

            let mut output = mem::uninitialized();
            try!(check_errors(vk.CreateMirSurfaceKHR(instance.internal_object(), &infos,
                                                     ptr::null(), &mut output)));
            output
        };

        Ok(Arc::new(Surface {
            instance: instance.clone(),
            surface: surface,
        }))
    }

    /// Creates a `Surface` from an Android window.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `window` is correct and stays alive for the entire
    /// lifetime of the surface.
    pub unsafe fn from_anativewindow<T>(instance: &Arc<Instance>, window: *const T)
                                        -> Result<Arc<Surface>, SurfaceCreationError>
    {
        let vk = instance.pointers();

        if !instance.loaded_extensions().khr_android_surface {
            return Err(SurfaceCreationError::MissingExtension { name: "VK_KHR_android_surface" });
        }

        let surface = {
            let infos = vk::AndroidSurfaceCreateInfoKHR {
                sType: vk::STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR,
                pNext: ptr::null(),
                flags: 0,   // reserved
                window: window as *mut _,
            };

            let mut output = mem::uninitialized();
            try!(check_errors(vk.CreateAndroidSurfaceKHR(instance.internal_object(), &infos,
                                                         ptr::null(), &mut output)));
            output
        };

        Ok(Arc::new(Surface {
            instance: instance.clone(),
            surface: surface,
        }))
    }

    /// Returns true if the given queue family can draw on this surface.
    pub fn is_supported(&self, queue: &QueueFamily) -> Result<bool, OomError> {
        unsafe {
            let vk = self.instance.pointers();

            let mut output = mem::uninitialized();
            try!(check_errors(
                vk.GetPhysicalDeviceSurfaceSupportKHR(queue.physical_device().internal_object(),
                                                      queue.id(), self.surface, &mut output)
            ));
            Ok(output != 0)
        }
    }

    /// Retreives the capabilities of a surface when used by a certain device.
    ///
    /// # Panic
    ///
    /// - Panicks if the device and the surface don't belong to the same instance.
    ///
    pub fn get_capabilities(&self, device: &PhysicalDevice) -> Result<Capabilities, OomError> { // TODO: wrong error type
        unsafe {
            assert_eq!(&*self.instance as *const _, &**device.instance() as *const _);

            let vk = self.instance.pointers();

            let caps = {
                let mut out: vk::SurfaceCapabilitiesKHR = mem::uninitialized();
                try!(check_errors(
                    vk.GetPhysicalDeviceSurfaceCapabilitiesKHR(device.internal_object(),
                                                               self.surface, &mut out)
                ));
                out
            };

            let formats = {
                let mut num = 0;
                try!(check_errors(
                    vk.GetPhysicalDeviceSurfaceFormatsKHR(device.internal_object(),
                                                          self.surface, &mut num,
                                                          ptr::null_mut())
                ));

                let mut formats = Vec::with_capacity(num as usize);
                try!(check_errors(
                    vk.GetPhysicalDeviceSurfaceFormatsKHR(device.internal_object(),
                                                          self.surface, &mut num,
                                                          formats.as_mut_ptr())
                ));
                formats.set_len(num as usize);
                formats
            };

            let modes = {
                let mut num = 0;
                try!(check_errors(
                    vk.GetPhysicalDeviceSurfacePresentModesKHR(device.internal_object(),
                                                               self.surface, &mut num,
                                                               ptr::null_mut())
                ));

                let mut modes = Vec::with_capacity(num as usize);
                try!(check_errors(
                    vk.GetPhysicalDeviceSurfacePresentModesKHR(device.internal_object(),
                                                               self.surface, &mut num,
                                                               modes.as_mut_ptr())
                ));
                modes.set_len(num as usize);
                debug_assert!(modes.iter().find(|&&m| m == vk::PRESENT_MODE_FIFO_KHR).is_some());
                SupportedPresentModes::from_list(modes.into_iter())
            };

            Ok(Capabilities {
                image_count: caps.minImageCount .. caps.maxImageCount + 1,
                current_extent: if caps.currentExtent.width == 0xffffffff &&
                                   caps.currentExtent.height == 0xffffffff
                {
                    None
                } else {
                    Some([caps.currentExtent.width, caps.currentExtent.height])
                },
                min_image_extent: [caps.minImageExtent.width, caps.minImageExtent.height],
                max_image_extent: [caps.maxImageExtent.width, caps.maxImageExtent.height],
                max_image_array_layers: caps.maxImageArrayLayers,
                supported_transforms: SurfaceTransform::from_bits(caps.supportedTransforms),
                current_transform: SurfaceTransform::from_bits(caps.supportedTransforms).into_iter().next().unwrap(),        // TODO:
                supported_composite_alpha: CompositeAlpha::from_bits(caps.supportedCompositeAlpha),
                supported_usage_flags: {
                    let usage = ImageUsage::from_bits(caps.supportedUsageFlags);
                    debug_assert!(usage.color_attachment);  // specs say that this must be true
                    usage
                },
                supported_formats: formats.into_iter().map(|f| {
                    (Format::from_num(f.format).unwrap(), ColorSpace::from_num(f.colorSpace))
                }).collect(),
                present_modes: modes,
            })
        }
    }
}

unsafe impl VulkanObject for Surface {
    type Object = vk::SurfaceKHR;

    #[inline]
    fn internal_object(&self) -> vk::SurfaceKHR {
        self.surface
    }
}

impl Drop for Surface {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let vk = self.instance.pointers();
            vk.DestroySurfaceKHR(self.instance.internal_object(), self.surface, ptr::null());
        }
    }
}

/// Error that can happen when creating a debug callback.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SurfaceCreationError {
    /// Not enough memory.
    OomError(OomError),

    /// The extension required for this function was not enabled.
    MissingExtension { name: &'static str },
}

impl error::Error for SurfaceCreationError {
    #[inline]
    fn description(&self) -> &str {
        match *self {
            SurfaceCreationError::OomError(_) => "not enough memory available",
            SurfaceCreationError::MissingExtension { .. } => "the extension required for this \
                                                              function was not enabled",
        }
    }

    #[inline]
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SurfaceCreationError::OomError(ref err) => Some(err),
            _ => None
        }
    }
}

impl fmt::Display for SurfaceCreationError {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", error::Error::description(self))
    }
}

impl From<OomError> for SurfaceCreationError {
    #[inline]
    fn from(err: OomError) -> SurfaceCreationError {
        SurfaceCreationError::OomError(err)
    }
}

impl From<Error> for SurfaceCreationError {
    #[inline]
    fn from(err: Error) -> SurfaceCreationError {
        match err {
            err @ Error::OutOfHostMemory => SurfaceCreationError::OomError(OomError::from(err)),
            err @ Error::OutOfDeviceMemory => SurfaceCreationError::OomError(OomError::from(err)),
            _ => panic!("unexpected error: {:?}", err)
        }
    }
}

/// The capabilities of a surface when used by a physical device.
///
/// You have to match these capabilities when you create a swapchain.
#[derive(Clone, Debug)]
pub struct Capabilities {
    /// Range of the number of images that can be created. Please remember that the end is out of
    /// the range.
    pub image_count: Range<u32>,

    /// The current dimensions of the surface. `None` means that the surface's dimensions will
    /// depend on the dimensions of the swapchain that you are going to create.
    pub current_extent: Option<[u32; 2]>,

    pub min_image_extent: [u32; 2],
    pub max_image_extent: [u32; 2],
    pub max_image_array_layers: u32,
    pub supported_transforms: Vec<SurfaceTransform>,
    pub current_transform: SurfaceTransform,
    pub supported_composite_alpha: Vec<CompositeAlpha>,
    pub supported_usage_flags: ImageUsage,
    pub supported_formats: Vec<(Format, ColorSpace)>,       // FIXME: driver can return FORMAT_UNDEFINED which indicates that it has no preferred format, so that field should be an Option

    /// List of present modes that are supported. `Fifo` is always guaranteed to be supported.
    pub present_modes: SupportedPresentModes,
}

/// The way presenting a swapchain is accomplished.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PresentMode {
    /// Immediately shows the image to the user. May result in visible tearing.
    Immediate = vk::PRESENT_MODE_IMMEDIATE_KHR,

    /// The action of presenting an image puts it in wait. When the next vertical blanking period
    /// happens, the waiting image is effectively shown to the user. If an image is presented while
    /// another one is waiting, it is replaced.
    Mailbox = vk::PRESENT_MODE_MAILBOX_KHR,

    /// The action of presenting an image adds it to a queue of images. At each vertical blanking
    /// period, the queue is poped and an image is presented.
    ///
    /// Guaranteed to be always supported.
    ///
    /// This is the equivalent of OpenGL's `SwapInterval` with a value of 1.
    Fifo = vk::PRESENT_MODE_FIFO_KHR,

    /// Same as `Fifo`, except that if the queue was empty during the previous vertical blanking
    /// period then it is equivalent to `Immediate`.
    ///
    /// This is the equivalent of OpenGL's `SwapInterval` with a value of -1.
    Relaxed = vk::PRESENT_MODE_FIFO_RELAXED_KHR,
}

/// List of `PresentMode`s that are supported.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SupportedPresentModes {
    pub immediate: bool,
    pub mailbox: bool,
    pub fifo: bool,
    pub relaxed: bool,
}

impl SupportedPresentModes {
    /// Builds a `SupportedPresentModes` with all fields set to false.
    #[inline]
    pub fn none() -> SupportedPresentModes {
        SupportedPresentModes {
            immediate: false,
            mailbox: false,
            fifo: false,
            relaxed: false,
        }
    }

    #[inline]
    fn from_list<I>(elem: I) -> SupportedPresentModes where I: Iterator<Item = vk::PresentModeKHR> {
        let mut result = SupportedPresentModes::none();
        for e in elem {
            match e {
                vk::PRESENT_MODE_IMMEDIATE_KHR => result.immediate = true,
                vk::PRESENT_MODE_MAILBOX_KHR => result.mailbox = true,
                vk::PRESENT_MODE_FIFO_KHR => result.fifo = true,
                vk::PRESENT_MODE_FIFO_RELAXED_KHR => result.relaxed = true,
                _ => panic!("Wrong value for vk::PresentModeKHR")
            }
        }
        result
    }

    /// Returns true if the given present mode is in this list of supported modes.
    #[inline]
    pub fn supports(&self, mode: PresentMode) -> bool {
        match mode {
            PresentMode::Immediate => self.immediate,
            PresentMode::Mailbox => self.mailbox,
            PresentMode::Fifo => self.fifo,
            PresentMode::Relaxed => self.relaxed,
        }
    }

    /// Returns an iterator to the list of supported present modes.
    #[inline]
    pub fn iter(&self) -> SupportedPresentModesIter {
        SupportedPresentModesIter(self.clone())
    }
}

/// Enumeration of the `PresentMode`s that are supported.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SupportedPresentModesIter(SupportedPresentModes);

impl Iterator for SupportedPresentModesIter {
    type Item = PresentMode;

    #[inline]
    fn next(&mut self) -> Option<PresentMode> {
        if self.0.immediate { self.0.immediate = false; return Some(PresentMode::Immediate); }
        if self.0.mailbox { self.0.mailbox = false; return Some(PresentMode::Mailbox); }
        if self.0.fifo { self.0.fifo = false; return Some(PresentMode::Fifo); }
        if self.0.relaxed { self.0.relaxed = false; return Some(PresentMode::Relaxed); }
        None
    }
}

/// A transformation to apply to the image before showing it on the screen.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SurfaceTransform {
    Identity = vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
    Rotate90 = vk::SURFACE_TRANSFORM_ROTATE_90_BIT_KHR,
    Rotate180 = vk::SURFACE_TRANSFORM_ROTATE_180_BIT_KHR,
    Rotate270 = vk::SURFACE_TRANSFORM_ROTATE_270_BIT_KHR,
    HorizontalMirror = vk::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR,
    HorizontalMirrorRotate90 = vk::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR,
    HorizontalMirrorRotate180 = vk::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR,
    HorizontalMirrorRotate270 = vk::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR,
    Inherit = vk::SURFACE_TRANSFORM_INHERIT_BIT_KHR,
}

impl SurfaceTransform {
    fn from_bits(val: u32) -> Vec<SurfaceTransform> {
        macro_rules! v {
            ($val:expr, $out:ident, $e:expr, $o:ident) => (
                if ($val & $e) != 0 { $out.push(SurfaceTransform::$o); }
            );
        }

        let mut result = Vec::with_capacity(9);
        v!(val, result, vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR, Identity);
        v!(val, result, vk::SURFACE_TRANSFORM_ROTATE_90_BIT_KHR, Rotate90);
        v!(val, result, vk::SURFACE_TRANSFORM_ROTATE_180_BIT_KHR, Rotate180);
        v!(val, result, vk::SURFACE_TRANSFORM_ROTATE_270_BIT_KHR, Rotate270);
        v!(val, result, vk::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR, HorizontalMirror);
        v!(val, result, vk::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR,
                        HorizontalMirrorRotate90);
        v!(val, result, vk::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR,
                        HorizontalMirrorRotate180);
        v!(val, result, vk::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR,
                        HorizontalMirrorRotate270);
        v!(val, result, vk::SURFACE_TRANSFORM_INHERIT_BIT_KHR, Inherit);
        result
    }
}

impl Default for SurfaceTransform {
    #[inline]
    fn default() -> SurfaceTransform {
        SurfaceTransform::Identity
    }
}

// How the alpha values of the pixels of the window are treated.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CompositeAlpha {
    /// The alpha channel of the image is ignored. All the pixels are considered as if they have a
    /// value of 1.0.
    Opaque = vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,

    /// The alpha channel of the image is respected. The color channels are expected to have
    /// already been multiplied by the alpha value.
    PreMultiplied = vk::COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR,

    /// The alpha channel of the image is respected. The color channels will be multiplied by the
    /// alpha value by the compositor before being added to what is behind.
    PostMultiplied = vk::COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR,

    /// Platform-specific behavior.
    Inherit = vk::COMPOSITE_ALPHA_INHERIT_BIT_KHR,
}

impl CompositeAlpha {
    fn from_bits(val: u32) -> Vec<CompositeAlpha> {
        let mut result = Vec::with_capacity(4);
        if (val & vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR) != 0 { result.push(CompositeAlpha::Opaque); }
        if (val & vk::COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR) != 0 { result.push(CompositeAlpha::PreMultiplied); }
        if (val & vk::COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR) != 0 { result.push(CompositeAlpha::PostMultiplied); }
        if (val & vk::COMPOSITE_ALPHA_INHERIT_BIT_KHR) != 0 { result.push(CompositeAlpha::Inherit); }
        result
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorSpace {
    SrgbNonLinear,
}

impl ColorSpace {
    #[inline]
    fn from_num(val: u32) -> ColorSpace {
        assert_eq!(val, vk::COLORSPACE_SRGB_NONLINEAR_KHR);
        ColorSpace::SrgbNonLinear
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;
    use swapchain::Surface;
    use swapchain::SurfaceCreationError;

    #[test]
    fn khr_win32_surface_ext_missing() {
        let instance = instance!();
        match unsafe { Surface::from_hwnd(&instance, ptr::null::<u8>(), ptr::null::<u8>()) } {
            Err(SurfaceCreationError::MissingExtension { .. }) => (),
            _ => panic!()
        }
    }

    #[test]
    fn khr_xcb_surface_ext_missing() {
        let instance = instance!();
        match unsafe { Surface::from_xcb(&instance, ptr::null::<u8>(), ptr::null::<u8>()) } {
            Err(SurfaceCreationError::MissingExtension { .. }) => (),
            _ => panic!()
        }
    }

    #[test]
    fn khr_xlib_surface_ext_missing() {
        let instance = instance!();
        match unsafe { Surface::from_xlib(&instance, ptr::null::<u8>(), ptr::null::<u8>()) } {
            Err(SurfaceCreationError::MissingExtension { .. }) => (),
            _ => panic!()
        }
    }

    #[test]
    fn khr_wayland_surface_ext_missing() {
        let instance = instance!();
        match unsafe { Surface::from_wayland(&instance, ptr::null::<u8>(), ptr::null::<u8>()) } {
            Err(SurfaceCreationError::MissingExtension { .. }) => (),
            _ => panic!()
        }
    }

    #[test]
    fn khr_mir_surface_ext_missing() {
        let instance = instance!();
        match unsafe { Surface::from_mir(&instance, ptr::null::<u8>(), ptr::null::<u8>()) } {
            Err(SurfaceCreationError::MissingExtension { .. }) => (),
            _ => panic!()
        }
    }

    #[test]
    fn khr_android_surface_ext_missing() {
        let instance = instance!();
        match unsafe { Surface::from_anativewindow(&instance, ptr::null::<u8>()) } {
            Err(SurfaceCreationError::MissingExtension { .. }) => (),
            _ => panic!()
        }
    }
}
