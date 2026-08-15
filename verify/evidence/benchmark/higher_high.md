# HigherHigh benchmark (`higher high relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 198.39M | 0.004 | 247.41M | 0.016 | 3.24× | 4.05× |
| 10,000 | 0.031 | 321.04M | 0.028 | 362.28M | 0.023 | 0.74× | 0.84× |
| 100,000 | 0.268 | 372.78M | 0.244 | 410.01M | 0.100 | 0.37× | 0.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.077 | 0.98× |
| 1 | 5 | 0.258 | 0.332 | 1.28× |
| 1 | 10 | 0.393 | 0.688 | 1.75× |
| 10 | 1 | 0.043 | 0.065 | 1.53× |
| 10 | 5 | 0.172 | 0.321 | 1.87× |
| 10 | 10 | 0.401 | 0.738 | 1.84× |
| 100 | 1 | 0.048 | 0.073 | 1.52× |
| 100 | 5 | 0.199 | 0.330 | 1.66× |
| 100 | 10 | 0.437 | 0.697 | 1.60× |
| 1,000 | 1 | 0.047 | 0.065 | 1.40× |
| 1,000 | 5 | 0.201 | 0.368 | 1.83× |
| 1,000 | 10 | 0.444 | 0.805 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
