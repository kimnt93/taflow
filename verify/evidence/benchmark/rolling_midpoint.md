# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 160.76M | 0.005 | 192.64M | 0.036 | 5.77× | 6.92× |
| 10,000 | 0.052 | 190.68M | 0.044 | 228.42M | 0.095 | 1.82× | 2.17× |
| 100,000 | 0.440 | 227.39M | 0.413 | 242.06M | 0.782 | 1.78× | 1.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.053 | 0.108 | 2.06× |
| 1 | 5 | 0.235 | 0.469 | 1.99× |
| 1 | 10 | 0.415 | 0.974 | 2.35× |
| 10 | 1 | 0.050 | 0.097 | 1.96× |
| 10 | 5 | 0.190 | 0.437 | 2.30× |
| 10 | 10 | 0.417 | 0.969 | 2.32× |
| 100 | 1 | 0.045 | 0.088 | 1.95× |
| 100 | 5 | 0.193 | 0.440 | 2.28× |
| 100 | 10 | 0.401 | 0.933 | 2.33× |
| 1,000 | 1 | 0.048 | 0.095 | 1.96× |
| 1,000 | 5 | 0.205 | 0.508 | 2.48× |
| 1,000 | 10 | 0.410 | 0.946 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
