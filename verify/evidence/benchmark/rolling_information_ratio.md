# RollingInformationRatio benchmark (`InformationRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.24M | 0.032 | 31.37M | 0.197 | 5.96× | 6.19× |
| 10,000 | 0.319 | 31.37M | 0.310 | 32.29M | 0.800 | 2.51× | 2.58× |
| 100,000 | 3.214 | 31.11M | 3.376 | 29.62M | 7.199 | 2.24× | 2.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.326 | 5.16× |
| 1 | 5 | 0.229 | 1.222 | 5.34× |
| 1 | 10 | 0.406 | 2.340 | 5.76× |
| 10 | 1 | 0.049 | 0.192 | 3.96× |
| 10 | 5 | 0.203 | 1.001 | 4.92× |
| 10 | 10 | 0.452 | 2.747 | 6.07× |
| 100 | 1 | 0.049 | 0.206 | 4.20× |
| 100 | 5 | 0.248 | 1.065 | 4.29× |
| 100 | 10 | 0.429 | 2.324 | 5.41× |
| 1,000 | 1 | 0.081 | 0.275 | 3.40× |
| 1,000 | 5 | 0.225 | 1.327 | 5.89× |
| 1,000 | 10 | 0.435 | 2.987 | 6.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
