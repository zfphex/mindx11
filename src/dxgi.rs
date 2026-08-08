use crate::types::{
    BOOL, GUID, HANDLE, HMODULE, HRESULT, HWND, IUnknown, IUnknownVtbl, LUID, RECT,
};
use core::ffi::c_void;

pub const IID_IDXGIOBJECT: GUID = GUID::from_u128(0xaec22fb8_76f3_4639_9be0_28eb43a67a2e);
pub const IID_IDXGIDEVICESUBOBJECT: GUID = GUID::from_u128(0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6);
pub const IID_IDXGIRESOURCE: GUID = GUID::from_u128(0x035f3ab4_482e_4e50_b41f_8a7f8bd8960b);
pub const IID_IDXGISURFACE: GUID = GUID::from_u128(0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec);
pub const IID_IDXGIADAPTER: GUID = GUID::from_u128(0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0);
pub const IID_IDXGIADAPTER1: GUID = GUID::from_u128(0x29038f61_3839_4626_91fd_086879011a05);
pub const IID_IDXGIOUTPUT: GUID = GUID::from_u128(0xae02eedb_c735_4690_8d52_5a8dc20213aa);
pub const IID_IDXGISWAPCHAIN: GUID = GUID::from_u128(0x310d36a0_d2e7_4c0a_aa04_6a9d23b8886a);
pub const IID_IDXGIFACTORY: GUID = GUID::from_u128(0x7b7166ec_21c7_44ae_b21a_c9ae321ae369);
pub const IID_IDXGIFACTORY1: GUID = GUID::from_u128(0x770aae78_f26f_4dba_a829_253c83d1b387);

pub const DXGI_USAGE_SHADER_INPUT: u32 = 0x10;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: u32 = 0x20;
pub const DXGI_USAGE_BACK_BUFFER: u32 = 0x40;
pub const DXGI_USAGE_SHARED: u32 = 0x80;
pub const DXGI_USAGE_READ_ONLY: u32 = 0x100;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: u32 = 0x200;
pub const DXGI_USAGE_UNORDERED_ACCESS: u32 = 0x400;

