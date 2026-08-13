# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.094 | 10.59M | 0.078 | 12.80M | 0.038 | 0.40× | 0.48× |
| 10,000 | 0.687 | 14.55M | 0.649 | 15.41M | 0.114 | 0.17× | 0.17× |
| 100,000 | 6.594 | 15.17M | 6.792 | 14.72M | 0.783 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.173 | 0.119 | 0.69× |
| 1 | 5 | 0.487 | 0.517 | 1.06× |
| 1 | 10 | 0.661 | 0.921 | 1.39× |
| 10 | 1 | 0.065 | 0.086 | 1.32× |
| 10 | 5 | 0.311 | 0.428 | 1.38× |
| 10 | 10 | 0.643 | 0.948 | 1.47× |
| 100 | 1 | 0.082 | 0.094 | 1.15× |
| 100 | 5 | 0.335 | 0.445 | 1.33× |
| 100 | 10 | 0.648 | 0.947 | 1.46× |
| 1,000 | 1 | 0.137 | 0.096 | 0.70× |
| 1,000 | 5 | 0.317 | 0.473 | 1.49× |
| 1,000 | 10 | 0.676 | 1.039 | 1.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
