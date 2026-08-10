# VolumeZoneOscillator benchmark (`VZO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.74M | 0.015 | 67.62M | 0.086 | 4.69× | 5.80× |
| 10,000 | 0.113 | 88.65M | 0.113 | 88.87M | 0.784 | 6.95× | 6.97× |
| 100,000 | 1.138 | 87.90M | 1.119 | 89.35M | 7.342 | 6.45× | 6.56× |
| 1,000,000 | 11.960 | 83.61M | 12.199 | 81.97M | 73.311 | 6.13× | 6.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.092 | 1.20× |
| 1 | 5 | 0.301 | 0.347 | 1.15× |
| 1 | 10 | 0.495 | 0.766 | 1.55× |
| 10 | 1 | 0.052 | 0.071 | 1.38× |
| 10 | 5 | 0.239 | 0.350 | 1.47× |
| 10 | 10 | 0.498 | 0.755 | 1.52× |
| 100 | 1 | 0.053 | 0.076 | 1.43× |
| 100 | 5 | 0.248 | 0.407 | 1.64× |
| 100 | 10 | 0.506 | 0.779 | 1.54× |
| 1,000 | 1 | 0.059 | 0.143 | 2.42× |
| 1,000 | 5 | 0.268 | 0.781 | 2.91× |
| 1,000 | 10 | 0.556 | 1.511 | 2.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
