use clap::{Parser, ValueEnum};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "GPU hash calculator (Rust)", long_about = None)]
struct Args {
    /// Input file to hash
    #[arg(short, long)]
    input: PathBuf,

    /// Hash algorithm (sha256/md5/...)
    #[arg(short, long, default_value = "sha256")]
    hash: String,

    /// GPU backend (auto/cuda/opencl)
    #[arg(short = 'g', long, default_value = "auto")]
    gpu: GpuBackend,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum GpuBackend {
    Auto,
    Cuda,
    Opencl,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // 讀取檔案內容
    let input = fs::read(&args.input)?;
    println!("Loaded input file: {:?} ({} bytes)", args.input, input.len());

    // 根據參數初始化 GPU
    match args.gpu {
        GpuBackend::Auto => {
            // 先試 CUDA，再試 OpenCL
            if try_cuda(&input, &args.hash)? {
                return Ok(());
            }
            if try_opencl(&input, &args.hash)? {
                return Ok(());
            }
            eprintln!("No supported GPU backend found (nVidia CUDA, AMD/INTEL OpenCL)!");
            std::process::exit(1);
        }
        GpuBackend::Cuda => {
            if !try_cuda(&input, &args.hash)? {
                eprintln!("CUDA backend not available or failed!");
                std::process::exit(1);
            }
        }
        GpuBackend::Opencl => {
            if !try_opencl(&input, &args.hash)? {
                eprintln!("OpenCL backend (AMD/INTEL) not available or failed!");
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

fn try_cuda(input: &[u8], hash: &str) -> Result<bool, Box<dyn Error>> {
    // 嘗試初始化 CUDA
    match cust::init(cust::CudaFlags::empty()) {
        Ok(_) => (),
        Err(_) => {
            println!("CUDA not available on this system.");
            return Ok(false);
        }
    }

    // 檢查至少有一個裝置
    let device_count = cust::device::Device::num_devices()?;
    if device_count == 0 {
        println!("No CUDA device found.");
        return Ok(false);
    }
    println!("Found {} CUDA device(s).", device_count);

    // 只簡單做 hash 範例 (尚未實作 SHA256)
    // 若要完整 SHA256 kernel 請參考 cudacracker 或 open-source CUDA SHA256 範例

    // ...（請填入 cust CUDA hash kernel 實作）
    println!("CUDA backend hash computation not fully implemented (stub).");
    Ok(true)
}

fn try_opencl(input: &[u8], hash: &str) -> Result<bool, Box<dyn Error>> {
    // 嘗試初始化 OpenCL
    #[cfg(feature = "opencl")]
    {
        use opencl3::platform::get_platforms;
        let plats = get_platforms()?;
        if plats.is_empty() {
            println!("No OpenCL platforms found.");
            return Ok(false);
        }
        println!("Found {} OpenCL platform(s).", plats.len());
        // ...（可繼續查詢裝置型號與類型，選取 GPU/CPU 等）
        // ...（請填入 OpenCL hash kernel 實作）
        println!("OpenCL backend hash computation not fully implemented (stub).");
        Ok(true)
    }
    #[cfg(not(feature = "opencl"))]
    {
        println!("OpenCL support not compiled in.");
        Ok(false)
    }
}
