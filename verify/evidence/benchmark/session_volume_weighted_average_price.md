# SessionVolumeWeightedAveragePrice benchmark (`SessionVwap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.18M | 0.007 | 139.60M | 0.365 | 31.50× | 51.02× |
| 10,000 | 0.067 | 149.82M | 0.060 | 167.21M | 2.381 | 35.67× | 39.81× |
| 100,000 | 0.590 | 169.53M | 0.562 | 177.92M | 22.738 | 38.55× | 40.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.280 | 3.27× |
| 1 | 5 | 0.274 | 1.121 | 4.09× |
| 1 | 10 | 0.407 | 2.487 | 6.11× |
| 10 | 1 | 0.042 | 0.222 | 5.27× |
| 10 | 5 | 0.190 | 1.147 | 6.04× |
| 10 | 10 | 0.404 | 2.361 | 5.85× |
| 100 | 1 | 0.050 | 0.251 | 5.04× |
| 100 | 5 | 0.233 | 1.435 | 6.16× |
| 100 | 10 | 0.445 | 2.588 | 5.81× |
| 1,000 | 1 | 0.051 | 0.452 | 8.91× |
| 1,000 | 5 | 0.217 | 2.447 | 11.26× |
| 1,000 | 10 | 0.510 | 4.827 | 9.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
