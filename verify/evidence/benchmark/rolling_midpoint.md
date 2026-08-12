# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.01M | 0.007 | 138.80M | 0.045 | 5.37× | 6.21× |
| 10,000 | 0.104 | 96.01M | 0.077 | 129.87M | 0.104 | 1.00× | 1.35× |
| 100,000 | 0.792 | 126.34M | 0.746 | 133.97M | 0.680 | 0.86× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.134 | 0.99× |
| 1 | 5 | 0.359 | 0.498 | 1.39× |
| 1 | 10 | 0.490 | 0.937 | 1.91× |
| 10 | 1 | 0.052 | 0.088 | 1.68× |
| 10 | 5 | 0.223 | 0.435 | 1.95× |
| 10 | 10 | 0.469 | 0.985 | 2.10× |
| 100 | 1 | 0.053 | 0.094 | 1.77× |
| 100 | 5 | 0.230 | 0.444 | 1.93× |
| 100 | 10 | 0.488 | 0.920 | 1.89× |
| 1,000 | 1 | 0.056 | 0.097 | 1.71× |
| 1,000 | 5 | 0.254 | 0.537 | 2.12× |
| 1,000 | 10 | 0.529 | 1.024 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
