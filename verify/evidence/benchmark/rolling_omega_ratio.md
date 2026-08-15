# RollingOmegaRatio benchmark (`OmegaRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.84M | 0.027 | 36.77M | 0.211 | 7.37× | 7.78× |
| 10,000 | 0.268 | 37.35M | 0.288 | 34.69M | 0.710 | 2.65× | 2.46× |
| 100,000 | 2.635 | 37.95M | 2.566 | 38.97M | 5.712 | 2.17× | 2.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.258 | 3.41× |
| 1 | 5 | 0.286 | 1.254 | 4.39× |
| 1 | 10 | 0.398 | 2.493 | 6.26× |
| 10 | 1 | 0.056 | 0.253 | 4.53× |
| 10 | 5 | 0.199 | 1.388 | 6.97× |
| 10 | 10 | 0.402 | 2.637 | 6.57× |
| 100 | 1 | 0.047 | 0.246 | 5.21× |
| 100 | 5 | 0.204 | 1.383 | 6.77× |
| 100 | 10 | 0.461 | 2.585 | 5.60× |
| 1,000 | 1 | 0.071 | 0.287 | 4.04× |
| 1,000 | 5 | 0.208 | 1.742 | 8.36× |
| 1,000 | 10 | 0.446 | 3.165 | 7.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
