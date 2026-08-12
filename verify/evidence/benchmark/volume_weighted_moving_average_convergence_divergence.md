# VolumeWeightedMovingAverageConvergenceDivergence benchmark (`VolumeWeightedMacd` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.22M | 0.025 | 39.45M | 0.674 | 28.46× | 26.59× |
| 10,000 | 0.194 | 51.64M | 0.195 | 51.22M | 4.687 | 24.20× | 24.01× |
| 100,000 | 1.782 | 56.13M | 1.864 | 53.65M | 52.726 | 29.60× | 28.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.326 | 3.92× |
| 1 | 5 | 0.286 | 1.507 | 5.27× |
| 1 | 10 | 0.573 | 3.211 | 5.61× |
| 10 | 1 | 0.057 | 0.293 | 5.11× |
| 10 | 5 | 0.274 | 1.687 | 6.17× |
| 10 | 10 | 0.510 | 3.424 | 6.71× |
| 100 | 1 | 0.056 | 0.332 | 5.93× |
| 100 | 5 | 0.250 | 1.751 | 7.00× |
| 100 | 10 | 0.584 | 3.634 | 6.23× |
| 1,000 | 1 | 0.078 | 0.834 | 10.73× |
| 1,000 | 5 | 0.254 | 3.861 | 15.23× |
| 1,000 | 10 | 0.590 | 7.715 | 13.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
