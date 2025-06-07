use clap::{ArgGroup, Parser};

mod cpu_hasher;
#[cfg(feature = "opencl")]
mod gpu_hasher;
#[cfg(feature = "opencl")]
mod ocl;
mod plotter;
mod poc_hashing;
mod scheduler;
mod shabal256;
mod utils;
mod writer;
mod buffer;

use crate::plotter::{Plotter, PlotterTask};
use crate::utils::set_low_prio;

#[derive(Parser, Debug)]
#[command(name = "signum-plotter", version, author, about, arg_required_else_help = true)]
#[command(group(ArgGroup::new("processing").args(["cpu", "gpu"]).multiple(true)))]
struct Args {
    #[arg(short = 'd', long = "ddio", help = "Disables direct i/o")]
    disable_direct_io: bool,
    #[arg(short = 'a', long = "daio", help = "Disables async writing (single RAM buffer mode)")]
    disable_async_io: bool,
    #[arg(short = 'l', long = "prio", help = "Runs with low priority")]
    low_priority: bool,
    #[arg(short = 'q', long = "quiet", help = "Runs in non-verbose mode")]
    quiet: bool,
    #[arg(short = 'b', long = "bench", help = "Runs in xPU benchmark mode")]
    benchmark: bool,

    #[arg(short = 'i', long = "id", required_unless_present = "ocl-devices", value_name = "numeric_ID")]
    numeric_id: Option<u64>,
    #[arg(short = 's', long = "sn", required_unless_present = "ocl-devices", value_name = "start_nonce")]
    start_nonce: Option<u64>,
    #[arg(short = 'n', long = "n", required_unless_present = "ocl-devices", value_name = "nonces")]
    nonces: Option<u64>,
    #[arg(short = 'p', long = "path", value_name = "path")]
    path: Option<String>,
    #[arg(short = 'm', long = "mem", value_name = "memory")]
    memory: Option<String>,
    #[arg(short = 'c', long = "cpu", value_name = "threads")]
    cpu: Option<u8>,

    #[cfg(feature = "opencl")]
    #[arg(short = 'g', long = "gpu", value_name = "platform_id:device_id:cores", num_args=1..)]
    gpu: Vec<String>,
    #[cfg(feature = "opencl")]
    #[arg(short = 'o', long = "opencl", help = "Display OpenCL platforms and devices")]
    ocl_devices: bool,
    #[cfg(feature = "opencl")]
    #[arg(short = 'z', long = "zcb", help = "Enables zero copy buffers for shared mem (integrated) gpus")]
    zero_copy: bool,
}

fn main() {
    let args = Args::parse();

    if args.low_priority {
        set_low_prio();
    }

    #[cfg(feature = "opencl")]
    if args.ocl_devices {
        ocl::platform_info();
        return;
    }

    let numeric_id = args.numeric_id.expect("numeric id required");
    let start_nonce = args.start_nonce.expect("start nonce required");
    let nonces = args.nonces.expect("nonces required");
    let output_path = args.path.unwrap_or_else(|| {
        std::env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    });
    let mem = args.memory.unwrap_or_else(|| "0B".to_owned());
    let mut cpu_threads = args.cpu.unwrap_or(0u8);

    let gpus = {
        #[cfg(feature = "opencl")]
        {
            if !args.gpu.is_empty() {
                Some(args.gpu.clone())
            } else {
                None
            }
        }
        #[cfg(not(feature = "opencl"))]
        {
            None
        }
    };

    let cores = sys_info::cpu_num().unwrap() as u8;
    cpu_threads = if cpu_threads == 0 { cores } else { std::cmp::min(2 * cores, cpu_threads) };

    #[cfg(feature = "opencl")]
    if !args.gpu.is_empty() && args.cpu.is_none() {
        cpu_threads = 0;
    }

    let p = Plotter::new();
    p.run(PlotterTask {
        numeric_id,
        start_nonce,
        nonces,
        output_path,
        mem,
        cpu_threads,
        gpus,
        direct_io: !args.disable_direct_io,
        async_io: !args.disable_async_io,
        quiet: args.quiet,
        benchmark: args.benchmark,
        #[cfg(feature = "opencl")]
        zcb: args.zero_copy,
        #[cfg(not(feature = "opencl"))]
        zcb: false,
    });
}
