// 使用 Rust + CUDA 進行 GPU 加速的 hash 計算範例
// 本範例仿寫自 https://vaktibabat.github.io/posts/cudacracker/
// 需求：
// - CUDA 開發環境
// - Rust crate: cust (https://github.com/dennis-hamester/cust)

use cust::prelude::*;
use std::error::Error;

// CUDA kernel (SHA256 hash 為例)
const KERNEL_SRC: &str = r#"
extern "C" __global__ void hash_kernel(const unsigned char* input, unsigned char* output, int len) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < len) {
        // 這裡以簡單按位取反作為範例，實際請用 SHA256 實現
        output[i] = ~input[i];
    }
}
"#;

fn main() -> Result<(), Box<dyn Error>> {
    // 初始化 CUDA
    cust::init(cust::CudaFlags::empty())?;
    let device = Device::get_device(0)?;
    let ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

    // 載入 CUDA module
    let module = Module::from_ptx(KERNEL_SRC, &[])?;

    // 準備 kernel
    let func = module.get_function("hash_kernel")?;

    // 待 hash 的輸入資料
    let input: Vec<u8> = b"hello, cuda hash!".to_vec();
    let len = input.len();

    // 分配 GPU 記憶體
    let d_input = DeviceBuffer::from_slice(&input)?;
    let mut d_output = DeviceBuffer::zeroed(len)?;

    // 啟動 kernel
    let block_size = 128;
    let grid_size = (len as u32 + block_size - 1) / block_size;
    unsafe {
        launch!(
            func<<<grid_size, block_size, 0, Stream::null()>>>(
                d_input.as_device_ptr(),
                d_output.as_device_ptr(),
                len as i32
            )
        )?;
    }

    // 從 GPU 取回結果
    let mut result = vec![0u8; len];
    d_output.copy_to(&mut result)?;

    println!("GPU hash (bitwise NOT sample): {:x?}", result);

    // 實際應用請將 kernel 實作替換為 SHA256 算法
    Ok(())
}
