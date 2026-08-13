# SessionVolumeWeightedAveragePrice benchmark (`SessionVwap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.077 | 12.94M | 0.070 | 14.24M | 0.479 | 6.19× | 6.81× |
| 10,000 | 0.545 | 18.34M | 0.533 | 18.76M | 2.356 | 4.32× | 4.42× |
| 100,000 | 5.782 | 17.29M | 5.465 | 18.30M | 30.207 | 5.22× | 5.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.285 | 2.67× |
| 1 | 5 | 0.437 | 1.187 | 2.72× |
| 1 | 10 | 0.709 | 2.496 | 3.52× |
| 10 | 1 | 0.077 | 0.223 | 2.91× |
| 10 | 5 | 0.351 | 1.088 | 3.10× |
| 10 | 10 | 0.703 | 2.316 | 3.29× |
| 100 | 1 | 0.092 | 0.247 | 2.70× |
| 100 | 5 | 0.357 | 1.398 | 3.92× |
| 100 | 10 | 0.695 | 2.567 | 3.69× |
| 1,000 | 1 | 0.134 | 0.457 | 3.42× |
| 1,000 | 5 | 0.353 | 2.445 | 6.93× |
| 1,000 | 10 | 0.745 | 4.842 | 6.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
