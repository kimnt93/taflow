# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.142 | 7.04M | 0.140 | 7.13M | 0.511 | 3.60× | 3.64× |
| 10,000 | 1.482 | 6.75M | 1.454 | 6.88M | 5.067 | 3.42× | 3.48× |
| 100,000 | 15.736 | 6.35M | 14.873 | 6.72M | 48.624 | 3.09× | 3.27× |
| 1,000,000 | 150.633 | 6.64M | 146.320 | 6.83M | 497.062 | 3.30× | 3.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.119 | 0.96× |
| 1 | 5 | 0.283 | 0.473 | 1.67× |
| 1 | 10 | 0.458 | 1.030 | 2.25× |
| 10 | 1 | 0.049 | 0.099 | 2.04× |
| 10 | 5 | 0.243 | 0.457 | 1.88× |
| 10 | 10 | 0.505 | 0.947 | 1.87× |
| 100 | 1 | 0.056 | 0.119 | 2.13× |
| 100 | 5 | 0.254 | 0.635 | 2.50× |
| 100 | 10 | 0.494 | 1.249 | 2.53× |
| 1,000 | 1 | 0.193 | 0.557 | 2.89× |
| 1,000 | 5 | 0.385 | 2.834 | 7.36× |
| 1,000 | 10 | 0.580 | 5.881 | 10.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
