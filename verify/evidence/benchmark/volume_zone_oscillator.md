# VolumeZoneOscillator benchmark (`VZO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.89M | 0.013 | 75.27M | 0.231 | 20.04× | 17.36× |
| 10,000 | 0.086 | 116.06M | 0.083 | 120.74M | 0.907 | 10.53× | 10.95× |
| 100,000 | 0.811 | 123.36M | 0.817 | 122.33M | 7.724 | 9.53× | 9.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.271 | 3.73× |
| 1 | 5 | 0.226 | 1.068 | 4.72× |
| 1 | 10 | 0.392 | 2.318 | 5.91× |
| 10 | 1 | 0.048 | 0.216 | 4.49× |
| 10 | 5 | 0.199 | 1.319 | 6.63× |
| 10 | 10 | 0.428 | 2.312 | 5.40× |
| 100 | 1 | 0.057 | 0.223 | 3.92× |
| 100 | 5 | 0.209 | 1.284 | 6.14× |
| 100 | 10 | 0.455 | 2.372 | 5.22× |
| 1,000 | 1 | 0.054 | 0.296 | 5.47× |
| 1,000 | 5 | 0.223 | 1.682 | 7.52× |
| 1,000 | 10 | 0.422 | 3.140 | 7.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
