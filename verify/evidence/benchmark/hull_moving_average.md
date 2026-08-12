# HullMovingAverage benchmark (`HMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 39.45M | 0.025 | 40.33M | 0.165 | 6.52× | 6.67× |
| 10,000 | 0.218 | 45.98M | 0.223 | 44.79M | 0.556 | 2.56× | 2.49× |
| 100,000 | 2.112 | 47.35M | 2.075 | 48.19M | 4.456 | 2.11× | 2.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.274 | 2.08× |
| 1 | 5 | 0.314 | 0.974 | 3.10× |
| 1 | 10 | 0.506 | 2.194 | 4.33× |
| 10 | 1 | 0.050 | 0.192 | 3.80× |
| 10 | 5 | 0.261 | 0.984 | 3.77× |
| 10 | 10 | 0.527 | 2.149 | 4.07× |
| 100 | 1 | 0.052 | 0.199 | 3.84× |
| 100 | 5 | 0.238 | 1.021 | 4.29× |
| 100 | 10 | 0.548 | 2.170 | 3.96× |
| 1,000 | 1 | 0.080 | 0.245 | 3.04× |
| 1,000 | 5 | 0.268 | 1.200 | 4.47× |
| 1,000 | 10 | 0.513 | 2.595 | 5.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
