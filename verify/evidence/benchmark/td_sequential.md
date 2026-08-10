# TomDeMarkSequential benchmark (`TDSequential` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.69M | 0.009 | 113.52M | 0.647 | 67.09× | 73.46× |
| 10,000 | 0.079 | 126.94M | 0.074 | 135.65M | 5.618 | 71.32× | 76.21× |
| 100,000 | 0.758 | 131.89M | 0.732 | 136.68M | 45.188 | 59.60× | 61.76× |
| 1,000,000 | 8.255 | 121.13M | 7.160 | 139.67M | 508.902 | 61.65× | 71.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.308 | 3.27× |
| 1 | 5 | 0.322 | 1.622 | 5.03× |
| 1 | 10 | 0.484 | 3.160 | 6.53× |
| 10 | 1 | 0.051 | 0.268 | 5.23× |
| 10 | 5 | 0.222 | 1.480 | 6.66× |
| 10 | 10 | 0.443 | 3.126 | 7.06× |
| 100 | 1 | 0.066 | 0.323 | 4.88× |
| 100 | 5 | 0.245 | 1.692 | 6.92× |
| 100 | 10 | 0.433 | 3.538 | 8.17× |
| 1,000 | 1 | 0.060 | 0.824 | 13.62× |
| 1,000 | 5 | 0.262 | 4.120 | 15.73× |
| 1,000 | 10 | 0.511 | 8.404 | 16.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
