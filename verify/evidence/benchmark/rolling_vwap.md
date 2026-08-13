# RollingVolumeWeightedAveragePrice benchmark (`RollingVWAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.504 | 1.98M | 0.517 | 1.94M | 0.256 | 0.51× | 0.49× |
| 10,000 | 4.678 | 2.14M | 4.742 | 2.11M | 1.404 | 0.30× | 0.30× |
| 100,000 | 51.306 | 1.95M | 47.011 | 2.13M | 12.750 | 0.25× | 0.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.319 | 0.263 | 0.83× |
| 1 | 5 | 0.489 | 1.402 | 2.87× |
| 1 | 10 | 0.661 | 2.237 | 3.38× |
| 10 | 1 | 0.072 | 0.198 | 2.74× |
| 10 | 5 | 0.328 | 0.993 | 3.03× |
| 10 | 10 | 0.625 | 2.277 | 3.64× |
| 100 | 1 | 0.120 | 0.210 | 1.75× |
| 100 | 5 | 0.324 | 1.085 | 3.35× |
| 100 | 10 | 0.692 | 2.405 | 3.48× |
| 1,000 | 1 | 0.553 | 0.336 | 0.61× |
| 1,000 | 5 | 0.864 | 1.662 | 1.92× |
| 1,000 | 10 | 1.591 | 3.709 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
