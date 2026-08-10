# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.88M | 0.019 | 51.30M | 0.055 | 2.74× | 2.82× |
| 10,000 | 0.168 | 59.58M | 0.148 | 67.46M | 0.198 | 1.18× | 1.34× |
| 100,000 | 1.455 | 68.72M | 1.413 | 70.77M | 1.579 | 1.09× | 1.12× |
| 1,000,000 | 15.456 | 64.70M | 14.280 | 70.03M | 16.697 | 1.08× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.134 | 2.14× |
| 1 | 5 | 0.315 | 0.580 | 1.84× |
| 1 | 10 | 0.552 | 1.235 | 2.24× |
| 10 | 1 | 0.076 | 0.136 | 1.78× |
| 10 | 5 | 0.362 | 2.006 | 5.54× |
| 10 | 10 | 1.399 | 1.205 | 0.86× |
| 100 | 1 | 0.268 | 0.156 | 0.58× |
| 100 | 5 | 0.333 | 0.662 | 1.99× |
| 100 | 10 | 0.685 | 1.379 | 2.01× |
| 1,000 | 1 | 0.076 | 0.119 | 1.56× |
| 1,000 | 5 | 0.341 | 0.749 | 2.19× |
| 1,000 | 10 | 0.661 | 1.355 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
