# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 60.84M | 0.016 | 62.98M | 0.044 | 2.70× | 2.79× |
| 10,000 | 0.203 | 49.28M | 0.203 | 49.33M | 0.229 | 1.13× | 1.13× |
| 100,000 | 2.165 | 46.20M | 2.160 | 46.29M | 1.974 | 0.91× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.116 | 0.93× |
| 1 | 5 | 0.384 | 0.485 | 1.26× |
| 1 | 10 | 0.430 | 0.907 | 2.11× |
| 10 | 1 | 0.041 | 0.084 | 2.04× |
| 10 | 5 | 0.179 | 0.402 | 2.24× |
| 10 | 10 | 0.403 | 0.980 | 2.43× |
| 100 | 1 | 0.049 | 0.092 | 1.88× |
| 100 | 5 | 0.194 | 0.442 | 2.28× |
| 100 | 10 | 0.424 | 0.866 | 2.04× |
| 1,000 | 1 | 0.073 | 0.109 | 1.49× |
| 1,000 | 5 | 0.213 | 0.543 | 2.55× |
| 1,000 | 10 | 0.414 | 1.101 | 2.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
