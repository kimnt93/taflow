# RoofingFilter benchmark (`RoofingFilter` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.19M | 0.039 | 25.52M | 0.182 | 4.05× | 4.65× |
| 10,000 | 0.328 | 30.47M | 0.329 | 30.35M | 0.498 | 1.52× | 1.51× |
| 100,000 | 3.126 | 31.99M | 3.083 | 32.44M | 3.526 | 1.13× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.279 | 1.60× |
| 1 | 5 | 0.429 | 1.414 | 3.30× |
| 1 | 10 | 0.621 | 2.647 | 4.27× |
| 10 | 1 | 0.074 | 0.240 | 3.26× |
| 10 | 5 | 0.317 | 1.416 | 4.46× |
| 10 | 10 | 0.618 | 2.495 | 4.04× |
| 100 | 1 | 0.075 | 0.247 | 3.27× |
| 100 | 5 | 0.305 | 1.360 | 4.46× |
| 100 | 10 | 0.656 | 2.768 | 4.22× |
| 1,000 | 1 | 0.115 | 0.286 | 2.49× |
| 1,000 | 5 | 0.307 | 1.550 | 5.05× |
| 1,000 | 10 | 0.665 | 2.863 | 4.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
