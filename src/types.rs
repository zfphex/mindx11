use core::ffi::c_void;

pub type BOOL = i32;
pub type HMODULE = *mut c_void;
pub type HWND = *mut c_void;
pub type HANDLE = *mut c_void;
pub type LUID = [u8; 8];

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct HRESULT(pub i32);

pub type HResult = HRESULT;

impl HRESULT {
    pub const S_OK: Self = Self(0);
    pub const E_FAIL: Self = Self(-2147467259); // 0x80004005
    pub const E_NOINTERFACE: Self = Self(-2147467262); // 0x80004002
    pub const E_POINTER: Self = Self(-2147467261); // 0x80004003
    pub const E_INVALIDARG: Self = Self(-2147024809); // 0x80070057
    pub const E_OUTOFMEMORY: Self = Self(-2147024882); // 0x8007000E

    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 >= 0
    }

    #[inline]
    pub const fn is_err(self) -> bool {
        self.0 < 0
    }

    #[inline]
    pub const fn code(self) -> u32 {
        self.0 as u32
    }

    pub fn name(self) -> Option<&'static str> {
        match self.0 {
            0 => Some("S_OK"),
            -2147467259 => Some("E_FAIL"),
            -2147467262 => Some("E_NOINTERFACE"),
            -2147467261 => Some("E_POINTER"),
            -2147024809 => Some("E_INVALIDARG"),
            -2147024882 => Some("E_OUTOFMEMORY"),
            -2005270523 => Some("DXGI_ERROR_DEVICE_REMOVED"),
            -2005270522 => Some("DXGI_ERROR_DEVICE_HUNG"),
            -2005270521 => Some("DXGI_ERROR_DEVICE_RESET"),
            -2005270527 => Some("DXGI_ERROR_INVALID_CALL"),
            -2005270524 => Some("DXGI_ERROR_UNSUPPORTED"),
            -2005270520 => Some("DXGI_ERROR_DRIVER_INTERNAL_ERROR"),
            -2005270519 => Some("DXGI_ERROR_NONEXCLUSIVE"),
            _ => None,
        }
    }

    pub fn os_message(self) -> Option<String> {
        use core::ptr::null_mut;

        unsafe extern "system" {
            fn FormatMessageW(
                flags: u32,
                source: *const (),
                id: u32,
                lang_id: u32,
                buf: *mut u16,
                size: u32,
                args: *const (),
            ) -> u32;
            fn LocalFree(hmem: *mut ()) -> *mut ();
        }

        const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0x00000100;
        const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
        const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;

        let mut buf: *mut u16 = null_mut();
        let len = unsafe {
            FormatMessageW(
                FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS,
                null_mut(),
                self.code(),
                0,
                &mut buf as *mut _ as *mut u16,
                0,
                null_mut(),
            )
        };

        if len > 0 && !buf.is_null() {
            let slice = unsafe { core::slice::from_raw_parts(buf, len as usize) };
            let msg = String::from_utf16_lossy(slice).trim().to_string();
            unsafe { LocalFree(buf as _) };
            Some(msg)
        } else {
            None
        }
    }
}

impl core::fmt::Debug for HRESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(name) = self.name() {
            write!(f, "HRESULT(0x{:08X} - {})", self.code(), name)
        } else {
            write!(f, "HRESULT(0x{:08X})", self.code())
        }
    }
}

impl core::fmt::Display for HRESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(name) = self.name() {
            #[cfg(target_os = "windows")]
            if let Some(msg) = self.os_message() {
                return write!(f, "HRESULT 0x{:08X} ({}): {}", self.code(), name, msg);
            }
            write!(f, "HRESULT 0x{:08X} ({})", self.code(), name)
        } else {
            #[cfg(target_os = "windows")]
            if let Some(msg) = self.os_message() {
                return write!(f, "HRESULT 0x{:08X}: {}", self.code(), msg);
            }
            write!(f, "HRESULT 0x{:08X}", self.code())
        }
    }
}

impl std::error::Error for HRESULT {}

impl PartialEq<i32> for HRESULT {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<i32> for HRESULT {
    fn partial_cmp(&self, other: &i32) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<HRESULT> for i32 {
    fn eq(&self, other: &HRESULT) -> bool {
        *self == other.0
    }
}

impl PartialOrd<HRESULT> for i32 {
    fn partial_cmp(&self, other: &HRESULT) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl From<i32> for HRESULT {
    fn from(code: i32) -> Self {
        HRESULT(code)
    }
}

impl From<HRESULT> for i32 {
    fn from(hr: HRESULT) -> Self {
        hr.0
    }
}

pub trait HResultExt<T> {
    fn expect_hr(self, msg: &str) -> T;
}

impl<T> HResultExt<T> for Result<T, HRESULT> {
    #[track_caller]
    fn expect_hr(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(hr) => panic!("{}: {}", msg, hr),
        }
    }
}


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
    pub unsafe fn QueryInterface<T>(&self, riid: &GUID) -> Result<T, HRESULT> {
        let mut obj = core::ptr::null_mut();
        let hr = unsafe { ((*(*self.0)).QueryInterface)(self.0 as _, riid as *const _, &mut obj) };
        if hr >= 0 {
            Ok(unsafe { core::mem::transmute_copy(&obj) })
        } else {
            Err(hr)
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
