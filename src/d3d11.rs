use crate::d3dcommon::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL, D3D_PRIMITIVE_TOPOLOGY};
use crate::dxgi::{DXGI_FORMAT, DXGI_SAMPLE_DESC, DXGI_SWAP_CHAIN_DESC, IDXGISwapChain};
use crate::types::{BOOL, GUID, HRESULT, IUnknown, IUnknownVtbl, RECT};
use core::ffi::c_void;

pub const IID_ID3D11DEVICECHILD: GUID = GUID::from_u128(0x1841e5c8_16b0_489b_bcc8_44cfb0d5deae);
pub const IID_ID3D11RESOURCE: GUID = GUID::from_u128(0xdc8e63ed_035f_4b1c_9552_43e674bcee61);
pub const IID_ID3D11BUFFER: GUID = GUID::from_u128(0x48574479_4fbf_4ea0_a519_a350ba861191);
pub const IID_ID3D11TEXTURE1D: GUID = GUID::from_u128(0xf82149a9_0544_476c_802e_b1b38d019d6e);
pub const IID_ID3D11TEXTURE2D: GUID = GUID::from_u128(0x6f15ba80_3153_496b_9415_a87131b4b941);
pub const IID_ID3D11TEXTURE3D: GUID = GUID::from_u128(0x03741620_4b02_439c_8126_f77b9f624239);
pub const IID_ID3D11VIEW: GUID = GUID::from_u128(0x839d1216_bb2e_412b_b7f4_a9dbebe08ed1);
pub const IID_ID3D11SHADERRESOURCEVIEW: GUID =
    GUID::from_u128(0xb0e06500_817c_4655_8ff1_11700f281446);
pub const IID_ID3D11RENDERTARGETVIEW: GUID =
    GUID::from_u128(0xdf7e015c_b711_4d9f_9f16_a40d30a4cb05);
pub const IID_ID3D11DEPTHSTENCILVIEW: GUID =
    GUID::from_u128(0x9fd40e0b_912f_4016_84e8_036317dbbcc0);
pub const IID_ID3D11UNORDEREDACCESSVIEW: GUID =
    GUID::from_u128(0x28acf50f_b637_400c_9815_f040d911a78b);
pub const IID_ID3D11VERTEXSHADER: GUID = GUID::from_u128(0x3b301d64_d678_4289_8897_22f8928b72f3);
pub const IID_ID3D11HULLSHADER: GUID = GUID::from_u128(0x8e5c6061_628a_4c8e_8264_bbe45cb3d5dd);
pub const IID_ID3D11DOMAINSHADER: GUID = GUID::from_u128(0xf582c508_00e0_4574_9009_d6ac43401226);
pub const IID_ID3D11GEOMETRYSHADER: GUID = GUID::from_u128(0x38217267_9b65_4d2f_b332_03429a1e0c7e);
pub const IID_ID3D11PIXELSHADER: GUID = GUID::from_u128(0xea82e40d_51dc_4f33_93d4_db7c9125ae8c);
pub const IID_ID3D11COMPUTESHADER: GUID = GUID::from_u128(0x4f5b196e_c2bd_495e_bd01_1fded38e4969);
pub const IID_ID3D11INPUTLAYOUT: GUID = GUID::from_u128(0xe4819266_5340_4996_8180_be58ab1119c2);
pub const IID_ID3D11BLENDSTATE: GUID = GUID::from_u128(0x75baa345_0dbb_4035_8697_225ae6e6677f);
pub const IID_ID3D11DEPTHSTENCILSTATE: GUID =
    GUID::from_u128(0x03823d0a_b402_480d_8077_2b08b30f4f30);
pub const IID_ID3D11RASTERIZERSTATE: GUID = GUID::from_u128(0x9bb4b882_d163_4776_b381_4b0487a90344);
pub const IID_ID3D11SAMPLERSTATE: GUID = GUID::from_u128(0xda6fe7da_435b_4748_a232_5921225d1144);
pub const IID_ID3D11QUERY: GUID = GUID::from_u128(0xd6c00747_87b7_425e_b84d_44d1965a3d42);
pub const IID_ID3D11DEVICECONTEXT: GUID = GUID::from_u128(0xc0bfa96c_e089_44fb_8eaf_26f8796190da);
pub const IID_ID3D11DEVICE: GUID = GUID::from_u128(0xdb6f6ddb_ac77_4e88_8253_819df9bbf140);

pub const D3D11_SDK_VERSION: u32 = 7;

pub const D3D11_CREATE_DEVICE_SINGLETHREADED: u32 = 0x1;
pub const D3D11_CREATE_DEVICE_DEBUG: u32 = 0x2;
pub const D3D11_CREATE_DEVICE_SWITCH_TO_REF: u32 = 0x4;
pub const D3D11_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: u32 = 0x8;
pub const D3D11_CREATE_DEVICE_BGRA_SUPPORT: u32 = 0x20;
pub const D3D11_CREATE_DEVICE_DEBUGGABLE: u32 = 0x40;
pub const D3D11_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: u32 = 0x80;
pub const D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT: u32 = 0x100;
pub const D3D11_CREATE_DEVICE_VIDEO_SUPPORT: u32 = 0x800;

pub const D3D11_BIND_VERTEX_BUFFER: u32 = 0x1;
pub const D3D11_BIND_INDEX_BUFFER: u32 = 0x2;
pub const D3D11_BIND_CONSTANT_BUFFER: u32 = 0x4;
pub const D3D11_BIND_SHADER_RESOURCE: u32 = 0x8;
pub const D3D11_BIND_STREAM_OUTPUT: u32 = 0x10;
pub const D3D11_BIND_RENDER_TARGET: u32 = 0x20;
pub const D3D11_BIND_DEPTH_STENCIL: u32 = 0x40;
pub const D3D11_BIND_UNORDERED_ACCESS: u32 = 0x80;
pub const D3D11_BIND_DECODER: u32 = 0x200;
pub const D3D11_BIND_VIDEO_ENCODER: u32 = 0x400;

pub const D3D11_CPU_ACCESS_WRITE: u32 = 0x10000;
pub const D3D11_CPU_ACCESS_READ: u32 = 0x20000;

pub const D3D11_RESOURCE_MISC_GENERATE_MIPS: u32 = 0x1;
pub const D3D11_RESOURCE_MISC_SHARED: u32 = 0x2;
pub const D3D11_RESOURCE_MISC_TEXTURECUBE: u32 = 0x4;
pub const D3D11_RESOURCE_MISC_DRAWINDIRECT_ARGS: u32 = 0x10;
pub const D3D11_RESOURCE_MISC_BUFFER_ALLOW_RAW_VIEWS: u32 = 0x20;
pub const D3D11_RESOURCE_MISC_BUFFER_STRUCTURED: u32 = 0x40;
pub const D3D11_RESOURCE_MISC_RESOURCE_CLAMP: u32 = 0x80;
pub const D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX: u32 = 0x100;
pub const D3D11_RESOURCE_MISC_GDI_COMPATIBLE: u32 = 0x200;

pub const D3D11_COLOR_WRITE_ENABLE_RED: u8 = 1;
pub const D3D11_COLOR_WRITE_ENABLE_GREEN: u8 = 2;
pub const D3D11_COLOR_WRITE_ENABLE_BLUE: u8 = 4;
pub const D3D11_COLOR_WRITE_ENABLE_ALPHA: u8 = 8;
pub const D3D11_COLOR_WRITE_ENABLE_ALL: u8 = D3D11_COLOR_WRITE_ENABLE_RED
    | D3D11_COLOR_WRITE_ENABLE_GREEN
    | D3D11_COLOR_WRITE_ENABLE_BLUE
    | D3D11_COLOR_WRITE_ENABLE_ALPHA;

