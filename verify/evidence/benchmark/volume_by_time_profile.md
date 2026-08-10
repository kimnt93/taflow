# VolumeByTimeProfile benchmark (`VolumeByTimeProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.31M | 0.053 | 18.74M | 1.615 | 26.34× | 30.26× |
| 10,000 | 0.543 | 18.40M | 0.481 | 20.78M | 14.940 | 27.49× | 31.04× |
| 100,000 | 6.371 | 15.70M | 4.739 | 21.10M | 169.117 | 26.54× | 35.69× |
| 1,000,000 | 159.741 | 6.26M | 94.198 | 10.62M | 1876.189 | 11.75× | 19.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.317 | 3.44× |
| 1 | 5 | 0.403 | 1.528 | 3.79× |
| 1 | 10 | 0.572 | 2.835 | 4.96× |
| 10 | 1 | 0.079 | 0.302 | 3.81× |
| 10 | 5 | 0.283 | 1.585 | 5.60× |
| 10 | 10 | 0.563 | 3.086 | 5.48× |
| 100 | 1 | 0.070 | 0.426 | 6.10× |
| 100 | 5 | 0.287 | 2.245 | 7.82× |
| 100 | 10 | 0.583 | 4.293 | 7.36× |
| 1,000 | 1 | 0.113 | 2.037 | 18.02× |
| 1,000 | 5 | 0.305 | 9.379 | 30.72× |
| 1,000 | 10 | 0.664 | 19.396 | 29.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
