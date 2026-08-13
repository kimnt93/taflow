# LowestSince benchmark (`lowest since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.06M | 0.038 | 26.58M | 0.261 | 6.01× | 6.93× |
| 10,000 | 0.291 | 34.35M | 0.286 | 34.97M | 2.689 | 9.24× | 9.40× |
| 100,000 | 2.727 | 36.67M | 2.715 | 36.83M | 25.462 | 9.34× | 9.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.144 | 0.203 | 1.41× |
| 1 | 5 | 0.440 | 0.335 | 0.76× |
| 1 | 10 | 0.588 | 0.658 | 1.12× |
| 10 | 1 | 0.063 | 0.073 | 1.16× |
| 10 | 5 | 0.292 | 0.327 | 1.12× |
| 10 | 10 | 0.601 | 0.697 | 1.16× |
| 100 | 1 | 0.071 | 0.099 | 1.40× |
| 100 | 5 | 0.295 | 0.452 | 1.53× |
| 100 | 10 | 0.638 | 0.934 | 1.47× |
| 1,000 | 1 | 0.099 | 0.343 | 3.46× |
| 1,000 | 5 | 0.331 | 1.728 | 5.22× |
| 1,000 | 10 | 0.628 | 3.621 | 5.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
