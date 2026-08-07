use core::ffi::c_void;
use mindx11::*;

fn main() {
    unsafe {
        let mut p_device: *mut c_void = core::ptr::null_mut();
        let mut p_context: *mut c_void = core::ptr::null_mut();
        let mut feature_level: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL::_11_0;

        let feature_levels = [
            D3D_FEATURE_LEVEL::_11_0,
            D3D_FEATURE_LEVEL::_10_1,
            D3D_FEATURE_LEVEL::_10_0,
        ];

        let hr = D3D11CreateDevice(
            core::ptr::null_mut(),
            D3D_DRIVER_TYPE::HARDWARE,
            core::ptr::null_mut(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            feature_levels.as_ptr(),
            feature_levels.len() as u32,
            D3D11_SDK_VERSION,
            &mut p_device as *mut *mut c_void as _,
            &mut feature_level,
            &mut p_context as *mut *mut c_void as _,
        );

        println!(
            "D3D11CreateDevice hr: 0x{:08X}, p_device: {:?}, p_context: {:?}, level: {:?}",
            hr as u32, p_device, p_context, feature_level
        );

        if hr < 0 || p_device.is_null() {
            println!("Failed to create D3D11 hardware device. Trying WARP driver...");
            let hr2 = D3D11CreateDevice(
                core::ptr::null_mut(),
                D3D_DRIVER_TYPE::WARP,
                core::ptr::null_mut(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                feature_levels.as_ptr(),
                feature_levels.len() as u32,
                D3D11_SDK_VERSION,
                &mut p_device as *mut *mut c_void as _,
                &mut feature_level,
                &mut p_context as *mut *mut c_void as _,
            );
            if hr2 < 0 || p_device.is_null() {
                println!("Failed to create WARP D3D11 device.");
                return;
            }
        }

        let dev = ID3D11Device(p_device as _);
        let ctx = ID3D11DeviceContext(p_context as _);

        println!("Successfully created Direct3D 11 Device!");

        let addref_res = dev.AddRef();
        println!("Device AddRef returned ref count: {}", addref_res);
        dev.Release();

        let buffer_desc = D3D11_BUFFER_DESC {
            ByteWidth: 64,
            Usage: D3D11_USAGE::DEFAULT,
            BindFlags: D3D11_BIND_VERTEX_BUFFER,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: 0,
        };

        let mut p_buffer: *mut c_void = core::ptr::null_mut();
        let hr_buf = dev.CreateBuffer(
            &buffer_desc,
            core::ptr::null(),
            &mut p_buffer as *mut *mut c_void as _,
        );
        println!(
            "CreateBuffer hr: 0x{:08X}, p_buffer: {:?}",
            hr_buf as u32, p_buffer
        );

        if hr_buf >= 0 && !p_buffer.is_null() {
            let buf = ID3D11Buffer(p_buffer as _);
            println!("Successfully created D3D11 Buffer!");
            let mut desc = D3D11_BUFFER_DESC {
                ByteWidth: 0,
                Usage: D3D11_USAGE::DEFAULT,
                BindFlags: 0,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            buf.GetDesc(&mut desc);
            println!("Buffer ByteWidth: {}", desc.ByteWidth);
            let refcount = buf.Release();
            println!("Buffer Release ref count: {}", refcount);
        }

        let shader_code = b"float4 main(float4 pos : POSITION) : SV_POSITION { return pos; }\0";
        let mut p_code_blob: *mut c_void = core::ptr::null_mut();
        let mut p_error_blob: *mut c_void = core::ptr::null_mut();

        let hr_compile = D3DCompile(
            shader_code.as_ptr() as _,
            shader_code.len() - 1,
            core::ptr::null(),
            core::ptr::null(),
            core::ptr::null_mut(),
            b"main\0".as_ptr(),
            b"vs_5_0\0".as_ptr(),
            D3DCOMPILE_ENABLE_STRICTNESS,
            0,
            &mut p_code_blob as *mut *mut c_void as _,
            &mut p_error_blob as *mut *mut c_void as _,
        );

        if hr_compile >= 0 && !p_code_blob.is_null() {
            let blob = ID3D10Blob(p_code_blob as _);
            println!(
                "Successfully compiled HLSL Vertex Shader! Blob Byte Size: {}",
                blob.GetBufferSize()
            );
            let refcount = blob.Release();
            println!("Blob Release ref count: {}", refcount);
        } else {
            println!(
                "HLSL Compilation Result: HRESULT 0x{:08X}",
                hr_compile as u32
            );
            if !p_error_blob.is_null() {
                let err_blob = ID3D10Blob(p_error_blob as _);
                let msg_ptr = err_blob.GetBufferPointer() as *const u8;
                let msg_len = err_blob.GetBufferSize();
                let slice = core::slice::from_raw_parts(msg_ptr, msg_len);
                let err_msg = core::str::from_utf8(slice).unwrap_or("Invalid UTF-8");
                println!("Shader compiler error: {}", err_msg);
                err_blob.Release();
            }
        }

        let ctx_refs = ctx.Release();
        println!("DeviceContext Release ref count: {}", ctx_refs);

        let dev_refs = dev.Release();
        println!("Device Release ref count: {}", dev_refs);

        println!("Zero-dependency Direct3D 11 test completed successfully!");
    }
}
