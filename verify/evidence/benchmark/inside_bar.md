# InsideBar benchmark (`inside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 189.69M | 0.004 | 279.83M | 0.028 | 5.23× | 7.71× |
| 10,000 | 0.030 | 335.89M | 0.026 | 385.55M | 0.041 | 1.39× | 1.59× |
| 100,000 | 0.273 | 366.08M | 0.279 | 357.86M | 0.229 | 0.84× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.106 | 1.30× |
| 1 | 5 | 0.275 | 0.351 | 1.28× |
| 1 | 10 | 0.374 | 0.758 | 2.03× |
| 10 | 1 | 0.047 | 0.071 | 1.53× |
| 10 | 5 | 0.181 | 0.354 | 1.96× |
| 10 | 10 | 0.386 | 0.765 | 1.98× |
| 100 | 1 | 0.042 | 0.071 | 1.69× |
| 100 | 5 | 0.175 | 0.335 | 1.91× |
| 100 | 10 | 0.413 | 0.765 | 1.85× |
| 1,000 | 1 | 0.051 | 0.078 | 1.54× |
| 1,000 | 5 | 0.190 | 0.477 | 2.52× |
| 1,000 | 10 | 0.425 | 1.119 | 2.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
