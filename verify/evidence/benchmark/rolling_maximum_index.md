# RollingMaximumIndex benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.40M | 0.056 | 17.84M | 0.035 | 0.58× | 0.63× |
| 10,000 | 0.511 | 19.58M | 0.513 | 19.49M | 0.093 | 0.18× | 0.18× |
| 100,000 | 4.829 | 20.71M | 4.921 | 20.32M | 0.690 | 0.14× | 0.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.180 | 0.108 | 0.60× |
| 1 | 5 | 0.366 | 0.447 | 1.22× |
| 1 | 10 | 0.577 | 0.921 | 1.60× |
| 10 | 1 | 0.064 | 0.088 | 1.36× |
| 10 | 5 | 0.307 | 0.431 | 1.40× |
| 10 | 10 | 0.609 | 0.915 | 1.50× |
| 100 | 1 | 0.068 | 0.099 | 1.45× |
| 100 | 5 | 0.317 | 0.440 | 1.39× |
| 100 | 10 | 0.609 | 0.899 | 1.48× |
| 1,000 | 1 | 0.124 | 0.098 | 0.79× |
| 1,000 | 5 | 0.307 | 0.463 | 1.51× |
| 1,000 | 10 | 0.628 | 0.979 | 1.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
