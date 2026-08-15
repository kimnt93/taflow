# VolumeZoneOscillator benchmark (`VZO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.82M | 0.008 | 118.22M | 0.209 | 21.33× | 24.76× |
| 10,000 | 0.082 | 121.94M | 0.078 | 128.04M | 0.879 | 10.71× | 11.25× |
| 100,000 | 0.822 | 121.66M | 0.780 | 128.19M | 7.388 | 8.99× | 9.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.276 | 2.37× |
| 1 | 5 | 0.272 | 1.082 | 3.98× |
| 1 | 10 | 0.415 | 2.288 | 5.52× |
| 10 | 1 | 0.047 | 0.217 | 4.59× |
| 10 | 5 | 0.187 | 1.242 | 6.65× |
| 10 | 10 | 0.453 | 2.373 | 5.24× |
| 100 | 1 | 0.050 | 0.226 | 4.51× |
| 100 | 5 | 0.194 | 1.358 | 6.99× |
| 100 | 10 | 0.449 | 2.334 | 5.19× |
| 1,000 | 1 | 0.052 | 0.281 | 5.42× |
| 1,000 | 5 | 0.234 | 1.736 | 7.42× |
| 1,000 | 10 | 0.443 | 3.082 | 6.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