pub const D3D11_CLEAR_DEPTH: u32 = 0x1;
pub const D3D11_CLEAR_STENCIL: u32 = 0x2;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_USAGE {
    DEFAULT = 0,
    IMMUTABLE = 1,
    DYNAMIC = 2,
    STAGING = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_MAP {
    READ = 1,
    WRITE = 2,
    READ_WRITE = 3,
    WRITE_DISCARD = 4,
    WRITE_NO_OVERWRITE = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_FILL_MODE {
    WIREFRAME = 2,
    SOLID = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_CULL_MODE {
    NONE = 1,
    FRONT = 2,
    BACK = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_FILTER {
    MIN_MAG_MIP_POINT = 0,
    MIN_MAG_POINT_MIP_LINEAR = 0x1,
    MIN_POINT_MAG_LINEAR_MIP_POINT = 0x4,
    MIN_POINT_MAG_MIP_LINEAR = 0x5,
    MIN_LINEAR_MAG_MIP_POINT = 0x10,
    MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x11,
    MIN_MAG_LINEAR_MIP_POINT = 0x14,
    MIN_MAG_MIP_LINEAR = 0x15,
    ANISOTROPIC = 0x55,
    COMPARISON_MIN_MAG_MIP_POINT = 0x80,
    COMPARISON_MIN_MAG_POINT_MIP_LINEAR = 0x81,
    COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT = 0x84,
    COMPARISON_MIN_POINT_MAG_MIP_LINEAR = 0x85,
    COMPARISON_MIN_LINEAR_MAG_MIP_POINT = 0x90,
    COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x91,
    COMPARISON_MIN_MAG_LINEAR_MIP_POINT = 0x94,
    COMPARISON_MIN_MAG_MIP_LINEAR = 0x95,
    COMPARISON_ANISOTROPIC = 0xd5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_TEXTURE_ADDRESS_MODE {
    WRAP = 1,
    MIRROR = 2,
    CLAMP = 3,
    BORDER = 4,
    MIRROR_ONCE = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_COMPARISON_FUNC {
    NEVER = 1,
    LESS = 2,
    EQUAL = 3,
    LESS_EQUAL = 4,
    GREATER = 5,
    NOT_EQUAL = 6,
    GREATER_EQUAL = 7,
    ALWAYS = 8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_DEPTH_WRITE_MASK {
    ZERO = 0,
    ALL = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_STENCIL_OP {
    KEEP = 1,
    ZERO = 2,
    REPLACE = 3,
    INCR_SAT = 4,
    DECR_SAT = 5,
    INVERT = 6,
    INCR = 7,
    DECR = 8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_BLEND {
    ZERO = 1,
    ONE = 2,
    SRC_COLOR = 3,
    INV_SRC_COLOR = 4,
    SRC_ALPHA = 5,
    INV_SRC_ALPHA = 6,
    DEST_ALPHA = 7,
    INV_DEST_ALPHA = 8,
    DEST_COLOR = 9,
    INV_DEST_COLOR = 10,
    SRC_ALPHA_SAT = 11,
    BLEND_FACTOR = 14,
    INV_BLEND_FACTOR = 15,
    SRC1_COLOR = 16,
    INV_SRC1_COLOR = 17,
    SRC1_ALPHA = 18,
    INV_SRC1_ALPHA = 19,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_BLEND_OP {
    ADD = 1,
    SUBTRACT = 2,
    REV_SUBTRACT = 3,
    MIN = 4,
    MAX = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_INPUT_CLASSIFICATION {
    PER_VERTEX_DATA = 0,
    PER_INSTANCE_DATA = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_RTV_DIMENSION {
    UNKNOWN = 0,
    BUFFER = 1,
    TEXTURE1D = 2,
    TEXTURE1DARRAY = 3,
    TEXTURE2D = 4,
    TEXTURE2DARRAY = 5,
    TEXTURE2DMS = 6,
    TEXTURE2DMSARRAY = 7,
    TEXTURE3D = 8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3D11_DSV_DIMENSION {
    UNKNOWN = 0,
    TEXTURE1D = 1,
    TEXTURE1DARRAY = 2,
    TEXTURE2D = 3,
    TEXTURE2DARRAY = 4,
    TEXTURE2DMS = 5,
    TEXTURE2DMSARRAY = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D11_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_VIEWPORT {
    pub TopLeftX: f32,
    pub TopLeftY: f32,
    pub Width: f32,
    pub Height: f32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_SUBRESOURCE_DATA {
    pub pSysMem: *const c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub StructureByteStride: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_BUFFER_RTV {
    pub FirstElement: u32,
    pub NumElements: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_TEX2D_RTV {
    pub MipSlice: u32,
}

#[repr(C)]
pub union D3D11_RENDER_TARGET_VIEW_DESC_UNION {
    pub Buffer: D3D11_BUFFER_RTV,
    pub Texture2D: D3D11_TEX2D_RTV,
}

#[repr(C)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_RTV_DIMENSION,
    pub u: D3D11_RENDER_TARGET_VIEW_DESC_UNION,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_TEX2D_DSV {
    pub MipSlice: u32,
}

#[repr(C)]
pub union D3D11_DEPTH_STENCIL_VIEW_DESC_UNION {
    pub Texture2D: D3D11_TEX2D_DSV,
}

#[repr(C)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_DSV_DIMENSION,
    pub Flags: u32,
    pub u: D3D11_DEPTH_STENCIL_VIEW_DESC_UNION,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: u32,
    pub u: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: u32,
    pub u: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub SemanticName: *const u8,
    pub SemanticIndex: u32,
    pub Format: DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D11_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_RASTERIZER_DESC {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: BOOL,
    pub ScissorEnable: BOOL,
    pub MultisampleEnable: BOOL,
    pub AntialiasedLineEnable: BOOL,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D11_STENCIL_OP,
    pub StencilDepthFailOp: D3D11_STENCIL_OP,
    pub StencilPassOp: D3D11_STENCIL_OP,
    pub StencilFunc: D3D11_COMPARISON_FUNC,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub DepthEnable: BOOL,
    pub DepthWriteMask: D3D11_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D11_COMPARISON_FUNC,
    pub StencilEnable: BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D11_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D11_DEPTH_STENCILOP_DESC,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: BOOL,
    pub SrcBlend: D3D11_BLEND,
    pub DestBlend: D3D11_BLEND,
    pub BlendOp: D3D11_BLEND_OP,
    pub SrcBlendAlpha: D3D11_BLEND,
    pub DestBlendAlpha: D3D11_BLEND,
    pub BlendOpAlpha: D3D11_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_BLEND_DESC {
    pub AlphaToCoverageEnable: BOOL,
    pub IndependentBlendEnable: BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_SAMPLER_DESC {
    pub Filter: D3D11_FILTER,
    pub AddressU: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D11_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D11_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_QUERY_DESC {
    pub Query: u32,
    pub MiscFlags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_COUNTER_DESC {
    pub Counter: u32,
    pub MiscFlags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    pub pData: *mut c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}

// Interfaces
#[repr(C)]
pub struct ID3D11DeviceChildVtbl {
    pub base: IUnknownVtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut c_void, ppDevice: *mut *mut ID3D11Device),
    pub GetPrivateData: unsafe extern "system" fn(
        this: *mut c_void,
        guid: *const GUID,
        pDataSize: *mut u32,
        pData: *mut c_void,
    ) -> HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(
        this: *mut c_void,
        guid: *const GUID,
        DataSize: u32,
        pData: *const c_void,
    ) -> HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        this: *mut c_void,
        guid: *const GUID,
        pData: *const IUnknown,
    ) -> HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11DeviceChild(pub *mut *const ID3D11DeviceChildVtbl);

impl ID3D11DeviceChild {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDevice(&self, ppDevice: &mut ID3D11Device) {
        unsafe { ((*(*self.0)).GetDevice)(self.0 as _, ppDevice as *mut _ as _) }
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: &GUID,
        pDataSize: &mut u32,
        pData: *mut c_void,
    ) -> HRESULT {
        unsafe {
            ((*(*self.0)).GetPrivateData)(self.0 as _, guid as *const _, pDataSize as *mut _, pData)
        }
    }
    pub unsafe fn SetPrivateData(&self, guid: &GUID, pData: &[u8]) -> HRESULT {
        unsafe {
            ((*(*self.0)).SetPrivateData)(
                self.0 as _,
                guid as *const _,
                pData.len() as u32,
                pData.as_ptr() as _,
            )
        }
    }
    pub unsafe fn SetPrivateDataInterface(&self, guid: &GUID, pData: Option<&IUnknown>) -> HRESULT {
        let unk = pData.map_or(core::ptr::null(), |u| u as *const _);
        unsafe { ((*(*self.0)).SetPrivateDataInterface)(self.0 as _, guid as *const _, unk) }
    }
}

#[repr(C)]
pub struct ID3D11ResourceVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetType: unsafe extern "system" fn(this: *mut c_void, pResourceDimension: *mut u32),
    pub SetEvictionPriority: unsafe extern "system" fn(this: *mut c_void, EvictionPriority: u32),
    pub GetEvictionPriority: unsafe extern "system" fn(this: *mut c_void) -> u32,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11Resource(pub *mut *const ID3D11ResourceVtbl);

impl ID3D11Resource {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetType(&self, pResourceDimension: &mut u32) {
        unsafe { ((*(*self.0)).GetType)(self.0 as _, pResourceDimension as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11BufferVtbl {
    pub base: ID3D11ResourceVtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_BUFFER_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11Buffer(pub *mut *const ID3D11BufferVtbl);

impl ID3D11Buffer {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_BUFFER_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11Texture1DVtbl {
    pub base: ID3D11ResourceVtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_TEXTURE1D_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11Texture1D(pub *mut *const ID3D11Texture1DVtbl);

impl ID3D11Texture1D {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_TEXTURE1D_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11Texture2DVtbl {
    pub base: ID3D11ResourceVtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_TEXTURE2D_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11Texture2D(pub *mut *const ID3D11Texture2DVtbl);

impl ID3D11Texture2D {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_TEXTURE2D_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11Texture3DVtbl {
    pub base: ID3D11ResourceVtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_TEXTURE3D_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11Texture3D(pub *mut *const ID3D11Texture3DVtbl);

impl ID3D11Texture3D {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_TEXTURE3D_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11ViewVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetResource:
        unsafe extern "system" fn(this: *mut c_void, ppResource: *mut *mut ID3D11Resource),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11View(pub *mut *const ID3D11ViewVtbl);

impl ID3D11View {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetResource(&self, ppResource: &mut ID3D11Resource) {
        unsafe { ((*(*self.0)).GetResource)(self.0 as _, ppResource as *mut _ as _) }
    }
}

#[repr(C)]
pub struct ID3D11ShaderResourceViewVtbl {
    pub base: ID3D11ViewVtbl,
    pub GetDesc:
        unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11ShaderResourceView(pub *mut *const ID3D11ShaderResourceViewVtbl);

impl ID3D11ShaderResourceView {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11RenderTargetViewVtbl {
    pub base: ID3D11ViewVtbl,
    pub GetDesc:
        unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_RENDER_TARGET_VIEW_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11RenderTargetView(pub *mut *const ID3D11RenderTargetViewVtbl);

impl ID3D11RenderTargetView {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_RENDER_TARGET_VIEW_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11DepthStencilViewVtbl {
    pub base: ID3D11ViewVtbl,
    pub GetDesc:
        unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11DepthStencilView(pub *mut *const ID3D11DepthStencilViewVtbl);

impl ID3D11DepthStencilView {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11UnorderedAccessViewVtbl {
    pub base: ID3D11ViewVtbl,
    pub GetDesc:
        unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11UnorderedAccessView(pub *mut *const ID3D11UnorderedAccessViewVtbl);

impl ID3D11UnorderedAccessView {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11VertexShaderVtbl {
    pub base: ID3D11DeviceChildVtbl,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11VertexShader(pub *mut *const ID3D11VertexShaderVtbl);

impl ID3D11VertexShader {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
}

#[repr(C)]
pub struct ID3D11PixelShaderVtbl {
    pub base: ID3D11DeviceChildVtbl,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11PixelShader(pub *mut *const ID3D11PixelShaderVtbl);

impl ID3D11PixelShader {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
}

#[repr(C)]
pub struct ID3D11ComputeShaderVtbl {
    pub base: ID3D11DeviceChildVtbl,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11ComputeShader(pub *mut *const ID3D11ComputeShaderVtbl);

impl ID3D11ComputeShader {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
}

#[repr(C)]
pub struct ID3D11InputLayoutVtbl {
    pub base: ID3D11DeviceChildVtbl,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11InputLayout(pub *mut *const ID3D11InputLayoutVtbl);

impl ID3D11InputLayout {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
}

#[repr(C)]
pub struct ID3D11BlendStateVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_BLEND_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11BlendState(pub *mut *const ID3D11BlendStateVtbl);

impl ID3D11BlendState {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_BLEND_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11DepthStencilStateVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_DEPTH_STENCIL_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11DepthStencilState(pub *mut *const ID3D11DepthStencilStateVtbl);

impl ID3D11DepthStencilState {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_DEPTH_STENCIL_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11RasterizerStateVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_RASTERIZER_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11RasterizerState(pub *mut *const ID3D11RasterizerStateVtbl);

impl ID3D11RasterizerState {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_RASTERIZER_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11SamplerStateVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub GetDesc: unsafe extern "system" fn(this: *mut c_void, pDesc: *mut D3D11_SAMPLER_DESC),
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11SamplerState(pub *mut *const ID3D11SamplerStateVtbl);

impl ID3D11SamplerState {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn GetDesc(&self, pDesc: &mut D3D11_SAMPLER_DESC) {
        unsafe { ((*(*self.0)).GetDesc)(self.0 as _, pDesc as *mut _) }
    }
}

#[repr(C)]
pub struct ID3D11DeviceContextVtbl {
    pub base: ID3D11DeviceChildVtbl,
    pub VSSetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *const *mut ID3D11Buffer,
    ),
    pub PSSetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *const *mut ID3D11ShaderResourceView,
    ),
    pub PSSetShader: unsafe extern "system" fn(
        this: *mut c_void,
        pPixelShader: *mut ID3D11PixelShader,
        ppClassInstances: *const *mut c_void,
        NumClassInstances: u32,
    ),
    pub PSSetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *const *mut ID3D11SamplerState,
    ),
    pub VSSetShader: unsafe extern "system" fn(
        this: *mut c_void,
        pVertexShader: *mut ID3D11VertexShader,
        ppClassInstances: *const *mut c_void,
        NumClassInstances: u32,
    ),
    pub DrawIndexed: unsafe extern "system" fn(
        this: *mut c_void,
        IndexCount: u32,
        StartIndexLocation: u32,
        BaseVertexLocation: i32,
    ),
    pub Draw:
        unsafe extern "system" fn(this: *mut c_void, VertexCount: u32, StartVertexLocation: u32),
    pub Map: unsafe extern "system" fn(
        this: *mut c_void,
        pResource: *mut ID3D11Resource,
        Subresource: u32,
        MapType: D3D11_MAP,
        MapFlags: u32,
        pMappedResource: *mut D3D11_MAPPED_SUBRESOURCE,
    ) -> HRESULT,
    pub Unmap: unsafe extern "system" fn(
        this: *mut c_void,
        pResource: *mut ID3D11Resource,
        Subresource: u32,
    ),
    pub PSSetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *const *mut ID3D11Buffer,
    ),
    pub IASetInputLayout:
        unsafe extern "system" fn(this: *mut c_void, pInputLayout: *mut ID3D11InputLayout),
    pub IASetVertexBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppVertexBuffers: *const *mut ID3D11Buffer,
        pStrides: *const u32,
        pOffsets: *const u32,
    ),
    pub IASetIndexBuffer: unsafe extern "system" fn(
        this: *mut c_void,
        pIndexBuffer: *mut ID3D11Buffer,
        Format: DXGI_FORMAT,
        Offset: u32,
    ),
    pub DrawIndexedInstanced: unsafe extern "system" fn(
        this: *mut c_void,
        IndexCountPerInstance: u32,
        InstanceCount: u32,
        StartIndexLocation: u32,
        BaseVertexLocation: i32,
        StartInstanceLocation: u32,
    ),
    pub DrawInstanced: unsafe extern "system" fn(
        this: *mut c_void,
        VertexCountPerInstance: u32,
        InstanceCount: u32,
        StartVertexLocation: u32,
        StartInstanceLocation: u32,
    ),
    pub GSSetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *const *mut ID3D11Buffer,
    ),
    pub GSSetShader: unsafe extern "system" fn(
        this: *mut c_void,
        pShader: *mut c_void,
        ppClassInstances: *const *mut c_void,
        NumClassInstances: u32,
    ),
    pub IASetPrimitiveTopology:
        unsafe extern "system" fn(this: *mut c_void, Topology: D3D_PRIMITIVE_TOPOLOGY),
    pub VSSetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *const *mut ID3D11ShaderResourceView,
    ),
    pub VSSetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *const *mut ID3D11SamplerState,
    ),
    pub Begin: unsafe extern "system" fn(this: *mut c_void, pAsync: *mut c_void),
    pub End: unsafe extern "system" fn(this: *mut c_void, pAsync: *mut c_void),
    pub GetData: unsafe extern "system" fn(
        this: *mut c_void,
        pAsync: *mut c_void,
        pData: *mut c_void,
        DataSize: u32,
        GetDataFlags: u32,
    ) -> HRESULT,
    pub SetPredication:
        unsafe extern "system" fn(this: *mut c_void, pPredicate: *mut c_void, PredicateValue: BOOL),
    pub GSSetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *const *mut ID3D11ShaderResourceView,
    ),
    pub GSSetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *const *mut ID3D11SamplerState,
    ),
    pub OMSetRenderTargets: unsafe extern "system" fn(
        this: *mut c_void,
        NumViews: u32,
        ppRenderTargetViews: *const *mut ID3D11RenderTargetView,
        pDepthStencilView: *mut ID3D11DepthStencilView,
    ),
    pub OMSetRenderTargetsAndUnorderedAccessViews: unsafe extern "system" fn(
        this: *mut c_void,
        NumRTVs: u32,
        ppRenderTargetViews: *const *mut ID3D11RenderTargetView,
        pDepthStencilView: *mut ID3D11DepthStencilView,
        UAVStartSlot: u32,
        NumUAVs: u32,
        ppUnorderedAccessViews: *const *mut ID3D11UnorderedAccessView,
        pUAVInitialCounts: *const u32,
    ),
    pub OMSetBlendState: unsafe extern "system" fn(
        this: *mut c_void,
        pBlendState: *mut ID3D11BlendState,
        BlendFactor: *const [f32; 4],
        SampleMask: u32,
    ),
    pub OMSetDepthStencilState: unsafe extern "system" fn(
        this: *mut c_void,
        pDepthStencilState: *mut ID3D11DepthStencilState,
        StencilRef: u32,
    ),
    pub SOSetTargets: unsafe extern "system" fn(
        this: *mut c_void,
        NumBuffers: u32,
        ppSOTargets: *const *mut ID3D11Buffer,
        pOffsets: *const u32,
    ),
    pub DrawAuto: unsafe extern "system" fn(this: *mut c_void),
    pub DrawIndexedInstancedIndirect: unsafe extern "system" fn(
        this: *mut c_void,
        pBufferForArgs: *mut ID3D11Buffer,
        AlignedByteOffsetForArgs: u32,
    ),
    pub DrawInstancedIndirect: unsafe extern "system" fn(
        this: *mut c_void,
        pBufferForArgs: *mut ID3D11Buffer,
        AlignedByteOffsetForArgs: u32,
    ),
    pub Dispatch: unsafe extern "system" fn(
        this: *mut c_void,
        ThreadGroupCountX: u32,
        ThreadGroupCountY: u32,
        ThreadGroupCountZ: u32,
    ),
    pub DispatchIndirect: unsafe extern "system" fn(
        this: *mut c_void,
        pBufferForArgs: *mut ID3D11Buffer,
        AlignedByteOffsetForArgs: u32,
    ),
    pub RSSetState:
        unsafe extern "system" fn(this: *mut c_void, pRasterizerState: *mut ID3D11RasterizerState),
    pub RSSetViewports: unsafe extern "system" fn(
        this: *mut c_void,
        NumViewports: u32,
        pViewports: *const D3D11_VIEWPORT,
    ),
    pub RSSetScissorRects:
        unsafe extern "system" fn(this: *mut c_void, NumRects: u32, pRects: *const RECT),
    pub CopySubresourceRegion: unsafe extern "system" fn(
        this: *mut c_void,
        pDstResource: *mut ID3D11Resource,
        DstSubresource: u32,
        DstX: u32,
        DstY: u32,
        DstZ: u32,
        pSrcResource: *mut ID3D11Resource,
        SrcSubresource: u32,
        pSrcBox: *const D3D11_BOX,
    ),
    pub CopyResource: unsafe extern "system" fn(
        this: *mut c_void,
        pDstResource: *mut ID3D11Resource,
        pSrcResource: *mut ID3D11Resource,
    ),
    pub UpdateSubresource: unsafe extern "system" fn(
        this: *mut c_void,
        pDstResource: *mut ID3D11Resource,
        DstSubresource: u32,
        pDstBox: *const D3D11_BOX,
        pSrcData: *const c_void,
        SrcRowPitch: u32,
        SrcDepthPitch: u32,
    ),
    pub CopyStructureCount: unsafe extern "system" fn(
        this: *mut c_void,
        pDstBuffer: *mut ID3D11Buffer,
        DstAlignedByteOffset: u32,
        pSrcView: *mut ID3D11UnorderedAccessView,
    ),
    pub ClearRenderTargetView: unsafe extern "system" fn(
        this: *mut c_void,
        pRenderTargetView: *mut ID3D11RenderTargetView,
        ColorRGBA: *const [f32; 4],
    ),
    pub ClearUnorderedAccessViewUint: unsafe extern "system" fn(
        this: *mut c_void,
        pUnorderedAccessView: *mut ID3D11UnorderedAccessView,
        Values: *const [u32; 4],
    ),
    pub ClearUnorderedAccessViewFloat: unsafe extern "system" fn(
        this: *mut c_void,
        pUnorderedAccessView: *mut ID3D11UnorderedAccessView,
        Values: *const [f32; 4],
    ),
    pub ClearDepthStencilView: unsafe extern "system" fn(
        this: *mut c_void,
        pDepthStencilView: *mut ID3D11DepthStencilView,
        ClearFlags: u32,
        Depth: f32,
        Stencil: u8,
    ),
    pub GenerateMips: unsafe extern "system" fn(
        this: *mut c_void,
        pShaderResourceView: *mut ID3D11ShaderResourceView,
    ),
    pub SetResourceMinLOD:
        unsafe extern "system" fn(this: *mut c_void, pResource: *mut ID3D11Resource, MinLOD: f32),
    pub GetResourceMinLOD:
        unsafe extern "system" fn(this: *mut c_void, pResource: *mut ID3D11Resource) -> f32,
    pub ResolveSubresource: unsafe extern "system" fn(
        this: *mut c_void,
        pDstResource: *mut ID3D11Resource,
        DstSubresource: u32,
        pSrcResource: *mut ID3D11Resource,
        SrcSubresource: u32,
        Format: DXGI_FORMAT,
    ),
    pub ExecuteCommandList: unsafe extern "system" fn(
        this: *mut c_void,
        pCommandList: *mut c_void,
        RestoreContextState: BOOL,
    ),
    pub HSSetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *const *mut ID3D11ShaderResourceView,
    ),
    pub HSSetShader: unsafe extern "system" fn(
        this: *mut c_void,
        pHullShader: *mut c_void,
        ppClassInstances: *const *mut c_void,
        NumClassInstances: u32,
    ),
    pub HSSetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *const *mut ID3D11SamplerState,
    ),
    pub HSSetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *const *mut ID3D11Buffer,
    ),
    pub DSSetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *const *mut ID3D11ShaderResourceView,
    ),
    pub DSSetShader: unsafe extern "system" fn(
        this: *mut c_void,
        pDomainShader: *mut c_void,
        ppClassInstances: *const *mut c_void,
        NumClassInstances: u32,
    ),
    pub DSSetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *const *mut ID3D11SamplerState,
    ),
    pub DSSetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *const *mut ID3D11Buffer,
    ),
    pub CSSetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *const *mut ID3D11ShaderResourceView,
    ),
    pub CSSetUnorderedAccessViews: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumUAVs: u32,
        ppUnorderedAccessViews: *const *mut ID3D11UnorderedAccessView,
        pUAVInitialCounts: *const u32,
    ),
    pub CSSetShader: unsafe extern "system" fn(
        this: *mut c_void,
        pComputeShader: *mut ID3D11ComputeShader,
        ppClassInstances: *const *mut c_void,
        NumClassInstances: u32,
    ),
    pub CSSetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *const *mut ID3D11SamplerState,
    ),
    pub CSSetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *const *mut ID3D11Buffer,
    ),
    pub VSGetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
    ),
    pub PSGetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView,
    ),
    pub PSGetShader: unsafe extern "system" fn(
        this: *mut c_void,
        ppPixelShader: *mut *mut ID3D11PixelShader,
        ppClassInstances: *mut *mut c_void,
        pNumClassInstances: *mut u32,
    ),
    pub PSGetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *mut *mut ID3D11SamplerState,
    ),
    pub VSGetShader: unsafe extern "system" fn(
        this: *mut c_void,
        ppVertexShader: *mut *mut ID3D11VertexShader,
        ppClassInstances: *mut *mut c_void,
        pNumClassInstances: *mut u32,
    ),
    pub PSGetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
    ),
    pub IAGetInputLayout:
        unsafe extern "system" fn(this: *mut c_void, ppInputLayout: *mut *mut ID3D11InputLayout),
    pub IAGetVertexBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppVertexBuffers: *mut *mut ID3D11Buffer,
        pStrides: *mut u32,
        pOffsets: *mut u32,
    ),
    pub IAGetIndexBuffer: unsafe extern "system" fn(
        this: *mut c_void,
        pIndexBuffer: *mut *mut ID3D11Buffer,
        Format: *mut DXGI_FORMAT,
        Offset: *mut u32,
    ),
    pub GSGetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
    ),
    pub GSGetShader: unsafe extern "system" fn(
        this: *mut c_void,
        ppGeometryShader: *mut *mut c_void,
        ppClassInstances: *mut *mut c_void,
        pNumClassInstances: *mut u32,
    ),
    pub IAGetPrimitiveTopology:
        unsafe extern "system" fn(this: *mut c_void, pTopology: *mut D3D_PRIMITIVE_TOPOLOGY),
    pub VSGetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView,
    ),
    pub VSGetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *mut *mut ID3D11SamplerState,
    ),
    pub GetPredication: unsafe extern "system" fn(
        this: *mut c_void,
        ppPredicate: *mut *mut c_void,
        pPredicateValue: *mut BOOL,
    ),
    pub GSGetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView,
    ),
    pub GSGetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *mut *mut ID3D11SamplerState,
    ),
    pub OMGetRenderTargets: unsafe extern "system" fn(
        this: *mut c_void,
        NumViews: u32,
        ppRenderTargetViews: *mut *mut ID3D11RenderTargetView,
        ppDepthStencilView: *mut *mut ID3D11DepthStencilView,
    ),
    pub OMGetRenderTargetsAndUnorderedAccessViews: unsafe extern "system" fn(
        this: *mut c_void,
        NumRTVs: u32,
        ppRenderTargetViews: *mut *mut ID3D11RenderTargetView,
        ppDepthStencilView: *mut *mut ID3D11DepthStencilView,
        UAVStartSlot: u32,
        NumUAVs: u32,
        ppUnorderedAccessViews: *mut *mut ID3D11UnorderedAccessView,
    ),
    pub OMGetBlendState: unsafe extern "system" fn(
        this: *mut c_void,
        ppBlendState: *mut *mut ID3D11BlendState,
        BlendFactor: *mut [f32; 4],
        pSampleMask: *mut u32,
    ),
    pub OMGetDepthStencilState: unsafe extern "system" fn(
        this: *mut c_void,
        ppDepthStencilState: *mut *mut ID3D11DepthStencilState,
        pStencilRef: *mut u32,
    ),
    pub SOGetTargets: unsafe extern "system" fn(
        this: *mut c_void,
        NumBuffers: u32,
        ppSOTargets: *mut *mut ID3D11Buffer,
    ),
    pub RSGetState: unsafe extern "system" fn(
        this: *mut c_void,
        ppRasterizerState: *mut *mut ID3D11RasterizerState,
    ),
    pub RSGetViewports: unsafe extern "system" fn(
        this: *mut c_void,
        pNumViewports: *mut u32,
        pViewports: *mut D3D11_VIEWPORT,
    ),
    pub RSGetScissorRects:
        unsafe extern "system" fn(this: *mut c_void, pNumRects: *mut u32, pRects: *mut RECT),
    pub HSGetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView,
    ),
    pub HSGetShader: unsafe extern "system" fn(
        this: *mut c_void,
        ppHullShader: *mut *mut c_void,
        ppClassInstances: *mut *mut c_void,
        pNumClassInstances: *mut u32,
    ),
    pub HSGetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *mut *mut ID3D11SamplerState,
    ),
    pub HSGetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
    ),
    pub DSGetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView,
    ),
    pub DSGetShader: unsafe extern "system" fn(
        this: *mut c_void,
        ppDomainShader: *mut *mut c_void,
        ppClassInstances: *mut *mut c_void,
        pNumClassInstances: *mut u32,
    ),
    pub DSGetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *mut *mut ID3D11SamplerState,
    ),
    pub DSGetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
    ),
    pub CSGetShaderResources: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumViews: u32,
        ppShaderResourceViews: *mut *mut ID3D11ShaderResourceView,
    ),
    pub CSGetUnorderedAccessViews: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumUAVs: u32,
        ppUnorderedAccessViews: *mut *mut ID3D11UnorderedAccessView,
    ),
    pub CSGetShader: unsafe extern "system" fn(
        this: *mut c_void,
        ppComputeShader: *mut *mut ID3D11ComputeShader,
        ppClassInstances: *mut *mut c_void,
        pNumClassInstances: *mut u32,
    ),
    pub CSGetSamplers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumSamplers: u32,
        ppSamplers: *mut *mut ID3D11SamplerState,
    ),
    pub CSGetConstantBuffers: unsafe extern "system" fn(
        this: *mut c_void,
        StartSlot: u32,
        NumBuffers: u32,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
    ),
    pub ClearState: unsafe extern "system" fn(this: *mut c_void),
    pub Flush: unsafe extern "system" fn(this: *mut c_void),
    pub GetContextFlags: unsafe extern "system" fn(this: *mut c_void) -> u32,
    pub FinishCommandList: unsafe extern "system" fn(
        this: *mut c_void,
        RestoreDeferredContextState: BOOL,
        ppCommandList: *mut *mut c_void,
    ) -> HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11DeviceContext(pub *mut *const ID3D11DeviceContextVtbl);

impl ID3D11DeviceContext {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn VSSetConstantBuffers(&self, StartSlot: u32, ppConstantBuffers: &[ID3D11Buffer]) {
        unsafe {
            ((*(*self.0)).VSSetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn PSSetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &[ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).PSSetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_ptr() as _,
            )
        }
    }
    pub unsafe fn PSSetShader(
        &self,
        pPixelShader: Option<&ID3D11PixelShader>,
        ppClassInstances: Option<&[*mut c_void]>,
    ) {
        let shader = pPixelShader.map_or(core::ptr::null_mut(), |s| s.0 as _);
        let (inst_ptr, inst_count) = match ppClassInstances {
            Some(s) => (s.as_ptr(), s.len() as u32),
            None => (core::ptr::null(), 0),
        };
        unsafe { ((*(*self.0)).PSSetShader)(self.0 as _, shader, inst_ptr, inst_count) }
    }
    pub unsafe fn PSSetSamplers(&self, StartSlot: u32, ppSamplers: &[ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).PSSetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn VSSetShader(
        &self,
        pVertexShader: Option<&ID3D11VertexShader>,
        ppClassInstances: Option<&[*mut c_void]>,
    ) {
        let shader = pVertexShader.map_or(core::ptr::null_mut(), |s| s.0 as _);
        let (inst_ptr, inst_count) = match ppClassInstances {
            Some(s) => (s.as_ptr(), s.len() as u32),
            None => (core::ptr::null(), 0),
        };
        unsafe { ((*(*self.0)).VSSetShader)(self.0 as _, shader, inst_ptr, inst_count) }
    }
    pub unsafe fn DrawIndexed(
        &self,
        IndexCount: u32,
        StartIndexLocation: u32,
        BaseVertexLocation: i32,
    ) {
        unsafe {
            ((*(*self.0)).DrawIndexed)(
                self.0 as _,
                IndexCount,
                StartIndexLocation,
                BaseVertexLocation,
            )
        }
    }
    pub unsafe fn Draw(&self, VertexCount: u32, StartVertexLocation: u32) {
        unsafe { ((*(*self.0)).Draw)(self.0 as _, VertexCount, StartVertexLocation) }
    }
    pub unsafe fn Map(
        &self,
        pResource: &ID3D11Resource,
        Subresource: u32,
        MapType: D3D11_MAP,
        MapFlags: u32,
        pMappedResource: &mut D3D11_MAPPED_SUBRESOURCE,
    ) -> HRESULT {
        unsafe {
            ((*(*self.0)).Map)(
                self.0 as _,
                pResource.0 as _,
                Subresource,
                MapType,
                MapFlags,
                pMappedResource as *mut _,
            )
        }
    }
    pub unsafe fn Unmap(&self, pResource: &ID3D11Resource, Subresource: u32) {
        unsafe { ((*(*self.0)).Unmap)(self.0 as _, pResource.0 as _, Subresource) }
    }
    pub unsafe fn PSSetConstantBuffers(&self, StartSlot: u32, ppConstantBuffers: &[ID3D11Buffer]) {
        unsafe {
            ((*(*self.0)).PSSetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn IASetInputLayout(&self, pInputLayout: Option<&ID3D11InputLayout>) {
        let layout = pInputLayout.map_or(core::ptr::null_mut(), |l| l.0 as _);
        unsafe { ((*(*self.0)).IASetInputLayout)(self.0 as _, layout) }
    }
    pub unsafe fn IASetVertexBuffers(
        &self,
        StartSlot: u32,
        ppVertexBuffers: &[ID3D11Buffer],
        pStrides: &[u32],
        pOffsets: &[u32],
    ) {
        unsafe {
            ((*(*self.0)).IASetVertexBuffers)(
                self.0 as _,
                StartSlot,
                ppVertexBuffers.len() as u32,
                ppVertexBuffers.as_ptr() as _,
                pStrides.as_ptr(),
                pOffsets.as_ptr(),
            )
        }
    }
    pub unsafe fn IASetIndexBuffer(
        &self,
        pIndexBuffer: Option<&ID3D11Buffer>,
        Format: DXGI_FORMAT,
        Offset: u32,
    ) {
        let buf = pIndexBuffer.map_or(core::ptr::null_mut(), |b| b.0 as _);
        unsafe { ((*(*self.0)).IASetIndexBuffer)(self.0 as _, buf, Format, Offset) }
    }
    pub unsafe fn DrawIndexedInstanced(
        &self,
        IndexCountPerInstance: u32,
        InstanceCount: u32,
        StartIndexLocation: u32,
        BaseVertexLocation: i32,
        StartInstanceLocation: u32,
    ) {
        unsafe {
            ((*(*self.0)).DrawIndexedInstanced)(
                self.0 as _,
                IndexCountPerInstance,
                InstanceCount,
                StartIndexLocation,
                BaseVertexLocation,
                StartInstanceLocation,
            )
        }
    }
    pub unsafe fn DrawInstanced(
        &self,
        VertexCountPerInstance: u32,
        InstanceCount: u32,
        StartVertexLocation: u32,
        StartInstanceLocation: u32,
    ) {
        unsafe {
            ((*(*self.0)).DrawInstanced)(
                self.0 as _,
                VertexCountPerInstance,
                InstanceCount,
                StartVertexLocation,
                StartInstanceLocation,
            )
        }
    }
    pub unsafe fn GSSetConstantBuffers(&self, StartSlot: u32, ppConstantBuffers: &[ID3D11Buffer]) {
        unsafe {
            ((*(*self.0)).GSSetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn GSSetShader(
        &self,
        pShader: Option<*mut c_void>,
        ppClassInstances: Option<&[*mut c_void]>,
    ) {
        let sh = pShader.unwrap_or(core::ptr::null_mut());
        let (inst_ptr, inst_count) = match ppClassInstances {
            Some(s) => (s.as_ptr(), s.len() as u32),
            None => (core::ptr::null(), 0),
        };
        unsafe { ((*(*self.0)).GSSetShader)(self.0 as _, sh, inst_ptr, inst_count) }
    }
    pub unsafe fn IASetPrimitiveTopology(&self, Topology: D3D_PRIMITIVE_TOPOLOGY) {
        unsafe { ((*(*self.0)).IASetPrimitiveTopology)(self.0 as _, Topology) }
    }
    pub unsafe fn VSSetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &[ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).VSSetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_ptr() as _,
            )
        }
    }
    pub unsafe fn VSSetSamplers(&self, StartSlot: u32, ppSamplers: &[ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).VSSetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn Begin(&self, pAsync: *mut c_void) {
        unsafe { ((*(*self.0)).Begin)(self.0 as _, pAsync) }
    }
    pub unsafe fn End(&self, pAsync: *mut c_void) {
        unsafe { ((*(*self.0)).End)(self.0 as _, pAsync) }
    }
    pub unsafe fn GetData(
        &self,
        pAsync: *mut c_void,
        pData: *mut c_void,
        DataSize: u32,
        GetDataFlags: u32,
    ) -> HRESULT {
        unsafe { ((*(*self.0)).GetData)(self.0 as _, pAsync, pData, DataSize, GetDataFlags) }
    }
    pub unsafe fn SetPredication(&self, pPredicate: Option<*mut c_void>, PredicateValue: BOOL) {
        let pred = pPredicate.unwrap_or(core::ptr::null_mut());
        unsafe { ((*(*self.0)).SetPredication)(self.0 as _, pred, PredicateValue) }
    }
    pub unsafe fn GSSetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &[ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).GSSetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_ptr() as _,
            )
        }
    }
    pub unsafe fn GSSetSamplers(&self, StartSlot: u32, ppSamplers: &[ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).GSSetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn OMSetRenderTargets(
        &self,
        ppRenderTargetViews: &[ID3D11RenderTargetView],
        pDepthStencilView: Option<&ID3D11DepthStencilView>,
    ) {
        let dsv = pDepthStencilView.map_or(core::ptr::null_mut(), |d| d.0 as _);
        unsafe {
            ((*(*self.0)).OMSetRenderTargets)(
                self.0 as _,
                ppRenderTargetViews.len() as u32,
                ppRenderTargetViews.as_ptr() as _,
                dsv,
            )
        }
    }
    pub unsafe fn OMSetRenderTargetsAndUnorderedAccessViews(
        &self,
        ppRenderTargetViews: &[ID3D11RenderTargetView],
        pDepthStencilView: Option<&ID3D11DepthStencilView>,
        UAVStartSlot: u32,
        ppUnorderedAccessViews: &[ID3D11UnorderedAccessView],
        pUAVInitialCounts: Option<&[u32]>,
    ) {
        let dsv = pDepthStencilView.map_or(core::ptr::null_mut(), |d| d.0 as _);
        let init_counts = pUAVInitialCounts.map_or(core::ptr::null(), |c| c.as_ptr());
        unsafe {
            ((*(*self.0)).OMSetRenderTargetsAndUnorderedAccessViews)(
                self.0 as _,
                ppRenderTargetViews.len() as u32,
                ppRenderTargetViews.as_ptr() as _,
                dsv,
                UAVStartSlot,
                ppUnorderedAccessViews.len() as u32,
                ppUnorderedAccessViews.as_ptr() as _,
                init_counts,
            )
        }
    }
    pub unsafe fn OMSetBlendState(
        &self,
        pBlendState: Option<&ID3D11BlendState>,
        BlendFactor: Option<&[f32; 4]>,
        SampleMask: u32,
    ) {
        let blend = pBlendState.map_or(core::ptr::null_mut(), |b| b.0 as _);
        let factor = BlendFactor.map_or(core::ptr::null(), |f| f as *const _);
        unsafe { ((*(*self.0)).OMSetBlendState)(self.0 as _, blend, factor, SampleMask) }
    }
    pub unsafe fn OMSetDepthStencilState(
        &self,
        pDepthStencilState: Option<&ID3D11DepthStencilState>,
        StencilRef: u32,
    ) {
        let dss = pDepthStencilState.map_or(core::ptr::null_mut(), |d| d.0 as _);
        unsafe { ((*(*self.0)).OMSetDepthStencilState)(self.0 as _, dss, StencilRef) }
    }
    pub unsafe fn SOSetTargets(&self, ppSOTargets: &[ID3D11Buffer], pOffsets: &[u32]) {
        unsafe {
            ((*(*self.0)).SOSetTargets)(
                self.0 as _,
                ppSOTargets.len() as u32,
                ppSOTargets.as_ptr() as _,
                pOffsets.as_ptr(),
            )
        }
    }
    pub unsafe fn DrawAuto(&self) {
        unsafe { ((*(*self.0)).DrawAuto)(self.0 as _) }
    }
    pub unsafe fn DrawIndexedInstancedIndirect(
        &self,
        pBufferForArgs: &ID3D11Buffer,
        AlignedByteOffsetForArgs: u32,
    ) {
        unsafe {
            ((*(*self.0)).DrawIndexedInstancedIndirect)(
                self.0 as _,
                pBufferForArgs.0 as _,
                AlignedByteOffsetForArgs,
            )
        }
    }
    pub unsafe fn DrawInstancedIndirect(
        &self,
        pBufferForArgs: &ID3D11Buffer,
        AlignedByteOffsetForArgs: u32,
    ) {
        unsafe {
            ((*(*self.0)).DrawInstancedIndirect)(
                self.0 as _,
                pBufferForArgs.0 as _,
                AlignedByteOffsetForArgs,
            )
        }
    }
    pub unsafe fn Dispatch(
        &self,
        ThreadGroupCountX: u32,
        ThreadGroupCountY: u32,
        ThreadGroupCountZ: u32,
    ) {
        unsafe {
            ((*(*self.0)).Dispatch)(
                self.0 as _,
                ThreadGroupCountX,
                ThreadGroupCountY,
                ThreadGroupCountZ,
            )
        }
    }
    pub unsafe fn DispatchIndirect(
        &self,
        pBufferForArgs: &ID3D11Buffer,
        AlignedByteOffsetForArgs: u32,
    ) {
        unsafe {
            ((*(*self.0)).DispatchIndirect)(
                self.0 as _,
                pBufferForArgs.0 as _,
                AlignedByteOffsetForArgs,
            )
        }
    }
    pub unsafe fn RSSetState(&self, pRasterizerState: Option<&ID3D11RasterizerState>) {
        let rs = pRasterizerState.map_or(core::ptr::null_mut(), |r| r.0 as _);
        unsafe { ((*(*self.0)).RSSetState)(self.0 as _, rs) }
    }
    pub unsafe fn RSSetViewports(&self, pViewports: &[D3D11_VIEWPORT]) {
        unsafe {
            ((*(*self.0)).RSSetViewports)(self.0 as _, pViewports.len() as u32, pViewports.as_ptr())
        }
    }
    pub unsafe fn RSSetScissorRects(&self, pRects: &[RECT]) {
        unsafe {
            ((*(*self.0)).RSSetScissorRects)(self.0 as _, pRects.len() as u32, pRects.as_ptr())
        }
    }
    pub unsafe fn CopySubresourceRegion(
        &self,
        pDstResource: &ID3D11Resource,
        DstSubresource: u32,
        DstX: u32,
        DstY: u32,
        DstZ: u32,
        pSrcResource: &ID3D11Resource,
        SrcSubresource: u32,
        pSrcBox: Option<&D3D11_BOX>,
    ) {
        let sb = pSrcBox.map_or(core::ptr::null(), |b| b as *const _);
        unsafe {
            ((*(*self.0)).CopySubresourceRegion)(
                self.0 as _,
                pDstResource.0 as _,
                DstSubresource,
                DstX,
                DstY,
                DstZ,
                pSrcResource.0 as _,
                SrcSubresource,
                sb,
            )
        }
    }
    pub unsafe fn CopyResource(
        &self,
        pDstResource: &ID3D11Resource,
        pSrcResource: &ID3D11Resource,
    ) {
        unsafe {
            ((*(*self.0)).CopyResource)(self.0 as _, pDstResource.0 as _, pSrcResource.0 as _)
        }
    }
    pub unsafe fn UpdateSubresource(
        &self,
        pDstResource: &ID3D11Resource,
        DstSubresource: u32,
        pDstBox: Option<&D3D11_BOX>,
        pSrcData: *const c_void,
        SrcRowPitch: u32,
        SrcDepthPitch: u32,
    ) {
        let db = pDstBox.map_or(core::ptr::null(), |b| b as *const _);
        unsafe {
            ((*(*self.0)).UpdateSubresource)(
                self.0 as _,
                pDstResource.0 as _,
                DstSubresource,
                db,
                pSrcData,
                SrcRowPitch,
                SrcDepthPitch,
            )
        }
    }
    pub unsafe fn CopyStructureCount(
        &self,
        pDstBuffer: &ID3D11Buffer,
        DstAlignedByteOffset: u32,
        pSrcView: &ID3D11UnorderedAccessView,
    ) {
        unsafe {
            ((*(*self.0)).CopyStructureCount)(
                self.0 as _,
                pDstBuffer.0 as _,
                DstAlignedByteOffset,
                pSrcView.0 as _,
            )
        }
    }
    pub unsafe fn ClearRenderTargetView(
        &self,
        pRenderTargetView: &ID3D11RenderTargetView,
        ColorRGBA: &[f32; 4],
    ) {
        unsafe {
            ((*(*self.0)).ClearRenderTargetView)(
                self.0 as _,
                pRenderTargetView.0 as _,
                ColorRGBA as *const _,
            )
        }
    }
    pub unsafe fn ClearUnorderedAccessViewUint(
        &self,
        pUnorderedAccessView: &ID3D11UnorderedAccessView,
        Values: &[u32; 4],
    ) {
        unsafe {
            ((*(*self.0)).ClearUnorderedAccessViewUint)(
                self.0 as _,
                pUnorderedAccessView.0 as _,
                Values as *const _,
            )
        }
    }
    pub unsafe fn ClearUnorderedAccessViewFloat(
        &self,
        pUnorderedAccessView: &ID3D11UnorderedAccessView,
        Values: &[f32; 4],
    ) {
        unsafe {
            ((*(*self.0)).ClearUnorderedAccessViewFloat)(
                self.0 as _,
                pUnorderedAccessView.0 as _,
                Values as *const _,
            )
        }
    }
    pub unsafe fn ClearDepthStencilView(
        &self,
        pDepthStencilView: &ID3D11DepthStencilView,
        ClearFlags: u32,
        Depth: f32,
        Stencil: u8,
    ) {
        unsafe {
            ((*(*self.0)).ClearDepthStencilView)(
                self.0 as _,
                pDepthStencilView.0 as _,
                ClearFlags,
                Depth,
                Stencil,
            )
        }
    }
    pub unsafe fn GenerateMips(&self, pShaderResourceView: &ID3D11ShaderResourceView) {
        unsafe { ((*(*self.0)).GenerateMips)(self.0 as _, pShaderResourceView.0 as _) }
    }
    pub unsafe fn SetResourceMinLOD(&self, pResource: &ID3D11Resource, MinLOD: f32) {
        unsafe { ((*(*self.0)).SetResourceMinLOD)(self.0 as _, pResource.0 as _, MinLOD) }
    }
    pub unsafe fn GetResourceMinLOD(&self, pResource: &ID3D11Resource) -> f32 {
        unsafe { ((*(*self.0)).GetResourceMinLOD)(self.0 as _, pResource.0 as _) }
    }
    pub unsafe fn ResolveSubresource(
        &self,
        pDstResource: &ID3D11Resource,
        DstSubresource: u32,
        pSrcResource: &ID3D11Resource,
        SrcSubresource: u32,
        Format: DXGI_FORMAT,
    ) {
        unsafe {
            ((*(*self.0)).ResolveSubresource)(
                self.0 as _,
                pDstResource.0 as _,
                DstSubresource,
                pSrcResource.0 as _,
                SrcSubresource,
                Format,
            )
        }
    }
    pub unsafe fn ExecuteCommandList(&self, pCommandList: *mut c_void, RestoreContextState: BOOL) {
        unsafe { ((*(*self.0)).ExecuteCommandList)(self.0 as _, pCommandList, RestoreContextState) }
    }
    pub unsafe fn HSSetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &[ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).HSSetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_ptr() as _,
            )
        }
    }
    pub unsafe fn HSSetShader(
        &self,
        pHullShader: Option<*mut c_void>,
        ppClassInstances: Option<&[*mut c_void]>,
    ) {
        let sh = pHullShader.unwrap_or(core::ptr::null_mut());
        let (inst_ptr, inst_count) = match ppClassInstances {
            Some(s) => (s.as_ptr(), s.len() as u32),
            None => (core::ptr::null(), 0),
        };
        unsafe { ((*(*self.0)).HSSetShader)(self.0 as _, sh, inst_ptr, inst_count) }
    }
    pub unsafe fn HSSetSamplers(&self, StartSlot: u32, ppSamplers: &[ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).HSSetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn HSSetConstantBuffers(&self, StartSlot: u32, ppConstantBuffers: &[ID3D11Buffer]) {
        unsafe {
            ((*(*self.0)).HSSetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn DSSetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &[ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).DSSetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_ptr() as _,
            )
        }
    }
    pub unsafe fn DSSetShader(
        &self,
        pDomainShader: Option<*mut c_void>,
        ppClassInstances: Option<&[*mut c_void]>,
    ) {
        let sh = pDomainShader.unwrap_or(core::ptr::null_mut());
        let (inst_ptr, inst_count) = match ppClassInstances {
            Some(s) => (s.as_ptr(), s.len() as u32),
            None => (core::ptr::null(), 0),
        };
        unsafe { ((*(*self.0)).DSSetShader)(self.0 as _, sh, inst_ptr, inst_count) }
    }
    pub unsafe fn DSSetSamplers(&self, StartSlot: u32, ppSamplers: &[ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).DSSetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn DSSetConstantBuffers(&self, StartSlot: u32, ppConstantBuffers: &[ID3D11Buffer]) {
        unsafe {
            ((*(*self.0)).DSSetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn CSSetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &[ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).CSSetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_ptr() as _,
            )
        }
    }
    pub unsafe fn CSSetUnorderedAccessViews(
        &self,
        StartSlot: u32,
        ppUnorderedAccessViews: &[ID3D11UnorderedAccessView],
        pUAVInitialCounts: Option<&[u32]>,
    ) {
        let init_counts = pUAVInitialCounts.map_or(core::ptr::null(), |c| c.as_ptr());
        unsafe {
            ((*(*self.0)).CSSetUnorderedAccessViews)(
                self.0 as _,
                StartSlot,
                ppUnorderedAccessViews.len() as u32,
                ppUnorderedAccessViews.as_ptr() as _,
                init_counts,
            )
        }
    }
    pub unsafe fn CSSetShader(
        &self,
        pComputeShader: Option<&ID3D11ComputeShader>,
        ppClassInstances: Option<&[*mut c_void]>,
    ) {
        let sh = pComputeShader.map_or(core::ptr::null_mut(), |s| s.0 as _);
        let (inst_ptr, inst_count) = match ppClassInstances {
            Some(s) => (s.as_ptr(), s.len() as u32),
            None => (core::ptr::null(), 0),
        };
        unsafe { ((*(*self.0)).CSSetShader)(self.0 as _, sh, inst_ptr, inst_count) }
    }
    pub unsafe fn CSSetSamplers(&self, StartSlot: u32, ppSamplers: &[ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).CSSetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn CSSetConstantBuffers(&self, StartSlot: u32, ppConstantBuffers: &[ID3D11Buffer]) {
        unsafe {
            ((*(*self.0)).CSSetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn VSGetConstantBuffers(
        &self,
        StartSlot: u32,
        ppConstantBuffers: &mut [ID3D11Buffer],
    ) {
        unsafe {
            ((*(*self.0)).VSGetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn PSGetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &mut [ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).PSGetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn PSGetShader(
        &self,
        ppPixelShader: &mut ID3D11PixelShader,
        ppClassInstances: Option<&mut [*mut c_void]>,
        pNumClassInstances: Option<&mut u32>,
    ) {
        let (inst_ptr, inst_count) = match (ppClassInstances, pNumClassInstances) {
            (Some(s), Some(c)) => (s.as_mut_ptr(), c as *mut _),
            _ => (core::ptr::null_mut(), core::ptr::null_mut()),
        };
        unsafe {
            ((*(*self.0)).PSGetShader)(
                self.0 as _,
                ppPixelShader as *mut _ as _,
                inst_ptr,
                inst_count,
            )
        }
    }
    pub unsafe fn PSGetSamplers(&self, StartSlot: u32, ppSamplers: &mut [ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).PSGetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn VSGetShader(
        &self,
        ppVertexShader: &mut ID3D11VertexShader,
        ppClassInstances: Option<&mut [*mut c_void]>,
        pNumClassInstances: Option<&mut u32>,
    ) {
        let (inst_ptr, inst_count) = match (ppClassInstances, pNumClassInstances) {
            (Some(s), Some(c)) => (s.as_mut_ptr(), c as *mut _),
            _ => (core::ptr::null_mut(), core::ptr::null_mut()),
        };
        unsafe {
            ((*(*self.0)).VSGetShader)(
                self.0 as _,
                ppVertexShader as *mut _ as _,
                inst_ptr,
                inst_count,
            )
        }
    }
    pub unsafe fn PSGetConstantBuffers(
        &self,
        StartSlot: u32,
        ppConstantBuffers: &mut [ID3D11Buffer],
    ) {
        unsafe {
            ((*(*self.0)).PSGetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn IAGetInputLayout(&self, ppInputLayout: &mut ID3D11InputLayout) {
        unsafe { ((*(*self.0)).IAGetInputLayout)(self.0 as _, ppInputLayout as *mut _ as _) }
    }
    pub unsafe fn IAGetVertexBuffers(
        &self,
        StartSlot: u32,
        ppVertexBuffers: &mut [ID3D11Buffer],
        pStrides: &mut [u32],
        pOffsets: &mut [u32],
    ) {
        let count = ppVertexBuffers
            .len()
            .min(pStrides.len())
            .min(pOffsets.len()) as u32;
        unsafe {
            ((*(*self.0)).IAGetVertexBuffers)(
                self.0 as _,
                StartSlot,
                count,
                ppVertexBuffers.as_mut_ptr() as _,
                pStrides.as_mut_ptr(),
                pOffsets.as_mut_ptr(),
            )
        }
    }
    pub unsafe fn IAGetIndexBuffer(
        &self,
        ppIndexBuffer: &mut ID3D11Buffer,
        Format: &mut DXGI_FORMAT,
        Offset: &mut u32,
    ) {
        unsafe {
            ((*(*self.0)).IAGetIndexBuffer)(
                self.0 as _,
                ppIndexBuffer as *mut _ as _,
                Format as *mut _,
                Offset as *mut _,
            )
        }
    }
    pub unsafe fn GSGetConstantBuffers(
        &self,
        StartSlot: u32,
        ppConstantBuffers: &mut [ID3D11Buffer],
    ) {
        unsafe {
            ((*(*self.0)).GSGetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn GSGetShader(
        &self,
        ppGeometryShader: &mut *mut c_void,
        ppClassInstances: Option<&mut [*mut c_void]>,
        pNumClassInstances: Option<&mut u32>,
    ) {
        let (inst_ptr, inst_count) = match (ppClassInstances, pNumClassInstances) {
            (Some(s), Some(c)) => (s.as_mut_ptr(), c as *mut _),
            _ => (core::ptr::null_mut(), core::ptr::null_mut()),
        };
        unsafe {
            ((*(*self.0)).GSGetShader)(
                self.0 as _,
                ppGeometryShader as *mut _,
                inst_ptr,
                inst_count,
            )
        }
    }
    pub unsafe fn IAGetPrimitiveTopology(&self, pTopology: &mut D3D_PRIMITIVE_TOPOLOGY) {
        unsafe { ((*(*self.0)).IAGetPrimitiveTopology)(self.0 as _, pTopology as *mut _) }
    }
    pub unsafe fn VSGetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &mut [ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).VSGetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn VSGetSamplers(&self, StartSlot: u32, ppSamplers: &mut [ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).VSGetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn GetPredication(&self, ppPredicate: &mut *mut c_void, pPredicateValue: &mut BOOL) {
        unsafe {
            ((*(*self.0)).GetPredication)(
                self.0 as _,
                ppPredicate as *mut _,
                pPredicateValue as *mut _,
            )
        }
    }
    pub unsafe fn GSGetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &mut [ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).GSGetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn GSGetSamplers(&self, StartSlot: u32, ppSamplers: &mut [ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).GSGetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn OMGetRenderTargets(
        &self,
        ppRenderTargetViews: &mut [ID3D11RenderTargetView],
        ppDepthStencilView: Option<&mut ID3D11DepthStencilView>,
    ) {
        let dsv = ppDepthStencilView.map_or(core::ptr::null_mut(), |d| d as *mut _ as _);
        unsafe {
            ((*(*self.0)).OMGetRenderTargets)(
                self.0 as _,
                ppRenderTargetViews.len() as u32,
                ppRenderTargetViews.as_mut_ptr() as _,
                dsv,
            )
        }
    }
    pub unsafe fn OMGetRenderTargetsAndUnorderedAccessViews(
        &self,
        ppRenderTargetViews: &mut [ID3D11RenderTargetView],
        ppDepthStencilView: Option<&mut ID3D11DepthStencilView>,
        UAVStartSlot: u32,
        ppUnorderedAccessViews: &mut [ID3D11UnorderedAccessView],
    ) {
        let dsv = ppDepthStencilView.map_or(core::ptr::null_mut(), |d| d as *mut _ as _);
        unsafe {
            ((*(*self.0)).OMGetRenderTargetsAndUnorderedAccessViews)(
                self.0 as _,
                ppRenderTargetViews.len() as u32,
                ppRenderTargetViews.as_mut_ptr() as _,
                dsv,
                UAVStartSlot,
                ppUnorderedAccessViews.len() as u32,
                ppUnorderedAccessViews.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn OMGetBlendState(
        &self,
        ppBlendState: &mut ID3D11BlendState,
        BlendFactor: Option<&mut [f32; 4]>,
        pSampleMask: Option<&mut u32>,
    ) {
        let factor = BlendFactor.map_or(core::ptr::null_mut(), |f| f as *mut _);
        let mask = pSampleMask.map_or(core::ptr::null_mut(), |m| m as *mut _);
        unsafe {
            ((*(*self.0)).OMGetBlendState)(self.0 as _, ppBlendState as *mut _ as _, factor, mask)
        }
    }
    pub unsafe fn OMGetDepthStencilState(
        &self,
        ppDepthStencilState: &mut ID3D11DepthStencilState,
        pStencilRef: Option<&mut u32>,
    ) {
        let sref = pStencilRef.map_or(core::ptr::null_mut(), |s| s as *mut _);
        unsafe {
            ((*(*self.0)).OMGetDepthStencilState)(
                self.0 as _,
                ppDepthStencilState as *mut _ as _,
                sref,
            )
        }
    }
    pub unsafe fn SOGetTargets(&self, ppSOTargets: &mut [ID3D11Buffer]) {
        unsafe {
            ((*(*self.0)).SOGetTargets)(
                self.0 as _,
                ppSOTargets.len() as u32,
                ppSOTargets.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn RSGetState(&self, ppRasterizerState: &mut ID3D11RasterizerState) {
        unsafe { ((*(*self.0)).RSGetState)(self.0 as _, ppRasterizerState as *mut _ as _) }
    }
    pub unsafe fn RSGetViewports(
        &self,
        pNumViewports: &mut u32,
        pViewports: Option<&mut [D3D11_VIEWPORT]>,
    ) {
        let vp = pViewports.map_or(core::ptr::null_mut(), |v| v.as_mut_ptr());
        unsafe { ((*(*self.0)).RSGetViewports)(self.0 as _, pNumViewports as *mut _, vp) }
    }
    pub unsafe fn RSGetScissorRects(&self, pNumRects: &mut u32, pRects: Option<&mut [RECT]>) {
        let rects = pRects.map_or(core::ptr::null_mut(), |r| r.as_mut_ptr());
        unsafe { ((*(*self.0)).RSGetScissorRects)(self.0 as _, pNumRects as *mut _, rects) }
    }
    pub unsafe fn HSGetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &mut [ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).HSGetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn HSGetShader(
        &self,
        ppHullShader: &mut *mut c_void,
        ppClassInstances: Option<&mut [*mut c_void]>,
        pNumClassInstances: Option<&mut u32>,
    ) {
        let (inst_ptr, inst_count) = match (ppClassInstances, pNumClassInstances) {
            (Some(s), Some(c)) => (s.as_mut_ptr(), c as *mut _),
            _ => (core::ptr::null_mut(), core::ptr::null_mut()),
        };
        unsafe {
            ((*(*self.0)).HSGetShader)(self.0 as _, ppHullShader as *mut _, inst_ptr, inst_count)
        }
    }
    pub unsafe fn HSGetSamplers(&self, StartSlot: u32, ppSamplers: &mut [ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).HSGetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn HSGetConstantBuffers(
        &self,
        StartSlot: u32,
        ppConstantBuffers: &mut [ID3D11Buffer],
    ) {
        unsafe {
            ((*(*self.0)).HSGetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn DSGetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &mut [ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).DSGetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn DSGetShader(
        &self,
        ppDomainShader: &mut *mut c_void,
        ppClassInstances: Option<&mut [*mut c_void]>,
        pNumClassInstances: Option<&mut u32>,
    ) {
        let (inst_ptr, inst_count) = match (ppClassInstances, pNumClassInstances) {
            (Some(s), Some(c)) => (s.as_mut_ptr(), c as *mut _),
            _ => (core::ptr::null_mut(), core::ptr::null_mut()),
        };
        unsafe {
            ((*(*self.0)).DSGetShader)(self.0 as _, ppDomainShader as *mut _, inst_ptr, inst_count)
        }
    }
    pub unsafe fn DSGetSamplers(&self, StartSlot: u32, ppSamplers: &mut [ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).DSGetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn DSGetConstantBuffers(
        &self,
        StartSlot: u32,
        ppConstantBuffers: &mut [ID3D11Buffer],
    ) {
        unsafe {
            ((*(*self.0)).DSGetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn CSGetShaderResources(
        &self,
        StartSlot: u32,
        ppShaderResourceViews: &mut [ID3D11ShaderResourceView],
    ) {
        unsafe {
            ((*(*self.0)).CSGetShaderResources)(
                self.0 as _,
                StartSlot,
                ppShaderResourceViews.len() as u32,
                ppShaderResourceViews.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn CSGetUnorderedAccessViews(
        &self,
        StartSlot: u32,
        ppUnorderedAccessViews: &mut [ID3D11UnorderedAccessView],
    ) {
        unsafe {
            ((*(*self.0)).CSGetUnorderedAccessViews)(
                self.0 as _,
                StartSlot,
                ppUnorderedAccessViews.len() as u32,
                ppUnorderedAccessViews.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn CSGetShader(
        &self,
        ppComputeShader: &mut ID3D11ComputeShader,
        ppClassInstances: Option<&mut [*mut c_void]>,
        pNumClassInstances: Option<&mut u32>,
    ) {
        let (inst_ptr, inst_count) = match (ppClassInstances, pNumClassInstances) {
            (Some(s), Some(c)) => (s.as_mut_ptr(), c as *mut _),
            _ => (core::ptr::null_mut(), core::ptr::null_mut()),
        };
        unsafe {
            ((*(*self.0)).CSGetShader)(
                self.0 as _,
                ppComputeShader as *mut _ as _,
                inst_ptr,
                inst_count,
            )
        }
    }
    pub unsafe fn CSGetSamplers(&self, StartSlot: u32, ppSamplers: &mut [ID3D11SamplerState]) {
        unsafe {
            ((*(*self.0)).CSGetSamplers)(
                self.0 as _,
                StartSlot,
                ppSamplers.len() as u32,
                ppSamplers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn CSGetConstantBuffers(
        &self,
        StartSlot: u32,
        ppConstantBuffers: &mut [ID3D11Buffer],
    ) {
        unsafe {
            ((*(*self.0)).CSGetConstantBuffers)(
                self.0 as _,
                StartSlot,
                ppConstantBuffers.len() as u32,
                ppConstantBuffers.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn ClearState(&self) {
        unsafe { ((*(*self.0)).ClearState)(self.0 as _) }
    }
    pub unsafe fn Flush(&self) {
        unsafe { ((*(*self.0)).Flush)(self.0 as _) }
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        unsafe { ((*(*self.0)).GetContextFlags)(self.0 as _) }
    }
    pub unsafe fn FinishCommandList(
        &self,
        RestoreDeferredContextState: BOOL,
        ppCommandList: &mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            ((*(*self.0)).FinishCommandList)(
                self.0 as _,
                RestoreDeferredContextState,
                ppCommandList as *mut _,
            )
        }
    }
}

#[repr(C)]
pub struct ID3D11DeviceVtbl {
    pub base: IUnknownVtbl,
    pub CreateBuffer: unsafe extern "system" fn(
        this: *mut c_void,
        pDesc: *const D3D11_BUFFER_DESC,
        pInitialData: *const D3D11_SUBRESOURCE_DATA,
        ppBuffer: *mut *mut ID3D11Buffer,
    ) -> HRESULT,
    pub CreateTexture1D: unsafe extern "system" fn(
        this: *mut c_void,
        pDesc: *const D3D11_TEXTURE1D_DESC,
        pInitialData: *const D3D11_SUBRESOURCE_DATA,
        ppTexture1D: *mut *mut ID3D11Texture1D,
    ) -> HRESULT,
    pub CreateTexture2D: unsafe extern "system" fn(
        this: *mut c_void,
        pDesc: *const D3D11_TEXTURE2D_DESC,
        pInitialData: *const D3D11_SUBRESOURCE_DATA,
        ppTexture2D: *mut *mut ID3D11Texture2D,
    ) -> HRESULT,
    pub CreateTexture3D: unsafe extern "system" fn(
        this: *mut c_void,
        pDesc: *const D3D11_TEXTURE3D_DESC,
        pInitialData: *const D3D11_SUBRESOURCE_DATA,
        ppTexture3D: *mut *mut ID3D11Texture3D,
    ) -> HRESULT,
    pub CreateShaderResourceView: unsafe extern "system" fn(
        this: *mut c_void,
        pResource: *mut ID3D11Resource,
        pDesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
        ppSRView: *mut *mut ID3D11ShaderResourceView,
    ) -> HRESULT,
    pub CreateUnorderedAccessView: unsafe extern "system" fn(
        this: *mut c_void,
        pResource: *mut ID3D11Resource,
        pDesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
        ppUAView: *mut *mut ID3D11UnorderedAccessView,
    ) -> HRESULT,
    pub CreateRenderTargetView: unsafe extern "system" fn(
        this: *mut c_void,
        pResource: *mut ID3D11Resource,
        pDesc: *const D3D11_RENDER_TARGET_VIEW_DESC,
        ppRTView: *mut *mut ID3D11RenderTargetView,
    ) -> HRESULT,
    pub CreateDepthStencilView: unsafe extern "system" fn(
        this: *mut c_void,
        pResource: *mut ID3D11Resource,
        pDesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
        ppDepthStencilView: *mut *mut ID3D11DepthStencilView,
    ) -> HRESULT,
    pub CreateInputLayout: unsafe extern "system" fn(
        this: *mut c_void,
        pInputElementDescs: *const D3D11_INPUT_ELEMENT_DESC,
        NumElements: u32,
        pShaderBytecodeWithInputSignature: *const c_void,
        BytecodeLength: usize,
        ppInputLayout: *mut *mut ID3D11InputLayout,
    ) -> HRESULT,
    pub CreateVertexShader: unsafe extern "system" fn(
        this: *mut c_void,
        pShaderBytecode: *const c_void,
        BytecodeLength: usize,
        pClassLinkage: *mut c_void,
        ppVertexShader: *mut *mut ID3D11VertexShader,
    ) -> HRESULT,
    pub CreateGeometryShader: unsafe extern "system" fn(
        this: *mut c_void,
        pShaderBytecode: *const c_void,
        BytecodeLength: usize,
        pClassLinkage: *mut c_void,
        ppGeometryShader: *mut *mut c_void,
    ) -> HRESULT,
    pub CreateGeometryShaderWithStreamOutput: unsafe extern "system" fn(
        this: *mut c_void,
        pShaderBytecode: *const c_void,
        BytecodeLength: usize,
        pSODeclaration: *const c_void,
        NumEntries: u32,
        pBufferStrides: *const u32,
        NumStrides: u32,
        RasterizedStream: u32,
        pClassLinkage: *mut c_void,
        ppGeometryShader: *mut *mut c_void,
    ) -> HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(
        this: *mut c_void,
        pShaderBytecode: *const c_void,
        BytecodeLength: usize,
        pClassLinkage: *mut c_void,
        ppPixelShader: *mut *mut ID3D11PixelShader,
    ) -> HRESULT,
    pub CreateHullShader: unsafe extern "system" fn(
        this: *mut c_void,
        pShaderBytecode: *const c_void,
        BytecodeLength: usize,
        pClassLinkage: *mut c_void,
        ppHullShader: *mut *mut c_void,
    ) -> HRESULT,
    pub CreateDomainShader: unsafe extern "system" fn(
        this: *mut c_void,
        pShaderBytecode: *const c_void,
        BytecodeLength: usize,
        pClassLinkage: *mut c_void,
        ppDomainShader: *mut *mut c_void,
    ) -> HRESULT,
    pub CreateComputeShader: unsafe extern "system" fn(
        this: *mut c_void,
        pShaderBytecode: *const c_void,
        BytecodeLength: usize,
        pClassLinkage: *mut c_void,
        ppComputeShader: *mut *mut ID3D11ComputeShader,
    ) -> HRESULT,
    pub CreateClassLinkage:
        unsafe extern "system" fn(this: *mut c_void, ppLinkage: *mut *mut c_void) -> HRESULT,
    pub CreateBlendState: unsafe extern "system" fn(
        this: *mut c_void,
        pBlendStateDesc: *const D3D11_BLEND_DESC,
        ppBlendState: *mut *mut ID3D11BlendState,
    ) -> HRESULT,
    pub CreateDepthStencilState: unsafe extern "system" fn(
        this: *mut c_void,
        pDepthStencilDesc: *const D3D11_DEPTH_STENCIL_DESC,
        ppDepthStencilState: *mut *mut ID3D11DepthStencilState,
    ) -> HRESULT,
    pub CreateRasterizerState: unsafe extern "system" fn(
        this: *mut c_void,
        pRasterizerDesc: *const D3D11_RASTERIZER_DESC,
        ppRasterizerState: *mut *mut ID3D11RasterizerState,
    ) -> HRESULT,
    pub CreateSamplerState: unsafe extern "system" fn(
        this: *mut c_void,
        pSamplerDesc: *const D3D11_SAMPLER_DESC,
        ppSamplerState: *mut *mut ID3D11SamplerState,
    ) -> HRESULT,
    pub CreateQuery: unsafe extern "system" fn(
        this: *mut c_void,
        pQueryDesc: *const D3D11_QUERY_DESC,
        ppQuery: *mut *mut c_void,
    ) -> HRESULT,
    pub CreatePredicate: unsafe extern "system" fn(
        this: *mut c_void,
        pPredicateDesc: *const D3D11_QUERY_DESC,
        ppPredicate: *mut *mut c_void,
    ) -> HRESULT,
    pub CreateCounter: unsafe extern "system" fn(
        this: *mut c_void,
        pCounterDesc: *const D3D11_COUNTER_DESC,
        ppCounter: *mut *mut c_void,
    ) -> HRESULT,
    pub CheckFormatSupport: unsafe extern "system" fn(
        this: *mut c_void,
        Format: DXGI_FORMAT,
        pFormatSupport: *mut u32,
    ) -> HRESULT,
    pub CheckMultisampleQualityLevels: unsafe extern "system" fn(
        this: *mut c_void,
        Format: DXGI_FORMAT,
        SampleCount: u32,
        pNumQualityLevels: *mut u32,
    ) -> HRESULT,
    pub CheckCounterInfo: unsafe extern "system" fn(this: *mut c_void, pCounterInfo: *mut c_void),
    pub CheckCounter: unsafe extern "system" fn(
        this: *mut c_void,
        pDesc: *const D3D11_COUNTER_DESC,
        pType: *mut u32,
        pActiveCounters: *mut u32,
        szName: *mut u8,
        pNameLength: *mut u32,
        szUnits: *mut u8,
        pUnitsLength: *mut u32,
        szDescription: *mut u8,
        pDescriptionLength: *mut u32,
    ) -> HRESULT,
    pub CheckFeatureSupport: unsafe extern "system" fn(
        this: *mut c_void,
        Feature: u32,
        pFeatureSupportData: *mut c_void,
        FeatureSupportDataSize: u32,
    ) -> HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(
        this: *mut c_void,
        guid: *const GUID,
        pDataSize: *mut u32,
        pData: *mut c_void,
    ) -> HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(
        this: *mut c_void,
        guid: *const GUID,
        DataSize: u32,
        pData: *const c_void,
    ) -> HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        this: *mut c_void,
        guid: *const GUID,
        pData: *const IUnknown,
    ) -> HRESULT,
    pub GetFeatureLevel: unsafe extern "system" fn(this: *mut c_void) -> D3D_FEATURE_LEVEL,
    pub GetCreationFlags: unsafe extern "system" fn(this: *mut c_void) -> u32,
    pub GetDeviceRemovedReason: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
    pub GetImmediateContext: unsafe extern "system" fn(
        this: *mut c_void,
        ppImmediateContext: *mut *mut ID3D11DeviceContext,
    ),
    pub SetExceptionMode: unsafe extern "system" fn(this: *mut c_void, RaiseFlags: u32) -> HRESULT,
    pub GetExceptionMode: unsafe extern "system" fn(this: *mut c_void) -> u32,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ID3D11Device(pub *mut *const ID3D11DeviceVtbl);

impl ID3D11Device {
    pub unsafe fn QueryInterface(&self, riid: &GUID, ppvObject: &mut *mut c_void) -> HRESULT {
        unsafe { IUnknown(self.0 as _).QueryInterface(riid, ppvObject) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).AddRef() }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { IUnknown(self.0 as _).Release() }
    }
    pub unsafe fn CreateBuffer(
        &self,
        pDesc: &D3D11_BUFFER_DESC,
        pInitialData: Option<&D3D11_SUBRESOURCE_DATA>,
        ppBuffer: &mut ID3D11Buffer,
    ) -> HRESULT {
        let init_data = pInitialData.map_or(core::ptr::null(), |d| d as *const _);
        unsafe {
            ((*(*self.0)).CreateBuffer)(
                self.0 as _,
                pDesc as *const _,
                init_data,
                ppBuffer as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateTexture1D(
        &self,
        pDesc: &D3D11_TEXTURE1D_DESC,
        pInitialData: Option<&D3D11_SUBRESOURCE_DATA>,
        ppTexture1D: &mut ID3D11Texture1D,
    ) -> HRESULT {
        let init_data = pInitialData.map_or(core::ptr::null(), |d| d as *const _);
        unsafe {
            ((*(*self.0)).CreateTexture1D)(
                self.0 as _,
                pDesc as *const _,
                init_data,
                ppTexture1D as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateTexture2D(
        &self,
        pDesc: &D3D11_TEXTURE2D_DESC,
        pInitialData: Option<&D3D11_SUBRESOURCE_DATA>,
        ppTexture2D: &mut ID3D11Texture2D,
    ) -> HRESULT {
        let init_data = pInitialData.map_or(core::ptr::null(), |d| d as *const _);
        unsafe {
            ((*(*self.0)).CreateTexture2D)(
                self.0 as _,
                pDesc as *const _,
                init_data,
                ppTexture2D as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateTexture3D(
        &self,
        pDesc: &D3D11_TEXTURE3D_DESC,
        pInitialData: Option<&D3D11_SUBRESOURCE_DATA>,
        ppTexture3D: &mut ID3D11Texture3D,
    ) -> HRESULT {
        let init_data = pInitialData.map_or(core::ptr::null(), |d| d as *const _);
        unsafe {
            ((*(*self.0)).CreateTexture3D)(
                self.0 as _,
                pDesc as *const _,
                init_data,
                ppTexture3D as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateShaderResourceView(
        &self,
        pResource: &ID3D11Resource,
        pDesc: Option<&D3D11_SHADER_RESOURCE_VIEW_DESC>,
        ppSRView: &mut ID3D11ShaderResourceView,
    ) -> HRESULT {
        let desc = pDesc.map_or(core::ptr::null(), |d| d as *const _);
        unsafe {
            ((*(*self.0)).CreateShaderResourceView)(
                self.0 as _,
                pResource.0 as _,
                desc,
                ppSRView as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateUnorderedAccessView(
        &self,
        pResource: &ID3D11Resource,
        pDesc: Option<&D3D11_UNORDERED_ACCESS_VIEW_DESC>,
        ppUAView: &mut ID3D11UnorderedAccessView,
    ) -> HRESULT {
        let desc = pDesc.map_or(core::ptr::null(), |d| d as *const _);
        unsafe {
            ((*(*self.0)).CreateUnorderedAccessView)(
                self.0 as _,
                pResource.0 as _,
                desc,
                ppUAView as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateRenderTargetView(
        &self,
        pResource: &ID3D11Resource,
        pDesc: Option<&D3D11_RENDER_TARGET_VIEW_DESC>,
        ppRTView: &mut ID3D11RenderTargetView,
    ) -> HRESULT {
        let desc = pDesc.map_or(core::ptr::null(), |d| d as *const _);
        unsafe {
            ((*(*self.0)).CreateRenderTargetView)(
                self.0 as _,
                pResource.0 as _,
                desc,
                ppRTView as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateDepthStencilView(
        &self,
        pResource: &ID3D11Resource,
        pDesc: Option<&D3D11_DEPTH_STENCIL_VIEW_DESC>,
        ppDepthStencilView: &mut ID3D11DepthStencilView,
    ) -> HRESULT {
        let desc = pDesc.map_or(core::ptr::null(), |d| d as *const _);
        unsafe {
            ((*(*self.0)).CreateDepthStencilView)(
                self.0 as _,
                pResource.0 as _,
                desc,
                ppDepthStencilView as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateInputLayout(
        &self,
        pInputElementDescs: &[D3D11_INPUT_ELEMENT_DESC],
        pShaderBytecodeWithInputSignature: &[u8],
        ppInputLayout: &mut ID3D11InputLayout,
    ) -> HRESULT {
        unsafe {
            ((*(*self.0)).CreateInputLayout)(
                self.0 as _,
                pInputElementDescs.as_ptr(),
                pInputElementDescs.len() as u32,
                pShaderBytecodeWithInputSignature.as_ptr() as _,
                pShaderBytecodeWithInputSignature.len(),
                ppInputLayout as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateVertexShader(
        &self,
        pShaderBytecode: &[u8],
        pClassLinkage: Option<*mut c_void>,
        ppVertexShader: &mut ID3D11VertexShader,
    ) -> HRESULT {
        let linkage = pClassLinkage.unwrap_or(core::ptr::null_mut());
        unsafe {
            ((*(*self.0)).CreateVertexShader)(
                self.0 as _,
                pShaderBytecode.as_ptr() as _,
                pShaderBytecode.len(),
                linkage,
                ppVertexShader as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateGeometryShader(
        &self,
        pShaderBytecode: &[u8],
        pClassLinkage: Option<*mut c_void>,
        ppGeometryShader: &mut *mut c_void,
    ) -> HRESULT {
        let linkage = pClassLinkage.unwrap_or(core::ptr::null_mut());
        unsafe {
            ((*(*self.0)).CreateGeometryShader)(
                self.0 as _,
                pShaderBytecode.as_ptr() as _,
                pShaderBytecode.len(),
                linkage,
                ppGeometryShader as *mut _,
            )
        }
    }
    pub unsafe fn CreatePixelShader(
        &self,
        pShaderBytecode: &[u8],
        pClassLinkage: Option<*mut c_void>,
        ppPixelShader: &mut ID3D11PixelShader,
    ) -> HRESULT {
        let linkage = pClassLinkage.unwrap_or(core::ptr::null_mut());
        unsafe {
            ((*(*self.0)).CreatePixelShader)(
                self.0 as _,
                pShaderBytecode.as_ptr() as _,
                pShaderBytecode.len(),
                linkage,
                ppPixelShader as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateHullShader(
        &self,
        pShaderBytecode: &[u8],
        pClassLinkage: Option<*mut c_void>,
        ppHullShader: &mut *mut c_void,
    ) -> HRESULT {
        let linkage = pClassLinkage.unwrap_or(core::ptr::null_mut());
        unsafe {
            ((*(*self.0)).CreateHullShader)(
                self.0 as _,
                pShaderBytecode.as_ptr() as _,
                pShaderBytecode.len(),
                linkage,
                ppHullShader as *mut _,
            )
        }
    }
    pub unsafe fn CreateDomainShader(
        &self,
        pShaderBytecode: &[u8],
        pClassLinkage: Option<*mut c_void>,
        ppDomainShader: &mut *mut c_void,
    ) -> HRESULT {
        let linkage = pClassLinkage.unwrap_or(core::ptr::null_mut());
        unsafe {
            ((*(*self.0)).CreateDomainShader)(
                self.0 as _,
                pShaderBytecode.as_ptr() as _,
                pShaderBytecode.len(),
                linkage,
                ppDomainShader as *mut _,
            )
        }
    }
    pub unsafe fn CreateComputeShader(
        &self,
        pShaderBytecode: &[u8],
        pClassLinkage: Option<*mut c_void>,
        ppComputeShader: &mut ID3D11ComputeShader,
    ) -> HRESULT {
        let linkage = pClassLinkage.unwrap_or(core::ptr::null_mut());
        unsafe {
            ((*(*self.0)).CreateComputeShader)(
                self.0 as _,
                pShaderBytecode.as_ptr() as _,
                pShaderBytecode.len(),
                linkage,
                ppComputeShader as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateBlendState(
        &self,
        pBlendStateDesc: &D3D11_BLEND_DESC,
        ppBlendState: &mut ID3D11BlendState,
    ) -> HRESULT {
        unsafe {
            ((*(*self.0)).CreateBlendState)(
                self.0 as _,
                pBlendStateDesc as *const _,
                ppBlendState as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateDepthStencilState(
        &self,
        pDepthStencilDesc: &D3D11_DEPTH_STENCIL_DESC,
        ppDepthStencilState: &mut ID3D11DepthStencilState,
    ) -> HRESULT {
        unsafe {
            ((*(*self.0)).CreateDepthStencilState)(
                self.0 as _,
                pDepthStencilDesc as *const _,
                ppDepthStencilState as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateRasterizerState(
        &self,
        pRasterizerDesc: &D3D11_RASTERIZER_DESC,
        ppRasterizerState: &mut ID3D11RasterizerState,
    ) -> HRESULT {
        unsafe {
            ((*(*self.0)).CreateRasterizerState)(
                self.0 as _,
                pRasterizerDesc as *const _,
                ppRasterizerState as *mut _ as _,
            )
        }
    }
    pub unsafe fn CreateSamplerState(
        &self,
        pSamplerDesc: &D3D11_SAMPLER_DESC,
        ppSamplerState: &mut ID3D11SamplerState,
    ) -> HRESULT {
        unsafe {
            ((*(*self.0)).CreateSamplerState)(
                self.0 as _,
                pSamplerDesc as *const _,
                ppSamplerState as *mut _ as _,
            )
        }
    }
    pub unsafe fn CheckFormatSupport(
        &self,
        Format: DXGI_FORMAT,
        pFormatSupport: &mut u32,
    ) -> HRESULT {
        unsafe { ((*(*self.0)).CheckFormatSupport)(self.0 as _, Format, pFormatSupport as *mut _) }
    }
    pub unsafe fn CheckMultisampleQualityLevels(
        &self,
        Format: DXGI_FORMAT,
        SampleCount: u32,
        pNumQualityLevels: &mut u32,
    ) -> HRESULT {
        unsafe {
            ((*(*self.0)).CheckMultisampleQualityLevels)(
                self.0 as _,
                Format,
                SampleCount,
                pNumQualityLevels as *mut _,
            )
        }
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        Feature: u32,
        pFeatureSupportData: *mut c_void,
        FeatureSupportDataSize: u32,
    ) -> HRESULT {
        unsafe {
            ((*(*self.0)).CheckFeatureSupport)(
                self.0 as _,
                Feature,
                pFeatureSupportData,
                FeatureSupportDataSize,
            )
        }
    }
    pub unsafe fn GetFeatureLevel(&self) -> D3D_FEATURE_LEVEL {
        unsafe { ((*(*self.0)).GetFeatureLevel)(self.0 as _) }
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        unsafe { ((*(*self.0)).GetCreationFlags)(self.0 as _) }
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> HRESULT {
        unsafe { ((*(*self.0)).GetDeviceRemovedReason)(self.0 as _) }
    }
    pub unsafe fn GetImmediateContext(&self, ppImmediateContext: &mut ID3D11DeviceContext) {
        unsafe {
            ((*(*self.0)).GetImmediateContext)(self.0 as _, ppImmediateContext as *mut _ as _)
        }
    }
}

pub mod ffi {
    use super::*;

    #[link(name = "d3d11")]
    unsafe extern "system" {
        pub fn D3D11CreateDevice(
            pAdapter: *mut c_void,
            DriverType: D3D_DRIVER_TYPE,
            Software: *mut c_void,
            Flags: u32,
            pFeatureLevels: *const D3D_FEATURE_LEVEL,
            FeatureLevels: u32,
            SDKVersion: u32,
            ppDevice: *mut *mut ID3D11Device,
            pFeatureLevel: *mut D3D_FEATURE_LEVEL,
            ppImmediateContext: *mut *mut ID3D11DeviceContext,
        ) -> HRESULT;

        pub fn D3D11CreateDeviceAndSwapChain(
            pAdapter: *mut c_void,
            DriverType: D3D_DRIVER_TYPE,
            Software: *mut c_void,
            Flags: u32,
            pFeatureLevels: *const D3D_FEATURE_LEVEL,
            FeatureLevels: u32,
            SDKVersion: u32,
            pSwapChainDesc: *const DXGI_SWAP_CHAIN_DESC,
            ppSwapChain: *mut *mut IDXGISwapChain,
            ppDevice: *mut *mut ID3D11Device,
            pFeatureLevel: *mut D3D_FEATURE_LEVEL,
            ppImmediateContext: *mut *mut ID3D11DeviceContext,
        ) -> HRESULT;
    }
}

pub unsafe fn D3D11CreateDevice(
    pAdapter: Option<*mut c_void>,
    DriverType: D3D_DRIVER_TYPE,
    Software: Option<*mut c_void>,
    Flags: u32,
    pFeatureLevels: Option<&[D3D_FEATURE_LEVEL]>,
    SDKVersion: u32,
    ppDevice: Option<&mut ID3D11Device>,
    pFeatureLevel: Option<&mut D3D_FEATURE_LEVEL>,
    ppImmediateContext: Option<&mut ID3D11DeviceContext>,
) -> HRESULT {
    let (fl_ptr, fl_len) = match pFeatureLevels {
        Some(slice) => (slice.as_ptr(), slice.len() as u32),
        None => (core::ptr::null(), 0),
    };
    let dev_ptr = ppDevice.map_or(core::ptr::null_mut(), |d| d as *mut _ as _);
    let fl_out_ptr = pFeatureLevel.map_or(core::ptr::null_mut(), |f| f as *mut _);
    let ctx_ptr = ppImmediateContext.map_or(core::ptr::null_mut(), |c| c as *mut _ as _);
    unsafe {
        ffi::D3D11CreateDevice(
            pAdapter.unwrap_or(core::ptr::null_mut()),
            DriverType,
            Software.unwrap_or(core::ptr::null_mut()),
            Flags,
            fl_ptr,
            fl_len,
            SDKVersion,
            dev_ptr,
            fl_out_ptr,
            ctx_ptr,
        )
    }
}

pub unsafe fn D3D11CreateDeviceAndSwapChain(
    pAdapter: Option<*mut c_void>,
    DriverType: D3D_DRIVER_TYPE,
    Software: Option<*mut c_void>,
    Flags: u32,
    pFeatureLevels: Option<&[D3D_FEATURE_LEVEL]>,
    SDKVersion: u32,
    pSwapChainDesc: Option<&DXGI_SWAP_CHAIN_DESC>,
    ppSwapChain: Option<&mut IDXGISwapChain>,
    ppDevice: Option<&mut ID3D11Device>,
    pFeatureLevel: Option<&mut D3D_FEATURE_LEVEL>,
    ppImmediateContext: Option<&mut ID3D11DeviceContext>,
) -> HRESULT {
    let (fl_ptr, fl_len) = match pFeatureLevels {
        Some(slice) => (slice.as_ptr(), slice.len() as u32),
        None => (core::ptr::null(), 0),
    };
    let sc_desc_ptr = pSwapChainDesc.map_or(core::ptr::null(), |s| s as *const _);
    let sc_ptr = ppSwapChain.map_or(core::ptr::null_mut(), |s| s as *mut _ as _);
    let dev_ptr = ppDevice.map_or(core::ptr::null_mut(), |d| d as *mut _ as _);
    let fl_out_ptr = pFeatureLevel.map_or(core::ptr::null_mut(), |f| f as *mut _);
    let ctx_ptr = ppImmediateContext.map_or(core::ptr::null_mut(), |c| c as *mut _ as _);
    unsafe {
        ffi::D3D11CreateDeviceAndSwapChain(
            pAdapter.unwrap_or(core::ptr::null_mut()),
            DriverType,
            Software.unwrap_or(core::ptr::null_mut()),
            Flags,
            fl_ptr,
            fl_len,
            SDKVersion,
            sc_desc_ptr,
            sc_ptr,
            dev_ptr,
            fl_out_ptr,
            ctx_ptr,
        )
    }
}
