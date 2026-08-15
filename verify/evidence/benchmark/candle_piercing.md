# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 166.00M | 0.003 | 355.01M | 0.033 | 5.41× | 11.56× |
| 10,000 | 0.074 | 135.42M | 0.068 | 147.42M | 0.118 | 1.60× | 1.74× |
| 100,000 | 0.845 | 118.37M | 0.786 | 127.19M | 0.968 | 1.15× | 1.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.181 | 1.72× |
| 1 | 5 | 0.310 | 0.471 | 1.52× |
| 1 | 10 | 0.387 | 0.895 | 2.31× |
| 10 | 1 | 0.040 | 0.091 | 2.27× |
| 10 | 5 | 0.180 | 0.440 | 2.45× |
| 10 | 10 | 0.453 | 0.928 | 2.05× |
| 100 | 1 | 0.045 | 0.087 | 1.93× |
| 100 | 5 | 0.183 | 0.426 | 2.33× |
| 100 | 10 | 0.388 | 0.991 | 2.56× |
| 1,000 | 1 | 0.055 | 0.104 | 1.88× |
| 1,000 | 5 | 0.213 | 0.475 | 2.23× |
| 1,000 | 10 | 0.417 | 1.046 | 2.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
