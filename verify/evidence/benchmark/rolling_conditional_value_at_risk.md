# RollingConditionalValueAtRisk benchmark (`ConditionalValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.127 | 7.88M | 0.130 | 7.71M | 0.337 | 2.65× | 2.60× |
| 10,000 | 1.324 | 7.55M | 1.304 | 7.67M | 1.953 | 1.48× | 1.50× |
| 100,000 | 13.618 | 7.34M | 13.819 | 7.24M | 16.996 | 1.25× | 1.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.277 | 2.48× |
| 1 | 5 | 0.270 | 1.283 | 4.76× |
| 1 | 10 | 0.411 | 2.635 | 6.42× |
| 10 | 1 | 0.051 | 0.271 | 5.36× |
| 10 | 5 | 0.198 | 1.149 | 5.81× |
| 10 | 10 | 0.402 | 2.471 | 6.15× |
| 100 | 1 | 0.062 | 0.255 | 4.13× |
| 100 | 5 | 0.205 | 1.449 | 7.06× |
| 100 | 10 | 0.446 | 2.780 | 6.23× |
| 1,000 | 1 | 0.186 | 0.413 | 2.22× |
| 1,000 | 5 | 0.344 | 2.294 | 6.66× |
| 1,000 | 10 | 0.563 | 4.352 | 7.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
