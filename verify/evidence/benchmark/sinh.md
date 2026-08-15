# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.42M | 0.007 | 141.90M | 0.033 | 3.96× | 4.67× |
| 10,000 | 0.067 | 149.63M | 0.064 | 157.25M | 0.092 | 1.38× | 1.45× |
| 100,000 | 0.644 | 155.17M | 0.620 | 161.23M | 0.663 | 1.03× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.157 | 1.82× |
| 1 | 5 | 0.240 | 0.463 | 1.93× |
| 1 | 10 | 0.391 | 0.896 | 2.29× |
| 10 | 1 | 0.040 | 0.082 | 2.05× |
| 10 | 5 | 0.183 | 0.419 | 2.29× |
| 10 | 10 | 0.421 | 0.886 | 2.11× |
| 100 | 1 | 0.040 | 0.089 | 2.21× |
| 100 | 5 | 0.178 | 0.412 | 2.31× |
| 100 | 10 | 0.373 | 0.898 | 2.41× |
| 1,000 | 1 | 0.046 | 0.092 | 2.00× |
| 1,000 | 5 | 0.192 | 0.457 | 2.38× |
| 1,000 | 10 | 0.437 | 1.036 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
