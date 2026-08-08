use crate::types::{GUID, HRESULT, IUnknown, IUnknownVtbl};
use core::ffi::c_void;

pub const IID_ID3D10BLOB: GUID = GUID::from_u128(0x8ba5fb08_5195_40e2_ac58_0d989c3a0102);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D_DRIVER_TYPE {
    UNKNOWN = 0,
    HARDWARE = 1,
    REFERENCE = 2,
    NULL = 3,
    SOFTWARE = 4,
    WARP = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum D3D_FEATURE_LEVEL {
    _1_0_CORE = 0x1000,
    _9_1 = 0x9100,
    _9_2 = 0x9200,
    _9_3 = 0x9300,
    _10_0 = 0xa000,
    _10_1 = 0xa100,
    _11_0 = 0xb000,
    _11_1 = 0xb100,
    _12_0 = 0xc000,
    _12_1 = 0xc100,
    _12_2 = 0xc200,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D_PRIMITIVE_TOPOLOGY {
    UNDEFINED = 0,
    POINTLIST = 1,
    LINELIST = 2,
    LINESTRIP = 3,
    TRIANGLELIST = 4,
    TRIANGLESTRIP = 5,
    LINELIST_ADJ = 10,
    LINESTRIP_ADJ = 11,
    TRIANGLELIST_ADJ = 12,
    TRIANGLESTRIP_ADJ = 13,
    _1_CONTROL_POINT_PATCHLIST = 33,
    _2_CONTROL_POINT_PATCHLIST = 34,
    _3_CONTROL_POINT_PATCHLIST = 35,
    _4_CONTROL_POINT_PATCHLIST = 36,
    _5_CONTROL_POINT_PATCHLIST = 37,
    _6_CONTROL_POINT_PATCHLIST = 38,
    _7_CONTROL_POINT_PATCHLIST = 39,
    _8_CONTROL_POINT_PATCHLIST = 40,
    _9_CONTROL_POINT_PATCHLIST = 41,
    _10_CONTROL_POINT_PATCHLIST = 42,
    _11_CONTROL_POINT_PATCHLIST = 43,
    _12_CONTROL_POINT_PATCHLIST = 44,
    _13_CONTROL_POINT_PATCHLIST = 45,
    _14_CONTROL_POINT_PATCHLIST = 46,
    _15_CONTROL_POINT_PATCHLIST = 47,
    _16_CONTROL_POINT_PATCHLIST = 48,
    _17_CONTROL_POINT_PATCHLIST = 49,
    _18_CONTROL_POINT_PATCHLIST = 50,
    _19_CONTROL_POINT_PATCHLIST = 51,
    _20_CONTROL_POINT_PATCHLIST = 52,
    _21_CONTROL_POINT_PATCHLIST = 53,
    _22_CONTROL_POINT_PATCHLIST = 54,
    _23_CONTROL_POINT_PATCHLIST = 55,
    _24_CONTROL_POINT_PATCHLIST = 56,
    _25_CONTROL_POINT_PATCHLIST = 57,
    _26_CONTROL_POINT_PATCHLIST = 58,
    _27_CONTROL_POINT_PATCHLIST = 59,
    _28_CONTROL_POINT_PATCHLIST = 60,
    _29_CONTROL_POINT_PATCHLIST = 61,
    _30_CONTROL_POINT_PATCHLIST = 62,
    _31_CONTROL_POINT_PATCHLIST = 63,
    _32_CONTROL_POINT_PATCHLIST = 64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D_SHADER_MACRO {
    pub Name: *const u8,
    pub Definition: *const u8,
}

#[repr(C)]
pub struct ID3D10BlobVtbl {
    pub base: IUnknownVtbl,
    pub GetBufferPointer: unsafe extern "system" fn(this: *mut c_void) -> *mut c_void,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut c_void) -> usize,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D10Blob(pub *mut *const ID3D10BlobVtbl);

pub type ID3DBlob = ID3D10Blob;

impl ID3D10Blob {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        let unk = IUnknown(self.0 as _);
        unsafe { unk.QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        let unk = IUnknown(self.0 as _);
        unsafe { unk.AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        let unk = IUnknown(self.0 as _);
        unsafe { unk.Release() }
    }
    pub unsafe fn GetBufferPointer(&self) -> *mut c_void {
        unsafe { ((*(*self.0)).GetBufferPointer)(self.0 as _) }
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        unsafe { ((*(*self.0)).GetBufferSize)(self.0 as _) }
    }
}
