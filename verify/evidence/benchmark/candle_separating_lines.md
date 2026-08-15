# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.72M | 0.004 | 230.30M | 0.035 | 4.71× | 8.17× |
| 10,000 | 0.063 | 159.50M | 0.056 | 178.31M | 0.137 | 2.19× | 2.45× |
| 100,000 | 0.630 | 158.65M | 0.601 | 166.41M | 0.984 | 1.56× | 1.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.113 | 1.49× |
| 1 | 5 | 0.219 | 0.470 | 2.15× |
| 1 | 10 | 0.398 | 0.972 | 2.44× |
| 10 | 1 | 0.047 | 0.097 | 2.05× |
| 10 | 5 | 0.208 | 0.529 | 2.54× |
| 10 | 10 | 0.400 | 0.922 | 2.31× |
| 100 | 1 | 0.046 | 0.097 | 2.12× |
| 100 | 5 | 0.190 | 0.483 | 2.55× |
| 100 | 10 | 0.422 | 0.931 | 2.21× |
| 1,000 | 1 | 0.056 | 0.100 | 1.78× |
| 1,000 | 5 | 0.203 | 0.481 | 2.37× |
| 1,000 | 10 | 0.470 | 1.024 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
