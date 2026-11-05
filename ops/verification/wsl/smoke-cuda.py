#!/usr/bin/env python3
import ctypes as c
import json
import math
import sys

PTX = b""".version 7.0
.target sm_50
.address_size 64

.visible .entry dense_relu(
    .param .u64 input_ptr,
    .param .u64 output_ptr
) {
    .reg .b64 %rd<5>;
    .reg .f32 %f<8>;
    .reg .u32 %r0;

    ld.param.u64 %rd1, [input_ptr];
    ld.param.u64 %rd2, [output_ptr];
    mov.u32 %r0, %tid.x;

    // Load x[0], x[1], x[2]
    ld.global.f32 %f0, [%rd1];
    ld.global.f32 %f1, [%rd1+4];
    ld.global.f32 %f2, [%rd1+8];

    // Branch on thread ID (%r0)
    setp.eq.u32 %p0, %r0, 0;
    @%p0 bra ROW0;
    setp.eq.u32 %p1, %r0, 1;
    @%p1 bra ROW1;
    setp.eq.u32 %p2, %r0, 2;
    @%p2 bra ROW2;
    bra END;

ROW0:
    // row 0: [0.5, -0.2, 0.1, 0.0]
    mul.f32 %f3, %f0, 0f3F000000; // 0.5
    fma.rn.f32 %f4, %f1, 0fBE4CCCCD, %f3; // -0.2
    fma.rn.f32 %f5, %f2, 0f3DCCCCCD, %f4; // 0.1
    // bias = 0.0
    // ReLU: max(0.0, sum)
    max.f32 %f6, %f5, 0f00000000;
    st.global.f32 [%rd2], %f6;
    bra END;

ROW1:
    // row 1: [-0.1, 0.4, 0.2, 0.1]
    mul.f32 %f3, %f0, 0fBDCCCCCD; // -0.1
    fma.rn.f32 %f4, %f1, 0f3ECCCCCD, %f3; // 0.4
    fma.rn.f32 %f5, %f2, 0f3E4CCCCD, %f4; // 0.2
    add.f32 %f5, %f5, 0f3DCCCCCD; // bias 0.1
    // ReLU: max(0.0, sum)
    max.f32 %f6, %f5, 0f00000000;
    st.global.f32 [%rd2+4], %f6;
    bra END;

ROW2:
    // row 2: [0.3, 0.1, -0.5, 0.2]
    mul.f32 %f3, %f0, 0f3E99999A; // 0.3
    fma.rn.f32 %f4, %f1, 0f3DCCCCCD, %f3; // 0.1
    fma.rn.f32 %f5, %f2, 0fBF000000, %f4; // -0.5
    add.f32 %f5, %f5, 0f3E4CCCCD; // bias 0.2
    // ReLU: max(0.0, sum)
    max.f32 %f6, %f5, 0f00000000;
    st.global.f32 [%rd2+8], %f6;
    bra END;

END:
    ret;
}
"""

def main():
    try:
        cuda = c.CDLL("libcuda.so.1")
    except OSError:
        print(json.dumps({
            "status": "SKIP",
            "reason": "libcuda.so.1 not available in current environment",
            "backend": "cpu-simulation"
        }))
        return

    def call(func_name, *args):
        fn = getattr(cuda, func_name)
        res = fn(*args)
        if res != 0:
            raise RuntimeError(f"{func_name} failed with code {res}")

    device = c.c_int()
    version = c.c_int()
    name = c.create_string_buffer(256)

    context, module, kernel = c.c_void_p(), c.c_void_p(), c.c_void_p()
    src, dst = c.c_uint64(), c.c_uint64()

    x = [1.0, 2.0, 3.0]
    rows = [
        [0.5, -0.2, 0.1, 0.0],
        [-0.1, 0.4, 0.2, 0.1],
        [0.3, 0.1, -0.5, 0.2],
    ]
    # CPU reference: ReLU(W * x + b)
    expected = []
    for r in rows:
        dot = sum(w * val for w, val in zip(r[:3], x)) + r[3]
        expected.append(max(0.0, dot))

    inputs = (c.c_float * len(x))(*x)
    output = (c.c_float * len(rows))()

    call("cuInit", 0)
    call("cuDeviceGet", c.byref(device), 0)
    call("cuDriverGetVersion", c.byref(version))
    call("cuDeviceGetName", name, len(name), device)
    call("cuCtxCreate_v2", c.byref(context), 0, device)
    try:
        call("cuModuleLoadData", c.byref(module), c.c_char_p(PTX))
        call("cuModuleGetFunction", c.byref(kernel), module, b"dense_relu")
        call("cuMemAlloc_v2", c.byref(src), c.c_size_t(c.sizeof(inputs)))
        call("cuMemAlloc_v2", c.byref(dst), c.c_size_t(c.sizeof(output)))
        call("cuMemcpyHtoD_v2", src, inputs, c.c_size_t(c.sizeof(inputs)))
        params = (c.c_void_p * 2)(c.addressof(src), c.addressof(dst))
        call("cuLaunchKernel", kernel, 1, 1, 1, len(rows), 1, 1, 0, None, params, None)
        call("cuCtxSynchronize")
        call("cuMemcpyDtoH_v2", output, dst, c.c_size_t(c.sizeof(output)))
    finally:
        call("cuCtxDestroy_v2", context)

    actual = list(output)
    if not all(math.isclose(a, b, abs_tol=1e-6) for a, b in zip(actual, expected, strict=True)):
        raise RuntimeError(f"CUDA result differs from the CPU reference: {actual} vs {expected}")
    print(json.dumps({
        "status": "PASS",
        "backend": "wsl2-cuda-driver",
        "gpu": name.value.decode(),
        "cuda_driver_api": version.value,
        "kernel": "dense_relu",
        "input": x,
        "weights_and_bias": rows,
        "output": actual,
        "cpu_reference": expected,
    }))

if __name__ == "__main__":
    main()
