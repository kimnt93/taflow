# HilbertDominantCycle benchmark (`HilbertDominantCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.062 | 16.00M | 0.061 | 16.40M | 0.189 | 3.02× | 3.09× |
| 10,000 | 0.599 | 16.68M | 0.586 | 17.06M | 1.002 | 1.67× | 1.71× |
| 100,000 | 5.990 | 16.69M | 5.882 | 17.00M | 9.168 | 1.53× | 1.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.222 | 2.66× |
| 1 | 5 | 0.297 | 1.114 | 3.75× |
| 1 | 10 | 0.511 | 1.846 | 3.61× |
| 10 | 1 | 0.052 | 0.163 | 3.14× |
| 10 | 5 | 0.232 | 0.805 | 3.47× |
| 10 | 10 | 0.526 | 1.894 | 3.60× |
| 100 | 1 | 0.058 | 0.188 | 3.26× |
| 100 | 5 | 0.241 | 0.949 | 3.93× |
| 100 | 10 | 0.529 | 2.040 | 3.86× |
| 1,000 | 1 | 0.115 | 0.264 | 2.30× |
| 1,000 | 5 | 0.280 | 1.325 | 4.74× |
| 1,000 | 10 | 0.555 | 2.625 | 4.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
