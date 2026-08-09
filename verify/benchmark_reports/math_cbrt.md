# MathCbrt benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 44.10M | 0.021 | 47.08M | 0.017 | 0.75× | 0.80× |
| 10,000 | 0.193 | 51.75M | 0.183 | 54.51M | 0.160 | 0.83× | 0.87× |
| 100,000 | 2.010 | 49.75M | 1.913 | 52.26M | 1.604 | 0.80× | 0.84× |
| 1,000,000 | 20.151 | 49.62M | 19.919 | 50.20M | 14.405 | 0.71× | 0.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.056 | 0.67× |
| 1 | 5 | 0.346 | 0.241 | 0.70× |
| 1 | 10 | 0.491 | 0.409 | 0.83× |
| 10 | 1 | 0.047 | 0.042 | 0.90× |
| 10 | 5 | 0.211 | 0.190 | 0.90× |
| 10 | 10 | 0.462 | 0.409 | 0.88× |
| 100 | 1 | 0.049 | 0.045 | 0.92× |
| 100 | 5 | 0.235 | 0.199 | 0.84× |
| 100 | 10 | 0.509 | 0.400 | 0.79× |
| 1,000 | 1 | 0.066 | 0.059 | 0.88× |
| 1,000 | 5 | 0.228 | 0.207 | 0.91× |
| 1,000 | 10 | 0.500 | 0.466 | 0.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
