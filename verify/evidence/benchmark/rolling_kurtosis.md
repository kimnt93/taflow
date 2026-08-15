# RollingKurtosis benchmark (`Kurtosis` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.81M | 0.029 | 34.77M | 0.174 | 6.06× | 6.06× |
| 10,000 | 0.285 | 35.11M | 0.272 | 36.75M | 0.534 | 1.87× | 1.96× |
| 100,000 | 2.882 | 34.69M | 2.743 | 36.46M | 4.801 | 1.67× | 1.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.245 | 3.36× |
| 1 | 5 | 0.223 | 1.224 | 5.49× |
| 1 | 10 | 0.412 | 2.245 | 5.44× |
| 10 | 1 | 0.048 | 0.214 | 4.50× |
| 10 | 5 | 0.189 | 1.286 | 6.80× |
| 10 | 10 | 0.407 | 2.258 | 5.55× |
| 100 | 1 | 0.045 | 0.232 | 5.09× |
| 100 | 5 | 0.211 | 1.257 | 5.96× |
| 100 | 10 | 0.424 | 2.352 | 5.55× |
| 1,000 | 1 | 0.075 | 0.250 | 3.31× |
| 1,000 | 5 | 0.221 | 1.503 | 6.79× |
| 1,000 | 10 | 0.485 | 2.659 | 5.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
