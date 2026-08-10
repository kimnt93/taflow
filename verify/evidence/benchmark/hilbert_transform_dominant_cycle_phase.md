# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.120 | 8.32M | 0.116 | 8.61M | 0.572 | 4.76× | 4.92× |
| 10,000 | 1.234 | 8.10M | 1.195 | 8.37M | 5.127 | 4.16× | 4.29× |
| 100,000 | 12.027 | 8.31M | 12.341 | 8.10M | 51.679 | 4.30× | 4.19× |
| 1,000,000 | 118.500 | 8.44M | 116.614 | 8.58M | 465.959 | 3.93× | 4.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.110 | 0.84× |
| 1 | 5 | 0.316 | 0.454 | 1.44× |
| 1 | 10 | 0.509 | 1.025 | 2.01× |
| 10 | 1 | 0.055 | 0.088 | 1.60× |
| 10 | 5 | 0.231 | 0.455 | 1.97× |
| 10 | 10 | 0.474 | 0.993 | 2.10× |
| 100 | 1 | 0.074 | 0.121 | 1.63× |
| 100 | 5 | 0.267 | 0.612 | 2.29× |
| 100 | 10 | 0.515 | 1.209 | 2.35× |
| 1,000 | 1 | 0.161 | 0.548 | 3.40× |
| 1,000 | 5 | 0.306 | 2.682 | 8.75× |
| 1,000 | 10 | 0.699 | 5.518 | 7.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
