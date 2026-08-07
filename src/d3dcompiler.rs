use crate::d3dcommon::{D3D_SHADER_MACRO, ID3D10Blob};
use crate::types::HRESULT;
use core::ffi::c_void;

pub const D3DCOMPILE_DEBUG: u32 = 1 << 0;
pub const D3DCOMPILE_SKIP_VALIDATION: u32 = 1 << 1;
pub const D3DCOMPILE_SKIP_OPTIMIZATION: u32 = 1 << 2;
pub const D3DCOMPILE_PACK_MATRIX_ROW_MAJOR: u32 = 1 << 3;
pub const D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR: u32 = 1 << 4;
pub const D3DCOMPILE_PARTIAL_PRECISION: u32 = 1 << 5;
pub const D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT: u32 = 1 << 6;
pub const D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT: u32 = 1 << 7;
pub const D3DCOMPILE_NO_PRESHADER: u32 = 1 << 8;
pub const D3DCOMPILE_AVOID_FLOW_CONTROL: u32 = 1 << 9;
pub const D3DCOMPILE_PREFER_FLOW_CONTROL: u32 = 1 << 10;
pub const D3DCOMPILE_ENABLE_STRICTNESS: u32 = 1 << 11;
pub const D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 1 << 12;
pub const D3DCOMPILE_IEEE_STRICTNESS: u32 = 1 << 13;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL0: u32 = 1 << 14;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL1: u32 = 0;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL2: u32 = (1 << 14) | (1 << 15);
pub const D3DCOMPILE_OPTIMIZATION_LEVEL3: u32 = 1 << 15;
pub const D3DCOMPILE_WARNINGS_ARE_ERRORS: u32 = 1 << 18;

#[link(name = "d3dcompiler")]
unsafe extern "system" {
    pub fn D3DCompile(
        pSrcData: *const c_void,
        SrcDataSize: usize,
        pSourceName: *const u8,
        pDefines: *const D3D_SHADER_MACRO,
        pInclude: *mut c_void,
        pEntrypoint: *const u8,
        pTarget: *const u8,
        Flags1: u32,
        Flags2: u32,
        ppCode: *mut *mut ID3D10Blob,
        ppErrorMsgs: *mut *mut ID3D10Blob,
    ) -> HRESULT;

    pub fn D3DCompile2(
        pSrcData: *const c_void,
        SrcDataSize: usize,
        pSourceName: *const u8,
        pDefines: *const D3D_SHADER_MACRO,
        pInclude: *mut c_void,
        pEntrypoint: *const u8,
        pTarget: *const u8,
        Flags1: u32,
        Flags2: u32,
        SecondaryDataFlags: u32,
        pSecondaryData: *const c_void,
        SecondaryDataSize: usize,
        ppCode: *mut *mut ID3D10Blob,
        ppErrorMsgs: *mut *mut ID3D10Blob,
    ) -> HRESULT;

    pub fn D3DCompileFromFile(
        pFileName: *const u16,
        pDefines: *const D3D_SHADER_MACRO,
        pInclude: *mut c_void,
        pEntrypoint: *const u8,
        pTarget: *const u8,
        Flags1: u32,
        Flags2: u32,
        ppCode: *mut *mut ID3D10Blob,
        ppErrorMsgs: *mut *mut ID3D10Blob,
    ) -> HRESULT;
}
