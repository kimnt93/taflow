# HullMovingAverage benchmark (`HMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.27M | 0.022 | 44.50M | 0.149 | 6.31× | 6.65× |
| 10,000 | 0.220 | 45.50M | 0.199 | 50.13M | 0.518 | 2.36× | 2.60× |
| 100,000 | 1.974 | 50.66M | 2.030 | 49.25M | 4.193 | 2.12× | 2.07× |
| 1,000,000 | 21.484 | 46.55M | 19.947 | 50.13M | 41.594 | 1.94× | 2.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.218 | 3.50× |
| 1 | 5 | 0.284 | 1.022 | 3.60× |
| 1 | 10 | 0.493 | 2.105 | 4.27× |
| 10 | 1 | 0.056 | 0.198 | 3.57× |
| 10 | 5 | 0.235 | 0.996 | 4.24× |
| 10 | 10 | 0.500 | 2.076 | 4.15× |
| 100 | 1 | 0.056 | 0.192 | 3.43× |
| 100 | 5 | 0.242 | 0.955 | 3.94× |
| 100 | 10 | 0.489 | 2.085 | 4.26× |
| 1,000 | 1 | 0.068 | 0.238 | 3.48× |
| 1,000 | 5 | 0.240 | 1.159 | 4.84× |
| 1,000 | 10 | 0.512 | 2.485 | 4.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
