# KlingerVolumeOscillator benchmark (`KVO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.113 | 8.87M | 0.100 | 10.04M | 0.301 | 2.67× | 3.02× |
| 10,000 | 0.899 | 11.13M | 0.878 | 11.39M | 1.492 | 1.66× | 1.70× |
| 100,000 | 8.550 | 11.70M | 8.647 | 11.56M | 13.317 | 1.56× | 1.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.313 | 2.70× |
| 1 | 5 | 0.413 | 1.605 | 3.88× |
| 1 | 10 | 0.707 | 2.665 | 3.77× |
| 10 | 1 | 0.083 | 0.267 | 3.20× |
| 10 | 5 | 0.351 | 1.499 | 4.27× |
| 10 | 10 | 0.717 | 2.839 | 3.96× |
| 100 | 1 | 0.086 | 0.261 | 3.03× |
| 100 | 5 | 0.361 | 1.549 | 4.29× |
| 100 | 10 | 0.729 | 2.854 | 3.92× |
| 1,000 | 1 | 0.184 | 0.387 | 2.10× |
| 1,000 | 5 | 0.399 | 2.217 | 5.56× |
| 1,000 | 10 | 0.776 | 4.276 | 5.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
