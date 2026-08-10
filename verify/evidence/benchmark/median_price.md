# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 174.47M | 0.005 | 222.22M | 0.031 | 5.36× | 6.83× |
| 10,000 | 0.021 | 478.58M | 0.018 | 566.95M | 0.034 | 1.62× | 1.92× |
| 100,000 | 0.180 | 555.30M | 0.142 | 703.50M | 0.080 | 0.44× | 0.56× |
| 1,000,000 | 2.391 | 418.28M | 1.953 | 512.11M | 1.155 | 0.48× | 0.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.124 | 1.30× |
| 1 | 5 | 0.250 | 0.526 | 2.11× |
| 1 | 10 | 0.556 | 1.065 | 1.91× |
| 10 | 1 | 0.056 | 0.088 | 1.58× |
| 10 | 5 | 0.302 | 0.511 | 1.69× |
| 10 | 10 | 0.494 | 1.018 | 2.06× |
| 100 | 1 | 0.052 | 0.091 | 1.77× |
| 100 | 5 | 0.257 | 0.491 | 1.91× |
| 100 | 10 | 0.501 | 0.944 | 1.89× |
| 1,000 | 1 | 0.052 | 0.087 | 1.68× |
| 1,000 | 5 | 0.271 | 0.493 | 1.82× |
| 1,000 | 10 | 0.560 | 0.914 | 1.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
