use mindx11::*;

fn main() {
    unsafe {
        let mut device = ID3D11Device(core::ptr::null_mut());
        let mut context = ID3D11DeviceContext(core::ptr::null_mut());
        let mut feature_level = D3D_FEATURE_LEVEL::_11_0;

        let feature_levels = [
            D3D_FEATURE_LEVEL::_11_0,
            D3D_FEATURE_LEVEL::_10_1,
            D3D_FEATURE_LEVEL::_10_0,
        ];

        let hr = D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE::HARDWARE,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            Some(&feature_levels),
            D3D11_SDK_VERSION,
            Some(&mut device),
            Some(&mut feature_level),
            Some(&mut context),
        );

        println!(
            "D3D11CreateDevice hr: 0x{:08X}, p_device: {:?}, p_context: {:?}, level: {:?}",
            hr as u32, device.0, context.0, feature_level
        );

        if hr < 0 || device.0.is_null() {
            println!("Failed to create D3D11 hardware device. Trying WARP driver...");
            let hr2 = D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE::WARP,
                None,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                Some(&feature_levels),
                D3D11_SDK_VERSION,
                Some(&mut device),
                Some(&mut feature_level),
                Some(&mut context),
            );
            if hr2 < 0 || device.0.is_null() {
                println!("Failed to create WARP D3D11 device.");
                return;
            }
        }

        println!("Successfully created Direct3D 11 Device!");

        let addref_res = device.AddRef();
        println!("Device AddRef returned ref count: {}", addref_res);
        device.Release();

        let buffer_desc = D3D11_BUFFER_DESC {
            ByteWidth: 64,
            Usage: D3D11_USAGE::DEFAULT,
            BindFlags: D3D11_BIND_VERTEX_BUFFER,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: 0,
        };

        let mut buffer = ID3D11Buffer(core::ptr::null_mut());
        let hr_buf = device.CreateBuffer(&buffer_desc, None, &mut buffer);
        println!(
            "CreateBuffer hr: 0x{:08X}, p_buffer: {:?}",
            hr_buf as u32, buffer.0
        );

        if hr_buf >= 0 && !buffer.0.is_null() {
            println!("Successfully created D3D11 Buffer!");
            let mut desc = D3D11_BUFFER_DESC {
                ByteWidth: 0,
                Usage: D3D11_USAGE::DEFAULT,
                BindFlags: 0,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            buffer.GetDesc(&mut desc);
            println!("Buffer ByteWidth: {}", desc.ByteWidth);
            let refcount = buffer.Release();
            println!("Buffer Release ref count: {}", refcount);
        }

        let shader_code = b"float4 main(float4 pos : POSITION) : SV_POSITION { return pos; }\0";
        let mut code_blob = ID3D10Blob(core::ptr::null_mut());
        let mut error_blob = ID3D10Blob(core::ptr::null_mut());

        let hr_compile = D3DCompile(
            shader_code,
            None,
            None,
            None,
            Some(b"main\0"),
            b"vs_5_0\0",
            D3DCOMPILE_ENABLE_STRICTNESS,
            0,
            &mut code_blob,
            Some(&mut error_blob),
        );

        if hr_compile >= 0 && !code_blob.0.is_null() {
            println!(
                "Successfully compiled HLSL Vertex Shader! Blob Byte Size: {}",
                code_blob.GetBufferSize()
            );
            let refcount = code_blob.Release();
            println!("Blob Release ref count: {}", refcount);
        } else {
            println!(
                "HLSL Compilation Result: HRESULT 0x{:08X}",
                hr_compile as u32
            );
            if !error_blob.0.is_null() {
                let msg_ptr = error_blob.GetBufferPointer() as *const u8;
                let msg_len = error_blob.GetBufferSize();
                let slice = core::slice::from_raw_parts(msg_ptr, msg_len);
                let err_msg = core::str::from_utf8(slice).unwrap_or("Invalid UTF-8");
                println!("Shader compiler error: {}", err_msg);
                error_blob.Release();
            }
        }

        let ctx_refs = context.Release();
        println!("DeviceContext Release ref count: {}", ctx_refs);

        let dev_refs = device.Release();
        println!("Device Release ref count: {}", dev_refs);

        println!("Zero-dependency Direct3D 11 test completed successfully!");
    }
}
