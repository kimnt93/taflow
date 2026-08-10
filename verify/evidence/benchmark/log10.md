# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.82M | 0.010 | 100.19M | 0.035 | 3.18× | 3.51× |
| 10,000 | 0.086 | 115.72M | 0.084 | 118.60M | 0.180 | 2.08× | 2.13× |
| 100,000 | 1.541 | 64.90M | 0.935 | 106.94M | 0.858 | 0.56× | 0.92× |
| 1,000,000 | 8.571 | 116.67M | 7.926 | 126.16M | 7.824 | 0.91× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.114 | 1.57× |
| 1 | 5 | 0.237 | 0.437 | 1.84× |
| 1 | 10 | 0.486 | 0.913 | 1.88× |
| 10 | 1 | 0.045 | 0.083 | 1.84× |
| 10 | 5 | 0.218 | 0.428 | 1.96× |
| 10 | 10 | 0.458 | 0.915 | 2.00× |
| 100 | 1 | 0.067 | 0.111 | 1.66× |
| 100 | 5 | 0.278 | 0.464 | 1.67× |
| 100 | 10 | 0.489 | 0.909 | 1.86× |
| 1,000 | 1 | 0.064 | 0.101 | 1.57× |
| 1,000 | 5 | 0.248 | 0.479 | 1.93× |
| 1,000 | 10 | 0.505 | 0.937 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
