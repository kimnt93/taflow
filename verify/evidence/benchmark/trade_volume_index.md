# TradeVolumeIndex benchmark (`TradeVolumeIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.42M | 0.006 | 155.57M | 0.227 | 30.00× | 35.24× |
| 10,000 | 0.069 | 144.35M | 0.065 | 152.92M | 0.915 | 13.21× | 14.00× |
| 100,000 | 0.814 | 122.78M | 0.983 | 101.70M | 9.625 | 11.82× | 9.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.420 | 4.47× |
| 1 | 5 | 0.341 | 1.314 | 3.86× |
| 1 | 10 | 0.540 | 2.461 | 4.55× |
| 10 | 1 | 0.051 | 0.237 | 4.61× |
| 10 | 5 | 0.245 | 1.334 | 5.45× |
| 10 | 10 | 0.528 | 2.327 | 4.41× |
| 100 | 1 | 0.051 | 0.226 | 4.41× |
| 100 | 5 | 0.232 | 1.379 | 5.95× |
| 100 | 10 | 0.516 | 2.520 | 4.88× |
| 1,000 | 1 | 0.076 | 0.297 | 3.90× |
| 1,000 | 5 | 0.252 | 1.680 | 6.66× |
| 1,000 | 10 | 0.510 | 3.146 | 6.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
