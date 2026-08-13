# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.169 | 5.92M | 0.188 | 5.32M | 0.044 | 0.26× | 0.23× |
| 10,000 | 1.681 | 5.95M | 1.709 | 5.85M | 0.152 | 0.09× | 0.09× |
| 100,000 | 17.655 | 5.66M | 17.346 | 5.77M | 1.236 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.123 | 0.96× |
| 1 | 5 | 0.434 | 0.479 | 1.10× |
| 1 | 10 | 0.579 | 0.935 | 1.61× |
| 10 | 1 | 0.068 | 0.095 | 1.39× |
| 10 | 5 | 0.291 | 0.442 | 1.52× |
| 10 | 10 | 0.582 | 0.914 | 1.57× |
| 100 | 1 | 0.082 | 0.095 | 1.16× |
| 100 | 5 | 0.310 | 0.454 | 1.46× |
| 100 | 10 | 0.623 | 0.911 | 1.46× |
| 1,000 | 1 | 0.238 | 0.112 | 0.47× |
| 1,000 | 5 | 0.444 | 0.507 | 1.14× |
| 1,000 | 10 | 0.760 | 1.088 | 1.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
