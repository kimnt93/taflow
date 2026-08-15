# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.42M | 0.009 | 107.38M | 0.039 | 3.76× | 4.19× |
| 10,000 | 0.102 | 97.66M | 0.109 | 91.87M | 0.100 | 0.98× | 0.92× |
| 100,000 | 1.024 | 97.65M | 0.971 | 102.97M | 0.661 | 0.65× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.106 | 1.89× |
| 1 | 5 | 0.277 | 0.495 | 1.79× |
| 1 | 10 | 0.396 | 1.028 | 2.60× |
| 10 | 1 | 0.046 | 0.096 | 2.08× |
| 10 | 5 | 0.199 | 0.473 | 2.37× |
| 10 | 10 | 0.408 | 0.964 | 2.36× |
| 100 | 1 | 0.043 | 0.115 | 2.69× |
| 100 | 5 | 0.202 | 0.481 | 2.38× |
| 100 | 10 | 0.388 | 1.026 | 2.65× |
| 1,000 | 1 | 0.058 | 0.101 | 1.75× |
| 1,000 | 5 | 0.216 | 0.575 | 2.66× |
| 1,000 | 10 | 0.434 | 1.134 | 2.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
