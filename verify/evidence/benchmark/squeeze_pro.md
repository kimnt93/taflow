# SqueezePro benchmark (`squeeze_pro` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.64M | 0.035 | 28.36M | 8.602 | 237.73× | 243.94× |
| 10,000 | 0.301 | 33.18M | 0.327 | 30.61M | 12.169 | 40.37× | 37.25× |
| 100,000 | 3.157 | 31.68M | 3.253 | 30.74M | 52.679 | 16.69× | 16.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.144 | 0.432 | 3.00× |
| 1 | 5 | 0.344 | 1.938 | 5.64× |
| 1 | 10 | 0.426 | 3.530 | 8.28× |
| 10 | 1 | 0.055 | 0.375 | 6.88× |
| 10 | 5 | 0.243 | 1.794 | 7.38× |
| 10 | 10 | 0.446 | 3.709 | 8.32× |
| 100 | 1 | 0.055 | 8.643 | 157.95× |
| 100 | 5 | 0.357 | 44.722 | 125.42× |
| 100 | 10 | 0.493 | 91.375 | 185.30× |
| 1,000 | 1 | 0.087 | 9.118 | 104.55× |
| 1,000 | 5 | 0.303 | 49.061 | 161.92× |
| 1,000 | 10 | 0.584 | 99.230 | 170.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
