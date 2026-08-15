# RollingMaximumIndex benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.97M | 0.006 | 168.91M | 0.039 | 5.37× | 6.67× |
| 10,000 | 0.058 | 172.04M | 0.054 | 186.43M | 0.143 | 2.47× | 2.67× |
| 100,000 | 0.612 | 163.32M | 0.534 | 187.39M | 0.754 | 1.23× | 1.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.122 | 1.53× |
| 1 | 5 | 0.311 | 0.459 | 1.48× |
| 1 | 10 | 0.371 | 0.882 | 2.38× |
| 10 | 1 | 0.042 | 0.088 | 2.08× |
| 10 | 5 | 0.197 | 0.499 | 2.54× |
| 10 | 10 | 0.386 | 0.891 | 2.31× |
| 100 | 1 | 0.041 | 0.093 | 2.27× |
| 100 | 5 | 0.175 | 0.415 | 2.37× |
| 100 | 10 | 0.431 | 0.943 | 2.19× |
| 1,000 | 1 | 0.052 | 0.100 | 1.91× |
| 1,000 | 5 | 0.195 | 0.467 | 2.39× |
| 1,000 | 10 | 0.437 | 1.044 | 2.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
