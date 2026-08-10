# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 166.43M | 0.005 | 188.96M | 0.037 | 6.24× | 7.08× |
| 10,000 | 0.024 | 415.00M | 0.026 | 382.03M | 0.047 | 1.94× | 1.79× |
| 100,000 | 0.238 | 419.82M | 0.188 | 532.93M | 0.071 | 0.30× | 0.38× |
| 1,000,000 | 2.798 | 357.38M | 2.129 | 469.65M | 1.128 | 0.40× | 0.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.119 | 1.84× |
| 1 | 5 | 0.294 | 0.542 | 1.84× |
| 1 | 10 | 0.578 | 1.092 | 1.89× |
| 10 | 1 | 0.050 | 0.089 | 1.79× |
| 10 | 5 | 0.259 | 0.547 | 2.11× |
| 10 | 10 | 0.650 | 1.104 | 1.70× |
| 100 | 1 | 0.050 | 0.089 | 1.78× |
| 100 | 5 | 0.269 | 0.494 | 1.84× |
| 100 | 10 | 0.655 | 1.073 | 1.64× |
| 1,000 | 1 | 0.050 | 0.098 | 1.95× |
| 1,000 | 5 | 0.294 | 0.465 | 1.58× |
| 1,000 | 10 | 0.547 | 1.224 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