pub const DXGI_SWAP_CHAIN_FLAG_NONPREROLLED: u32 = 1;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: u32 = 2;
pub const DXGI_SWAP_CHAIN_FLAG_ALWAYS_OPTIMIZE: u32 = 4;
pub const DXGI_SWAP_CHAIN_FLAG_PREROLLED: u32 = 8;
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: u32 = 512;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXGI_FORMAT {
    UNKNOWN = 0,
    R32G32B32A32_TYPELESS = 1,
    R32G32B32A32_FLOAT = 2,
    R32G32B32A32_UINT = 3,
    R32G32B32A32_SINT = 4,
    R32G32B32_TYPELESS = 5,
    R32G32B32_FLOAT = 6,
    R32G32B32_UINT = 7,
    R32G32B32_SINT = 8,
    R16G16B16A16_TYPELESS = 9,
    R16G16B16A16_FLOAT = 10,
    R16G16B16A16_UNORM = 11,
    R16G16B16A16_UINT = 12,
    R16G16B16A16_SNORM = 13,
    R16G16B16A16_SINT = 14,
    R32G32_TYPELESS = 15,
    R32G32_FLOAT = 16,
    R32G32_UINT = 17,
    R32G32_SINT = 18,
    R32G8X24_TYPELESS = 19,
    D32_FLOAT_S8X24_UINT = 20,
    R32_FLOAT_X8X24_TYPELESS = 21,
    X32_TYPELESS_G8X24_UINT = 22,
    R10G10B10A2_TYPELESS = 23,
    R10G10B10A2_UNORM = 24,
    R10G10B10A2_UINT = 25,
    R11G11B10_FLOAT = 26,
    R8G8B8A8_TYPELESS = 27,
    R8G8B8A8_UNORM = 28,
    R8G8B8A8_UNORM_SRGB = 29,
    R8G8B8A8_UINT = 30,
    R8G8B8A8_SNORM = 31,
    R8G8B8A8_SINT = 32,
    R16G16_TYPELESS = 33,
    R16G16_FLOAT = 34,
    R16G16_UNORM = 35,
    R16G16_UINT = 36,
    R16G16_SNORM = 37,
    R16G16_SINT = 38,
    R32_TYPELESS = 39,
    D32_FLOAT = 40,
    R32_FLOAT = 41,
    R32_UINT = 42,
    R32_SINT = 43,
    R24G8_TYPELESS = 44,
    D24_UNORM_S8_UINT = 45,
    R24_UNORM_X8_TYPELESS = 46,
    X24_TYPELESS_G8_UINT = 47,
    R8G8_TYPELESS = 48,
    R8G8_UNORM = 49,
    R8G8_UINT = 50,
    R8G8_SNORM = 51,
    R8G8_SINT = 52,
    R16_TYPELESS = 53,
    R16_FLOAT = 54,
    D16_UNORM = 55,
    R16_UNORM = 56,
    R16_UINT = 57,
    R16_SNORM = 58,
    R16_SINT = 59,
    R8_TYPELESS = 60,
    R8_UNORM = 61,
    R8_UINT = 62,
    R8_SNORM = 63,
    R8_SINT = 64,
    A8_UNORM = 65,
    R1_UNORM = 66,
    R9G9B9E5_SHAREDEXP = 67,
    R8G8_B8G8_UNORM = 68,
    G8R8_G8B8_UNORM = 69,
    BC1_TYPELESS = 70,
    BC1_UNORM = 71,
    BC1_UNORM_SRGB = 72,
    BC2_TYPELESS = 73,
    BC2_UNORM = 74,
    BC2_UNORM_SRGB = 75,
    BC3_TYPELESS = 76,
    BC3_UNORM = 77,
    BC3_UNORM_SRGB = 78,
    BC4_TYPELESS = 79,
    BC4_UNORM = 80,
    BC4_SNORM = 81,
    BC5_TYPELESS = 82,
    BC5_UNORM = 83,
    BC5_SNORM = 84,
    B5G6R5_UNORM = 85,
    B5G5R5A1_UNORM = 86,
    B8G8R8A8_UNORM = 87,
    B8G8R8X8_UNORM = 88,
    R10G10B10_XR_BIAS_A2_UNORM = 89,
    B8G8R8A8_TYPELESS = 90,
    B8G8R8A8_UNORM_SRGB = 91,
    B8G8R8X8_TYPELESS = 92,
    B8G8R8X8_UNORM_SRGB = 93,
    BC6H_TYPELESS = 94,
    BC6H_UF16 = 95,
    BC6H_SF16 = 96,
    BC7_TYPELESS = 97,
    BC7_UNORM = 98,
    BC7_UNORM_SRGB = 99,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXGI_SWAP_EFFECT {
    DISCARD = 0,
    SEQUENTIAL = 1,
    FLIP_SEQUENTIAL = 3,
    FLIP_DISCARD = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXGI_MODE_SCANLINE_ORDER {
    UNSPECIFIED = 0,
    PROGRESSIVE = 1,
    UPPER_FIELD_FIRST = 2,
    LOWER_FIELD_FIRST = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXGI_MODE_SCALING {
    UNSPECIFIED = 0,
    CENTERED = 1,
    STRETCHED = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXGI_MODE_ROTATION {
    UNSPECIFIED = 0,
    IDENTITY = 1,
    ROTATE90 = 2,
    ROTATE180 = 3,
    ROTATE270 = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DXGI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DXGI_MODE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: DXGI_MODE_DESC,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub OutputWindow: HWND,
    pub Windowed: BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: LUID,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: LUID,
    pub Flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: RECT,
    pub AttachedToDesktop: BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}

#[repr(C)]
pub struct IDXGIObjectVtbl {
    pub base: IUnknownVtbl,
    pub SetPrivateData: unsafe extern "system" fn(
        this: *mut c_void,
        Name: *const GUID,
        DataSize: u32,
        pData: *const c_void,
    ) -> HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        this: *mut c_void,
        Name: *const GUID,
        pUnknown: *const IUnknown,
    ) -> HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(
        this: *mut c_void,
        Name: *const GUID,
        pDataSize: *mut u32,
        pData: *mut c_void,
    ) -> HRESULT,
    pub GetParent: unsafe extern "system" fn(
        this: *mut c_void,
        riid: *const GUID,
        ppParent: *mut *mut c_void,
    ) -> HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IDXGIObject(pub *mut *const IDXGIObjectVtbl);

impl IDXGIObject {
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn SetPrivateData(&self, Name: &GUID, pData: &[u8]) -> Result<(), HRESULT> {
        let hr = unsafe {
            ((*(*self.0)).SetPrivateData)(
                self.0 as _,
                Name as *const _,
                pData.len() as u32,
                pData.as_ptr() as _,
            )
        };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn SetPrivateDataInterface(
        &self,
        Name: &GUID,
        pUnknown: Option<&IUnknown>,
    ) -> Result<(), HRESULT> {
        let unk = pUnknown.map_or(core::ptr::null(), |u| u as *const _);
        let hr =
            unsafe { ((*(*self.0)).SetPrivateDataInterface)(self.0 as _, Name as *const _, unk) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn GetPrivateData(
        &self,
        Name: &GUID,
        pData: Option<&mut [u8]>,
    ) -> Result<u32, HRESULT> {
        let (ptr, mut size) = match pData {
            Some(buf) => (buf.as_mut_ptr() as *mut c_void, buf.len() as u32),
            None => (core::ptr::null_mut(), 0),
        };
        let hr =
            unsafe { ((*(*self.0)).GetPrivateData)(self.0 as _, Name as *const _, &mut size, ptr) };
        if hr >= 0 { Ok(size) } else { Err(hr) }
    }
    pub unsafe fn GetParent<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        let mut parent = core::ptr::null_mut();
        let hr = unsafe { ((*(*self.0)).GetParent)(self.0 as _, riid as *const _, &mut parent) };
        if hr >= 0 {
            Ok(unsafe { core::mem::transmute_copy(&parent) })
        } else {
            Err(hr)
        }
    }
}

#[repr(C)]
pub struct IDXGIDeviceSubObjectVtbl {
    pub base: IDXGIObjectVtbl,
    pub GetDevice: unsafe extern "system" fn(
        this: *mut c_void,
        riid: *const GUID,
        ppDevice: *mut *mut c_void,
    ) -> HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IDXGIDeviceSubObject(pub *mut *const IDXGIDeviceSubObjectVtbl);

impl IDXGIDeviceSubObject {
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDevice<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        let mut dev = core::ptr::null_mut();
        let hr = unsafe { ((*(*self.0)).GetDevice)(self.0 as _, riid as *const _, &mut dev) };
        if hr >= 0 {
            Ok(unsafe { core::mem::transmute_copy(&dev) })
        } else {
            Err(hr)
        }
    }
}

#[repr(C)]
pub struct IDXGIResourceVtbl {
    pub base: IDXGIDeviceSubObjectVtbl,
    pub GetSharedHandle:
        unsafe extern "system" fn(this: *mut c_void, pSharedHandle: *mut *mut c_void) -> HRESULT,
    pub GetUsage: unsafe extern "system" fn(this: *mut c_void, pUsage: *mut u32) -> HRESULT,
    pub SetEvictionPriority:
        unsafe extern "system" fn(this: *mut c_void, EvictionPriority: u32) -> HRESULT,
    pub GetEvictionPriority:
        unsafe extern "system" fn(this: *mut c_void, pEvictionPriority: *mut u32) -> HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IDXGIResource(pub *mut *const IDXGIResourceVtbl);

impl IDXGIResource {
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetSharedHandle(&self) -> Result<HANDLE, HRESULT> {
        let mut handle = core::ptr::null_mut();
        let hr = unsafe { ((*(*self.0)).GetSharedHandle)(self.0 as _, &mut handle) };
        if hr >= 0 { Ok(handle) } else { Err(hr) }
    }
    pub unsafe fn GetUsage(&self) -> Result<u32, HRESULT> {
        let mut usage = 0u32;
        let hr = unsafe { ((*(*self.0)).GetUsage)(self.0 as _, &mut usage) };
        if hr >= 0 { Ok(usage) } else { Err(hr) }
    }
    pub unsafe fn SetEvictionPriority(&self, EvictionPriority: u32) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).SetEvictionPriority)(self.0 as _, EvictionPriority) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn GetEvictionPriority(&self) -> Result<u32, HRESULT> {
        let mut priority = 0u32;
        let hr = unsafe { ((*(*self.0)).GetEvictionPriority)(self.0 as _, &mut priority) };
        if hr >= 0 { Ok(priority) } else { Err(hr) }
    }
}

#[repr(C)]
pub struct IDXGISurfaceVtbl {
    pub base: IDXGIDeviceSubObjectVtbl,
    pub GetDesc:
        unsafe extern "system" fn(this: *mut c_void, pDesc: *mut DXGI_SURFACE_DESC) -> HRESULT,
    pub Map: unsafe extern "system" fn(
        this: *mut c_void,
        pLockedRect: *mut DXGI_MAPPED_RECT,
        MapFlags: u32,
    ) -> HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IDXGISurface(pub *mut *const IDXGISurfaceVtbl);

impl IDXGISurface {
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self) -> Result<DXGI_SURFACE_DESC, HRESULT> {
        let mut desc = core::mem::MaybeUninit::uninit();
        let hr = unsafe { ((*(*self.0)).GetDesc)(self.0 as _, desc.as_mut_ptr()) };
        if hr >= 0 {
            Ok(unsafe { desc.assume_init() })
        } else {
            Err(hr)
        }
    }
    pub unsafe fn Map(&self, MapFlags: u32) -> Result<DXGI_MAPPED_RECT, HRESULT> {
        let mut rect = core::mem::MaybeUninit::uninit();
        let hr = unsafe { ((*(*self.0)).Map)(self.0 as _, rect.as_mut_ptr() as _, MapFlags) };
        if hr >= 0 {
            Ok(unsafe { rect.assume_init() })
        } else {
            Err(hr)
        }
    }
    pub unsafe fn Unmap(&self) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).Unmap)(self.0 as _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
}

#[repr(C)]
pub struct IDXGIAdapterVtbl {
    pub base: IDXGIObjectVtbl,
    pub EnumOutputs: unsafe extern "system" fn(
        this: *mut c_void,
        Output: u32,
        ppOutput: *mut *mut IDXGIOutput,
    ) -> HRESULT,
    pub GetDesc:
        unsafe extern "system" fn(this: *mut c_void, pDesc: *mut DXGI_ADAPTER_DESC) -> HRESULT,
    pub CheckInterfaceSupport: unsafe extern "system" fn(
        this: *mut c_void,
        InterfaceName: *const GUID,
        pUMDVersion: *mut i64,
    ) -> HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IDXGIAdapter(pub *mut *const IDXGIAdapterVtbl);

impl IDXGIAdapter {
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn EnumOutputs(&self, Output: u32) -> Result<IDXGIOutput, HRESULT> {
        let mut out = IDXGIOutput(core::ptr::null_mut());
        let hr =
            unsafe { ((*(*self.0)).EnumOutputs)(self.0 as _, Output, &mut out.0 as *mut _ as _) };
        if hr >= 0 { Ok(out) } else { Err(hr) }
    }
    pub unsafe fn GetDesc(&self) -> Result<DXGI_ADAPTER_DESC, HRESULT> {
        let mut desc = core::mem::MaybeUninit::uninit();
        let hr = unsafe { ((*(*self.0)).GetDesc)(self.0 as _, desc.as_mut_ptr()) };
        if hr >= 0 {
            Ok(unsafe { desc.assume_init() })
        } else {
            Err(hr)
        }
    }
    pub unsafe fn CheckInterfaceSupport(&self, InterfaceName: &GUID) -> Result<i64, HRESULT> {
        let mut ver = 0i64;
        let hr = unsafe {
            ((*(*self.0)).CheckInterfaceSupport)(self.0 as _, InterfaceName as *const _, &mut ver)
        };
        if hr >= 0 { Ok(ver) } else { Err(hr) }
    }
}

#[repr(C)]
pub struct IDXGIAdapter1Vtbl {
    pub base: IDXGIAdapterVtbl,
    pub GetDesc1:
        unsafe extern "system" fn(this: *mut c_void, pDesc: *mut DXGI_ADAPTER_DESC1) -> HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IDXGIAdapter1(pub *mut *const IDXGIAdapter1Vtbl);

impl IDXGIAdapter1 {
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc1(&self) -> Result<DXGI_ADAPTER_DESC1, HRESULT> {
        let mut desc = core::mem::MaybeUninit::uninit();
        let hr = unsafe { ((*(*self.0)).GetDesc1)(self.0 as _, desc.as_mut_ptr()) };
        if hr >= 0 {
            Ok(unsafe { desc.assume_init() })
        } else {
            Err(hr)
        }
    }
}

#[repr(C)]
pub struct IDXGIOutputVtbl {
    pub base: IDXGIObjectVtbl,
    pub GetDesc:
        unsafe extern "system" fn(this: *mut c_void, pDesc: *mut DXGI_OUTPUT_DESC) -> HRESULT,
    pub GetDisplayModeList: unsafe extern "system" fn(
        this: *mut c_void,
        EnumFormat: DXGI_FORMAT,
        Flags: u32,
        pNumModes: *mut u32,
        pDesc: *mut DXGI_MODE_DESC,
    ) -> HRESULT,
    pub FindClosestMatchingMode: unsafe extern "system" fn(
        this: *mut c_void,
        pModeToMatch: *const DXGI_MODE_DESC,
        pClosestMatch: *mut DXGI_MODE_DESC,
        pConcernedDevice: *const IUnknown,
    ) -> HRESULT,
    pub WaitForVBlank: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(
        this: *mut c_void,
        pDevice: *const IUnknown,
        Exclusive: BOOL,
    ) -> HRESULT,
    pub ReleaseOwnership: unsafe extern "system" fn(this: *mut c_void),
    pub GetGammaControlCapabilities:
        unsafe extern "system" fn(this: *mut c_void, pGammaCaps: *mut c_void) -> HRESULT,
    pub SetGammaControl:
        unsafe extern "system" fn(this: *mut c_void, pArray: *const c_void) -> HRESULT,
    pub GetGammaControl:
        unsafe extern "system" fn(this: *mut c_void, pArray: *mut c_void) -> HRESULT,
    pub SetDisplaySurface: unsafe extern "system" fn(
        this: *mut c_void,
        pScanoutSurface: *const IDXGISurface,
    ) -> HRESULT,
    pub GetDisplaySurfaceData:
        unsafe extern "system" fn(this: *mut c_void, pDestination: *const IDXGISurface) -> HRESULT,
    pub GetFrameStatistics:
        unsafe extern "system" fn(this: *mut c_void, pStats: *mut c_void) -> HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IDXGIOutput(pub *mut *const IDXGIOutputVtbl);

impl IDXGIOutput {
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self) -> Result<DXGI_OUTPUT_DESC, HRESULT> {
        let mut desc = core::mem::MaybeUninit::uninit();
        let hr = unsafe { ((*(*self.0)).GetDesc)(self.0 as _, desc.as_mut_ptr()) };
        if hr >= 0 {
            Ok(unsafe { desc.assume_init() })
        } else {
            Err(hr)
        }
    }
    pub unsafe fn GetDisplayModeList(
        &self,
        EnumFormat: DXGI_FORMAT,
        Flags: u32,
        pDesc: Option<&mut [DXGI_MODE_DESC]>,
    ) -> Result<u32, HRESULT> {
        let (desc_ptr, mut count) = match pDesc {
            Some(s) => (s.as_mut_ptr(), s.len() as u32),
            None => (core::ptr::null_mut(), 0),
        };
        let hr = unsafe {
            ((*(*self.0)).GetDisplayModeList)(self.0 as _, EnumFormat, Flags, &mut count, desc_ptr)
        };
        if hr >= 0 { Ok(count) } else { Err(hr) }
    }
    pub unsafe fn FindClosestMatchingMode(
        &self,
        pModeToMatch: &DXGI_MODE_DESC,
        pConcernedDevice: Option<&IUnknown>,
    ) -> Result<DXGI_MODE_DESC, HRESULT> {
        let dev = pConcernedDevice.map_or(core::ptr::null(), |d| d as *const _);
        let mut match_mode = core::mem::MaybeUninit::uninit();
        let hr = unsafe {
            ((*(*self.0)).FindClosestMatchingMode)(
                self.0 as _,
                pModeToMatch as *const _,
                match_mode.as_mut_ptr(),
                dev,
            )
        };
        if hr >= 0 {
            Ok(unsafe { match_mode.assume_init() })
        } else {
            Err(hr)
        }
    }
    pub unsafe fn WaitForVBlank(&self) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).WaitForVBlank)(self.0 as _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn TakeOwnership(&self, pDevice: &IUnknown, Exclusive: BOOL) -> Result<(), HRESULT> {
        let hr =
            unsafe { ((*(*self.0)).TakeOwnership)(self.0 as _, pDevice as *const _, Exclusive) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn ReleaseOwnership(&self) {
        unsafe { ((*(*self.0)).ReleaseOwnership)(self.0 as _) }
    }
    pub unsafe fn SetDisplaySurface(&self, pScanoutSurface: &IDXGISurface) -> Result<(), HRESULT> {
        let hr =
            unsafe { ((*(*self.0)).SetDisplaySurface)(self.0 as _, pScanoutSurface as *const _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn GetDisplaySurfaceData(&self, pDestination: &IDXGISurface) -> Result<(), HRESULT> {
        let hr =
            unsafe { ((*(*self.0)).GetDisplaySurfaceData)(self.0 as _, pDestination as *const _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
}

#[repr(C)]
pub struct IDXGISwapChainVtbl {
    pub base: IDXGIDeviceSubObjectVtbl,
    pub Present:
        unsafe extern "system" fn(this: *mut c_void, SyncInterval: u32, Flags: u32) -> HRESULT,
    pub GetBuffer: unsafe extern "system" fn(
        this: *mut c_void,
        Buffer: u32,
        riid: *const GUID,
        ppSurface: *mut *mut c_void,
    ) -> HRESULT,
    pub SetFullscreenState: unsafe extern "system" fn(
        this: *mut c_void,
        Fullscreen: BOOL,
        pTarget: *mut IDXGIOutput,
    ) -> HRESULT,
    pub GetFullscreenState: unsafe extern "system" fn(
        this: *mut c_void,
        pFullscreen: *mut BOOL,
        ppTarget: *mut *mut IDXGIOutput,
    ) -> HRESULT,
    pub GetDesc:
        unsafe extern "system" fn(this: *mut c_void, pDesc: *mut DXGI_SWAP_CHAIN_DESC) -> HRESULT,
    pub ResizeBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        BufferCount: u32,
        Width: u32,
        Height: u32,
        NewFormat: DXGI_FORMAT,
        SwapChainFlags: u32,
    ) -> HRESULT,
    pub ResizeTarget: unsafe extern "system" fn(
        this: *mut c_void,
        pNewTargetParameters: *const DXGI_MODE_DESC,
    ) -> HRESULT,
    pub GetContainingOutput:
        unsafe extern "system" fn(this: *mut c_void, ppOutput: *mut *mut IDXGIOutput) -> HRESULT,
    pub GetFrameStatistics:
        unsafe extern "system" fn(this: *mut c_void, pStats: *mut c_void) -> HRESULT,
    pub GetLastPresentCount:
        unsafe extern "system" fn(this: *mut c_void, pLastPresentCount: *mut u32) -> HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IDXGISwapChain(pub *mut *const IDXGISwapChainVtbl);

impl IDXGISwapChain {
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn Present(&self, SyncInterval: u32, Flags: u32) -> Result<(), HRESULT> {
        let hr = unsafe { ((*(*self.0)).Present)(self.0 as _, SyncInterval, Flags) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn GetBuffer<T>(&self, Buffer: u32, riid: &GUID) -> Result<T, HRESULT> {
        let mut surface = core::ptr::null_mut();
        let hr = unsafe {
            ((*(*self.0)).GetBuffer)(self.0 as _, Buffer, riid as *const _, &mut surface)
        };
        if hr >= 0 {
            Ok(unsafe { core::mem::transmute_copy(&surface) })
        } else {
            Err(hr)
        }
    }
    pub unsafe fn GetDesc(&self) -> Result<DXGI_SWAP_CHAIN_DESC, HRESULT> {
        let mut desc = core::mem::MaybeUninit::uninit();
        let hr = unsafe { ((*(*self.0)).GetDesc)(self.0 as _, desc.as_mut_ptr()) };
        if hr >= 0 {
            Ok(unsafe { desc.assume_init() })
        } else {
            Err(hr)
        }
    }
    pub unsafe fn ResizeBuffers(
        &self,
        BufferCount: u32,
        Width: u32,
        Height: u32,
        NewFormat: DXGI_FORMAT,
        SwapChainFlags: u32,
    ) -> Result<(), HRESULT> {
        let hr = unsafe {
            ((*(*self.0)).ResizeBuffers)(
                self.0 as _,
                BufferCount,
                Width,
                Height,
                NewFormat,
                SwapChainFlags,
            )
        };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn SetFullscreenState(
        &self,
        Fullscreen: BOOL,
        pTarget: Option<&IDXGIOutput>,
    ) -> Result<(), HRESULT> {
        let target = pTarget.map_or(core::ptr::null_mut(), |t| t.0 as _);
        let hr = unsafe { ((*(*self.0)).SetFullscreenState)(self.0 as _, Fullscreen, target) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn GetFullscreenState(&self) -> Result<(BOOL, Option<IDXGIOutput>), HRESULT> {
        let mut fs = 0;
        let mut target = IDXGIOutput(core::ptr::null_mut());
        let hr = unsafe {
            ((*(*self.0)).GetFullscreenState)(self.0 as _, &mut fs, &mut target.0 as *mut _ as _)
        };
        if hr >= 0 {
            let t_opt = if !target.0.is_null() {
                Some(target)
            } else {
                None
            };
            Ok((fs, t_opt))
        } else {
            Err(hr)
        }
    }
    pub unsafe fn ResizeTarget(
        &self,
        pNewTargetParameters: &DXGI_MODE_DESC,
    ) -> Result<(), HRESULT> {
        let hr =
            unsafe { ((*(*self.0)).ResizeTarget)(self.0 as _, pNewTargetParameters as *const _) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn GetContainingOutput(&self) -> Result<IDXGIOutput, HRESULT> {
        let mut output = IDXGIOutput(core::ptr::null_mut());
        let hr = unsafe {
            ((*(*self.0)).GetContainingOutput)(self.0 as _, &mut output.0 as *mut _ as _)
        };
        if hr >= 0 { Ok(output) } else { Err(hr) }
    }
    pub unsafe fn GetLastPresentCount(&self) -> Result<u32, HRESULT> {
        let mut count = 0u32;
        let hr = unsafe { ((*(*self.0)).GetLastPresentCount)(self.0 as _, &mut count) };
        if hr >= 0 { Ok(count) } else { Err(hr) }
    }
}

#[repr(C)]
pub struct IDXGIFactoryVtbl {
    pub base: IDXGIObjectVtbl,
    pub EnumAdapters: unsafe extern "system" fn(
        this: *mut c_void,
        Adapter: u32,
        ppAdapter: *mut *mut IDXGIAdapter,
    ) -> HRESULT,
    pub MakeWindowAssociation:
        unsafe extern "system" fn(this: *mut c_void, WindowHandle: HWND, Flags: u32) -> HRESULT,
    pub GetWindowAssociation:
        unsafe extern "system" fn(this: *mut c_void, pWindowHandle: *mut HWND) -> HRESULT,
    pub CreateSwapChain: unsafe extern "system" fn(
        this: *mut c_void,
        pDevice: *mut c_void,
        pDesc: *const DXGI_SWAP_CHAIN_DESC,
        ppSwapChain: *mut *mut IDXGISwapChain,
    ) -> HRESULT,
    pub CreateSoftwareAdapter: unsafe extern "system" fn(
        this: *mut c_void,
        Module: *mut c_void,
        ppAdapter: *mut *mut IDXGIAdapter,
    ) -> HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IDXGIFactory(pub *mut *const IDXGIFactoryVtbl);

impl IDXGIFactory {
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn EnumAdapters(&self, Adapter: u32) -> Result<IDXGIAdapter, HRESULT> {
        let mut ad = IDXGIAdapter(core::ptr::null_mut());
        let hr =
            unsafe { ((*(*self.0)).EnumAdapters)(self.0 as _, Adapter, &mut ad.0 as *mut _ as _) };
        if hr >= 0 { Ok(ad) } else { Err(hr) }
    }
    pub unsafe fn MakeWindowAssociation(
        &self,
        WindowHandle: Option<HWND>,
        Flags: u32,
    ) -> Result<(), HRESULT> {
        let hwnd_ptr = WindowHandle.unwrap_or(core::ptr::null_mut());
        let hr = unsafe { ((*(*self.0)).MakeWindowAssociation)(self.0 as _, hwnd_ptr, Flags) };
        if hr >= 0 { Ok(()) } else { Err(hr) }
    }
    pub unsafe fn GetWindowAssociation(&self) -> Result<HWND, HRESULT> {
        let mut hwnd = core::ptr::null_mut();
        let hr = unsafe { ((*(*self.0)).GetWindowAssociation)(self.0 as _, &mut hwnd) };
        if hr >= 0 { Ok(hwnd) } else { Err(hr) }
    }
    pub unsafe fn CreateSwapChain(
        &self,
        pDevice: &IUnknown,
        pDesc: &DXGI_SWAP_CHAIN_DESC,
    ) -> Result<IDXGISwapChain, HRESULT> {
        let mut sc = IDXGISwapChain(core::ptr::null_mut());
        let hr = unsafe {
            ((*(*self.0)).CreateSwapChain)(
                self.0 as _,
                pDevice.0 as _,
                pDesc as *const _,
                &mut sc.0 as *mut _ as _,
            )
        };
        if hr >= 0 { Ok(sc) } else { Err(hr) }
    }
    pub unsafe fn CreateSoftwareAdapter(
        &self,
        Module: Option<HMODULE>,
    ) -> Result<IDXGIAdapter, HRESULT> {
        let mod_ptr = Module.unwrap_or(core::ptr::null_mut());
        let mut ad = IDXGIAdapter(core::ptr::null_mut());
        let hr = unsafe {
            ((*(*self.0)).CreateSoftwareAdapter)(self.0 as _, mod_ptr, &mut ad.0 as *mut _ as _)
        };
        if hr >= 0 { Ok(ad) } else { Err(hr) }
    }
}

#[repr(C)]
pub struct IDXGIFactory1Vtbl {
    pub base: IDXGIFactoryVtbl,
    pub EnumAdapters1: unsafe extern "system" fn(
        this: *mut c_void,
        Adapter: u32,
        ppAdapter: *mut *mut IDXGIAdapter1,
    ) -> HRESULT,
    pub IsCurrent: unsafe extern "system" fn(this: *mut c_void) -> BOOL,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IDXGIFactory1(pub *mut *const IDXGIFactory1Vtbl);

impl IDXGIFactory1 {
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn EnumAdapters1(&self, Adapter: u32) -> Result<IDXGIAdapter1, HRESULT> {
        let mut ad = IDXGIAdapter1(core::ptr::null_mut());
        let hr =
            unsafe { ((*(*self.0)).EnumAdapters1)(self.0 as _, Adapter, &mut ad.0 as *mut _ as _) };
        if hr >= 0 { Ok(ad) } else { Err(hr) }
    }
    pub unsafe fn IsCurrent(&self) -> BOOL {
        unsafe { ((*(*self.0)).IsCurrent)(self.0 as _) }
    }
}

mod ffi {
    use super::*;
    #[link(name = "dxgi")]
    unsafe extern "system" {
        pub fn CreateDXGIFactory(riid: *const GUID, ppFactory: *mut *mut c_void) -> HRESULT;
        pub fn CreateDXGIFactory1(riid: *const GUID, ppFactory: *mut *mut c_void) -> HRESULT;
        pub fn CreateDXGIFactory2(
            Flags: u32,
            riid: *const GUID,
            ppFactory: *mut *mut c_void,
        ) -> HRESULT;
    }
}

pub unsafe fn CreateDXGIFactory<T>(riid: &GUID) -> Result<T, HRESULT> {
    let mut factory = core::ptr::null_mut();
    let hr = unsafe { ffi::CreateDXGIFactory(riid as *const _, &mut factory) };
    if hr >= 0 {
        Ok(unsafe { core::mem::transmute_copy(&factory) })
    } else {
        Err(hr)
    }
}

pub unsafe fn CreateDXGIFactory1<T>(riid: &GUID) -> Result<T, HRESULT> {
    let mut factory = core::ptr::null_mut();
    let hr = unsafe { ffi::CreateDXGIFactory1(riid as *const _, &mut factory) };
    if hr >= 0 {
        Ok(unsafe { core::mem::transmute_copy(&factory) })
    } else {
        Err(hr)
    }
}

pub unsafe fn CreateDXGIFactory2<T>(Flags: u32, riid: &GUID) -> Result<T, HRESULT> {
    let mut factory = core::ptr::null_mut();
    let hr = unsafe { ffi::CreateDXGIFactory2(Flags, riid as *const _, &mut factory) };
    if hr >= 0 {
        Ok(unsafe { core::mem::transmute_copy(&factory) })
    } else {
        Err(hr)
    }
}
