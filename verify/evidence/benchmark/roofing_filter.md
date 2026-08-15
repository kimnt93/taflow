# RoofingFilter benchmark (`RoofingFilter` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 156.30M | 0.005 | 184.95M | 0.188 | 29.42× | 34.81× |
| 10,000 | 0.047 | 212.10M | 0.045 | 223.65M | 0.519 | 11.01× | 11.61× |
| 100,000 | 0.420 | 238.36M | 0.394 | 253.58M | 3.548 | 8.46× | 9.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.283 | 2.75× |
| 1 | 5 | 0.249 | 1.387 | 5.58× |
| 1 | 10 | 0.391 | 2.624 | 6.72× |
| 10 | 1 | 0.048 | 0.264 | 5.46× |
| 10 | 5 | 0.188 | 1.403 | 7.46× |
| 10 | 10 | 0.413 | 2.569 | 6.23× |
| 100 | 1 | 0.048 | 0.233 | 4.82× |
| 100 | 5 | 0.190 | 1.353 | 7.13× |
| 100 | 10 | 0.387 | 2.683 | 6.93× |
| 1,000 | 1 | 0.053 | 0.273 | 5.17× |
| 1,000 | 5 | 0.210 | 1.564 | 7.46× |
| 1,000 | 10 | 0.446 | 2.868 | 6.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
