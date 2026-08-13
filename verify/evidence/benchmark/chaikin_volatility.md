# ChaikinVolatility benchmark (`ChaikinVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.076 | 13.08M | 0.069 | 14.40M | 0.222 | 2.91× | 3.20× |
| 10,000 | 0.596 | 16.77M | 0.580 | 17.24M | 0.813 | 1.36× | 1.40× |
| 100,000 | 5.736 | 17.44M | 5.669 | 17.64M | 6.433 | 1.12× | 1.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.231 | 0.270 | 1.17× |
| 1 | 5 | 0.461 | 1.555 | 3.38× |
| 1 | 10 | 0.682 | 2.815 | 4.13× |
| 10 | 1 | 0.084 | 0.252 | 2.99× |
| 10 | 5 | 0.325 | 1.524 | 4.69× |
| 10 | 10 | 0.659 | 2.573 | 3.90× |
| 100 | 1 | 0.083 | 0.247 | 2.98× |
| 100 | 5 | 0.325 | 1.487 | 4.57× |
| 100 | 10 | 0.667 | 2.858 | 4.28× |
| 1,000 | 1 | 0.142 | 0.330 | 2.33× |
| 1,000 | 5 | 0.320 | 1.958 | 6.11× |
| 1,000 | 10 | 0.691 | 3.277 | 4.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
