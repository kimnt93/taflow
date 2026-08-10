# VolumeZoneOscillator benchmark (`VZO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.80M | 0.014 | 69.54M | 0.195 | 12.26× | 13.58× |
| 10,000 | 0.120 | 83.40M | 0.125 | 80.10M | 0.796 | 6.64× | 6.37× |
| 100,000 | 1.160 | 86.21M | 1.258 | 79.47M | 6.991 | 6.03× | 5.56× |
| 1,000,000 | 11.576 | 86.39M | 12.083 | 82.76M | 69.299 | 5.99× | 5.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.260 | 2.26× |
| 1 | 5 | 0.332 | 1.275 | 3.84× |
| 1 | 10 | 0.494 | 2.262 | 4.58× |
| 10 | 1 | 0.054 | 0.211 | 3.88× |
| 10 | 5 | 0.237 | 1.233 | 5.19× |
| 10 | 10 | 0.491 | 2.337 | 4.76× |
| 100 | 1 | 0.055 | 0.226 | 4.10× |
| 100 | 5 | 0.259 | 1.265 | 4.88× |
| 100 | 10 | 0.523 | 2.534 | 4.84× |
| 1,000 | 1 | 0.068 | 0.285 | 4.20× |
| 1,000 | 5 | 0.278 | 1.678 | 6.04× |
| 1,000 | 10 | 0.542 | 3.074 | 5.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
