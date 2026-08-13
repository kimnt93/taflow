# HilbertDominantCycle benchmark (`HilbertDominantCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.215 | 4.65M | 0.211 | 4.74M | 0.175 | 0.81× | 0.83× |
| 10,000 | 2.087 | 4.79M | 2.178 | 4.59M | 0.957 | 0.46× | 0.44× |
| 100,000 | 22.037 | 4.54M | 20.469 | 4.89M | 8.747 | 0.40× | 0.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.191 | 1.78× |
| 1 | 5 | 0.379 | 1.131 | 2.98× |
| 1 | 10 | 0.633 | 2.168 | 3.43× |
| 10 | 1 | 0.077 | 0.161 | 2.09× |
| 10 | 5 | 0.290 | 0.789 | 2.72× |
| 10 | 10 | 0.625 | 1.888 | 3.02× |
| 100 | 1 | 0.090 | 0.171 | 1.91× |
| 100 | 5 | 0.307 | 0.836 | 2.72× |
| 100 | 10 | 0.618 | 1.909 | 3.09× |
| 1,000 | 1 | 0.281 | 0.256 | 0.91× |
| 1,000 | 5 | 0.474 | 1.240 | 2.62× |
| 1,000 | 10 | 0.833 | 2.572 | 3.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
