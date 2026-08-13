# UpDownVolumeRatio benchmark (`UpDownVolumeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.18M | 0.031 | 32.04M | 4.129 | 116.33× | 132.28× |
| 10,000 | 0.247 | 40.49M | 0.233 | 42.91M | 40.045 | 162.16× | 171.81× |
| 100,000 | 2.235 | 44.74M | 2.165 | 46.19M | 400.435 | 179.16× | 184.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.236 | 2.74× |
| 1 | 5 | 0.331 | 1.006 | 3.04× |
| 1 | 10 | 0.632 | 2.013 | 3.18× |
| 10 | 1 | 0.085 | 0.256 | 3.01× |
| 10 | 5 | 0.320 | 1.525 | 4.77× |
| 10 | 10 | 0.762 | 2.608 | 3.42× |
| 100 | 1 | 0.069 | 0.607 | 8.84× |
| 100 | 5 | 0.304 | 3.060 | 10.06× |
| 100 | 10 | 0.655 | 6.326 | 9.66× |
| 1,000 | 1 | 0.098 | 5.209 | 53.35× |
| 1,000 | 5 | 0.399 | 24.286 | 60.85× |
| 1,000 | 10 | 0.731 | 47.433 | 64.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
