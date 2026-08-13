# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.108 | 9.24M | 0.136 | 7.34M | 0.048 | 0.45× | 0.35× |
| 10,000 | 0.699 | 14.30M | 0.675 | 14.82M | 0.113 | 0.16× | 0.17× |
| 100,000 | 7.508 | 13.32M | 7.261 | 13.77M | 1.382 | 0.18× | 0.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.159 | 0.136 | 0.86× |
| 1 | 5 | 0.498 | 0.525 | 1.05× |
| 1 | 10 | 0.660 | 1.037 | 1.57× |
| 10 | 1 | 0.075 | 0.108 | 1.45× |
| 10 | 5 | 0.326 | 0.501 | 1.54× |
| 10 | 10 | 0.661 | 1.025 | 1.55× |
| 100 | 1 | 0.078 | 0.101 | 1.30× |
| 100 | 5 | 0.317 | 0.479 | 1.51× |
| 100 | 10 | 0.700 | 1.037 | 1.48× |
| 1,000 | 1 | 0.145 | 0.109 | 0.75× |
| 1,000 | 5 | 0.320 | 0.533 | 1.67× |
| 1,000 | 10 | 0.695 | 1.085 | 1.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
