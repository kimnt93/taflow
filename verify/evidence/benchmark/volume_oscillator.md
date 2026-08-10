# VolumeOscillator benchmark (`VolumeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.37M | 0.014 | 72.22M | 0.063 | 4.47× | 4.52× |
| 10,000 | 0.106 | 94.52M | 0.115 | 86.80M | 0.418 | 3.95× | 3.63× |
| 100,000 | 1.038 | 96.38M | 1.103 | 90.67M | 4.019 | 3.87× | 3.64× |
| 1,000,000 | 10.517 | 95.08M | 11.315 | 88.38M | 40.983 | 3.90× | 3.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.101 | 1.60× |
| 1 | 5 | 0.309 | 0.413 | 1.34× |
| 1 | 10 | 0.470 | 0.882 | 1.88× |
| 10 | 1 | 0.052 | 0.081 | 1.56× |
| 10 | 5 | 0.231 | 0.438 | 1.89× |
| 10 | 10 | 0.495 | 0.868 | 1.75× |
| 100 | 1 | 0.054 | 0.085 | 1.56× |
| 100 | 5 | 0.276 | 0.453 | 1.64× |
| 100 | 10 | 0.499 | 0.836 | 1.68× |
| 1,000 | 1 | 0.061 | 0.121 | 2.00× |
| 1,000 | 5 | 0.239 | 0.631 | 2.63× |
| 1,000 | 10 | 0.510 | 1.286 | 2.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
