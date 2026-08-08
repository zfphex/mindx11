use mindx11::*;

fn main() {
    unsafe {
        let feature_levels = [
            D3D_FEATURE_LEVEL::_11_0,
            D3D_FEATURE_LEVEL::_10_1,
            D3D_FEATURE_LEVEL::_10_0,
        ];

        let (device, _feature_level, context) = match D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE::HARDWARE,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            Some(&feature_levels),
            D3D11_SDK_VERSION,
        ) {
            Ok(res) => {
                println!(
                    "D3D11CreateDevice Hardware succeeded! device: {:?}, context: {:?}, level: {:?}",
                    res.0.0, res.2.0, res.1
                );
                res
            }
            Err(hr) => {
                println!(
                    "Hardware device failed with {}. Trying WARP driver...",
                    hr
                );
                match D3D11CreateDevice(
                    None,
                    D3D_DRIVER_TYPE::WARP,
                    None,
                    D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                    Some(&feature_levels),
                    D3D11_SDK_VERSION,
                ) {
                    Ok(res) => {
                        println!(
                            "D3D11CreateDevice WARP succeeded! device: {:?}, context: {:?}, level: {:?}",
                            res.0.0, res.2.0, res.1
                        );
                        res
                    }
                    Err(hr2) => {
                        println!("Failed to create WARP D3D11 device: {}", hr2);
                        return;
                    }
                }
            }
        };

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

        match device.CreateBuffer(&buffer_desc, None) {
            Ok(buffer) => {
                println!("Successfully created D3D11 Buffer! Pointer: {:?}", buffer.0);
                let desc = buffer.GetDesc();
                println!("Buffer ByteWidth: {}", desc.ByteWidth);
                let refcount = buffer.Release();
                println!("Buffer Release ref count: {}", refcount);
            }
            Err(hr_buf) => {
                println!("CreateBuffer failed: {}", hr_buf);
            }
        }

        let shader_code = b"float4 main(float4 pos : POSITION) : SV_POSITION { return pos; }\0";

        match D3DCompile(
            shader_code,
            None,
            None,
            None,
            Some(c"main"),
            c"vs_5_0",
            D3DCOMPILE_ENABLE_STRICTNESS,
            0,
        ) {
            Ok((code_blob, _)) => {
                println!(
                    "Successfully compiled HLSL Vertex Shader! Blob Byte Size: {}",
                    code_blob.GetBufferSize()
                );
                let slice = code_blob.as_slice();
                println!(
                    "First 4 bytes of bytecode: {:?}",
                    &slice[..4.min(slice.len())]
                );
                let refcount = code_blob.Release();
                println!("Blob Release ref count: {}", refcount);
            }
            Err((hr_compile, error_blob_opt)) => {
                println!(
                    "HLSL Compilation Result: {}",
                    hr_compile
                );
                if let Some(err_blob) = error_blob_opt {
                    let slice = err_blob.as_slice();
                    let err_msg = core::str::from_utf8(slice).unwrap_or("Invalid UTF-8");
                    println!("Shader compiler error: {}", err_msg);
                    err_blob.Release();
                }
            }
        }

        let ctx_refs = context.Release();
        println!("DeviceContext Release ref count: {}", ctx_refs);

        let dev_refs = device.Release();
        println!("Device Release ref count: {}", dev_refs);

        println!("Zero-dependency Direct3D 11 test completed successfully!");
    }
}
