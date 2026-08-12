# BarsSince benchmark (`bars since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 166.84M | 0.005 | 194.83M | 0.124 | 20.69× | 24.16× |
| 10,000 | 0.030 | 328.68M | 0.028 | 362.21M | 1.134 | 37.28× | 41.08× |
| 100,000 | 0.317 | 315.45M | 0.239 | 419.17M | 15.378 | 48.51× | 64.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.070 | 0.57× |
| 1 | 5 | 0.239 | 0.333 | 1.39× |
| 1 | 10 | 0.500 | 0.697 | 1.39× |
| 10 | 1 | 0.066 | 0.075 | 1.13× |
| 10 | 5 | 0.264 | 0.313 | 1.19× |
| 10 | 10 | 0.504 | 0.685 | 1.36× |
| 100 | 1 | 0.056 | 0.081 | 1.45× |
| 100 | 5 | 0.285 | 0.415 | 1.46× |
| 100 | 10 | 0.571 | 0.851 | 1.49× |
| 1,000 | 1 | 0.055 | 0.172 | 3.15× |
| 1,000 | 5 | 0.298 | 0.945 | 3.17× |
| 1,000 | 10 | 0.583 | 1.965 | 3.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
