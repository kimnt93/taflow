# YangZhang benchmark (`YangZhangVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.188 | 5.31M | 0.167 | 5.99M | 0.304 | 1.61× | 1.82× |
| 10,000 | 1.634 | 6.12M | 1.545 | 6.47M | 1.710 | 1.05× | 1.11× |
| 100,000 | 15.173 | 6.59M | 15.675 | 6.38M | 15.754 | 1.04× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.282 | 2.30× |
| 1 | 5 | 0.575 | 1.469 | 2.55× |
| 1 | 10 | 0.707 | 2.553 | 3.61× |
| 10 | 1 | 0.081 | 0.242 | 3.01× |
| 10 | 5 | 0.334 | 1.453 | 4.35× |
| 10 | 10 | 0.688 | 2.682 | 3.90× |
| 100 | 1 | 0.090 | 0.253 | 2.80× |
| 100 | 5 | 0.316 | 1.491 | 4.71× |
| 100 | 10 | 0.661 | 2.605 | 3.94× |
| 1,000 | 1 | 0.253 | 0.399 | 1.58× |
| 1,000 | 5 | 0.412 | 2.285 | 5.55× |
| 1,000 | 10 | 0.809 | 4.566 | 5.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
