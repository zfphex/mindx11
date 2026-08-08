use core::ffi::c_void;

pub type HRESULT = i32;
pub type BOOL = i32;
pub type HMODULE = *mut c_void;
pub type HWND = *mut c_void;
pub type HANDLE = *mut c_void;
pub type LUID = [u8; 8];

pub const S_OK: HRESULT = 0;
pub const E_FAIL: HRESULT = -2147467259; // 0x80004005
pub const E_NOINTERFACE: HRESULT = -2147467262; // 0x80004002
pub const E_POINTER: HRESULT = -2147467261; // 0x80004003
pub const E_INVALIDARG: HRESULT = -2147024809; // 0x80070057
pub const E_OUTOFMEMORY: HRESULT = -2147024882; // 0x8007000E

pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        let b = uuid.to_be_bytes();
        Self {
            data1: u32::from_be_bytes([b[0], b[1], b[2], b[3]]),
            data2: u16::from_be_bytes([b[4], b[5]]),
            data3: u16::from_be_bytes([b[6], b[7]]),
            data4: [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]],
        }
    }
}

pub const IID_IUNKNOWN: GUID = GUID::from_u128(0x00000000_0000_0000_c000_00000000046);

#[repr(C)]
pub struct IUnknownVtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut c_void,
        riid: *const GUID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut c_void) -> u32,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IUnknown(pub *mut *const IUnknownVtbl);

impl IUnknown {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe {
            ((*(*self.0)).QueryInterface)(self.0 as _, riid as *const _, ppvObject as *mut _ as _)
        }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { ((*(*self.0)).AddRef)(self.0 as _) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { ((*(*self.0)).Release)(self.0 as _) }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
