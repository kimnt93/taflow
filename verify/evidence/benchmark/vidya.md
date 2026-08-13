# VariableIndexDynamicAverage benchmark (`VIDYA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.21M | 0.047 | 21.44M | 0.205 | 3.73× | 4.39× |
| 10,000 | 0.382 | 26.15M | 0.375 | 26.70M | 0.526 | 1.38× | 1.41× |
| 100,000 | 3.983 | 25.10M | 3.907 | 25.59M | 3.923 | 0.98× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.151 | 0.286 | 1.90× |
| 1 | 5 | 0.421 | 1.529 | 3.63× |
| 1 | 10 | 0.620 | 2.810 | 4.53× |
| 10 | 1 | 0.075 | 0.252 | 3.36× |
| 10 | 5 | 0.282 | 1.590 | 5.64× |
| 10 | 10 | 0.614 | 2.623 | 4.27× |
| 100 | 1 | 0.074 | 0.251 | 3.38× |
| 100 | 5 | 0.298 | 1.517 | 5.09× |
| 100 | 10 | 0.590 | 2.860 | 4.85× |
| 1,000 | 1 | 0.115 | 0.288 | 2.52× |
| 1,000 | 5 | 0.282 | 1.681 | 5.95× |
| 1,000 | 10 | 0.606 | 3.004 | 4.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
