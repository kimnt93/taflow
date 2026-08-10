# AverageDailyRange benchmark (`AverageDailyRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.84M | 0.013 | 78.49M | 0.406 | 25.54× | 31.89× |
| 10,000 | 0.083 | 120.59M | 0.063 | 159.75M | 2.399 | 28.94× | 38.33× |
| 100,000 | 0.649 | 154.19M | 0.542 | 184.65M | 23.467 | 36.18× | 43.33× |
| 1,000,000 | 5.848 | 170.99M | 5.299 | 188.70M | 258.413 | 44.19× | 48.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.315 | 3.43× |
| 1 | 5 | 0.447 | 1.523 | 3.40× |
| 1 | 10 | 0.575 | 2.873 | 4.99× |
| 10 | 1 | 0.060 | 0.284 | 4.72× |
| 10 | 5 | 0.285 | 1.506 | 5.28× |
| 10 | 10 | 0.556 | 2.852 | 5.13× |
| 100 | 1 | 0.062 | 0.297 | 4.75× |
| 100 | 5 | 0.287 | 1.629 | 5.69× |
| 100 | 10 | 0.594 | 3.160 | 5.32× |
| 1,000 | 1 | 0.063 | 0.506 | 7.97× |
| 1,000 | 5 | 0.287 | 2.740 | 9.55× |
| 1,000 | 10 | 0.612 | 5.303 | 8.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
