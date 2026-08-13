# LogReturn benchmark (`LogReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.75M | 0.053 | 18.70M | 0.171 | 3.04× | 3.20× |
| 10,000 | 0.450 | 22.23M | 0.455 | 21.98M | 0.520 | 1.16× | 1.14× |
| 100,000 | 4.397 | 22.74M | 4.432 | 22.57M | 4.893 | 1.11× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.225 | 2.57× |
| 1 | 5 | 0.411 | 1.316 | 3.21× |
| 1 | 10 | 0.639 | 2.244 | 3.51× |
| 10 | 1 | 0.077 | 0.219 | 2.85× |
| 10 | 5 | 0.299 | 1.234 | 4.13× |
| 10 | 10 | 0.613 | 2.223 | 3.62× |
| 100 | 1 | 0.076 | 0.219 | 2.88× |
| 100 | 5 | 0.298 | 1.268 | 4.26× |
| 100 | 10 | 0.650 | 8.510 | 13.09× |
| 1,000 | 1 | 0.132 | 0.237 | 1.79× |
| 1,000 | 5 | 0.315 | 1.364 | 4.33× |
| 1,000 | 10 | 0.635 | 2.634 | 4.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
