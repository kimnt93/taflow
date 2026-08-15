# RollingMinimumIndex benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 169.98M | 0.005 | 203.63M | 0.039 | 6.71× | 8.04× |
| 10,000 | 0.053 | 189.78M | 0.049 | 206.18M | 0.096 | 1.83× | 1.99× |
| 100,000 | 0.525 | 190.38M | 0.518 | 193.01M | 0.703 | 1.34× | 1.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.137 | 1.31× |
| 1 | 5 | 0.225 | 0.447 | 1.98× |
| 1 | 10 | 0.380 | 0.916 | 2.41× |
| 10 | 1 | 0.050 | 0.097 | 1.95× |
| 10 | 5 | 0.204 | 0.446 | 2.19× |
| 10 | 10 | 0.395 | 0.928 | 2.35× |
| 100 | 1 | 0.041 | 0.093 | 2.29× |
| 100 | 5 | 0.186 | 0.451 | 2.43× |
| 100 | 10 | 0.430 | 0.926 | 2.15× |
| 1,000 | 1 | 0.046 | 0.099 | 2.14× |
| 1,000 | 5 | 0.201 | 0.460 | 2.29× |
| 1,000 | 10 | 0.411 | 1.030 | 2.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
