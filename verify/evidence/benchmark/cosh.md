# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.36M | 0.006 | 154.60M | 0.036 | 4.65× | 5.60× |
| 10,000 | 0.062 | 161.36M | 0.056 | 180.01M | 0.086 | 1.39× | 1.55× |
| 100,000 | 0.590 | 169.61M | 0.541 | 184.79M | 0.629 | 1.07× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.101 | 1.63× |
| 1 | 5 | 0.196 | 0.431 | 2.19× |
| 1 | 10 | 0.378 | 0.924 | 2.44× |
| 10 | 1 | 0.041 | 0.082 | 2.02× |
| 10 | 5 | 0.175 | 0.411 | 2.35× |
| 10 | 10 | 0.409 | 0.897 | 2.20× |
| 100 | 1 | 0.044 | 0.093 | 2.10× |
| 100 | 5 | 0.214 | 0.442 | 2.06× |
| 100 | 10 | 0.375 | 0.860 | 2.29× |
| 1,000 | 1 | 0.055 | 0.095 | 1.74× |
| 1,000 | 5 | 0.201 | 0.444 | 2.21× |
| 1,000 | 10 | 0.461 | 0.956 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
