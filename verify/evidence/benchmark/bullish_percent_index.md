# BullishPercentIndex benchmark (`BullishPercentIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 208.37M | 0.003 | 287.32M | 11.506 | 2397.42× | 3305.77× |
| 10,000 | 0.029 | 346.53M | 0.026 | 387.97M | 114.863 | 3980.37× | 4456.33× |
| 100,000 | 0.253 | 395.74M | 0.231 | 433.50M | 1121.411 | 4437.84× | 4861.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.347 | 3.23× |
| 1 | 5 | 0.254 | 1.045 | 4.12× |
| 1 | 10 | 0.394 | 2.229 | 5.66× |
| 10 | 1 | 0.049 | 0.314 | 6.40× |
| 10 | 5 | 0.187 | 1.840 | 9.84× |
| 10 | 10 | 0.445 | 3.239 | 7.28× |
| 100 | 1 | 0.044 | 1.357 | 30.71× |
| 100 | 5 | 0.213 | 7.194 | 33.70× |
| 100 | 10 | 0.387 | 14.272 | 36.88× |
| 1,000 | 1 | 0.051 | 12.394 | 240.81× |
| 1,000 | 5 | 0.352 | 58.240 | 165.37× |
| 1,000 | 10 | 0.526 | 118.654 | 225.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
