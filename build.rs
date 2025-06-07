extern crate cc;

fn main() {
    let mut shared_config = cc::Build::new();

    #[cfg(target_env = "msvc")]
    shared_config
        .flag("/O2")
        .flag("/Oi")
        .flag("/Ot")
        .flag("/Oy")
        .flag("/GT")
        .flag("/GL");

    #[cfg(not(target_env = "msvc"))]
    shared_config.flag("-std=c99").flag("-mtune=native");

    let mut config = shared_config.clone();

    #[cfg(target_env = "msvc")]
    {
        config.flag("/arch:AVX512");
    }

    #[cfg(not(target_env = "msvc"))]
    {
        config.flag("-msse2");
        config.flag("-mavx2");
        config.flag("-mavx");
        config.flag("-mavx512f");
    }

    config
        .file("src/c/sph_shabal.c")
        .file("src/c/common.c")
        .file("src/c/mshabal_128_sse2.c")
        .file("src/c/noncegen_128_sse2.c")
        .file("src/c/mshabal_256_avx2.c")
        .file("src/c/noncegen_256_avx2.c")
        .file("src/c/mshabal_128_avx.c")
        .file("src/c/noncegen_128_avx.c")
        .file("src/c/mshabal_512_avx512f.c")
        .file("src/c/noncegen_512_avx512f.c")
        .compile("shabal");
}
