# VolumeRelativeStrengthIndex benchmark (`VolumeRsi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.86M | 0.039 | 25.83M | 0.199 | 4.74× | 5.14× |
| 10,000 | 0.373 | 26.82M | 0.317 | 31.55M | 0.813 | 2.18× | 2.57× |
| 100,000 | 3.134 | 31.91M | 3.010 | 33.22M | 6.997 | 2.23× | 2.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.194 | 0.258 | 1.33× |
| 1 | 5 | 0.360 | 1.110 | 3.08× |
| 1 | 10 | 0.612 | 2.449 | 4.00× |
| 10 | 1 | 0.082 | 0.228 | 2.80× |
| 10 | 5 | 0.286 | 1.247 | 4.35× |
| 10 | 10 | 0.591 | 2.392 | 4.05× |
| 100 | 1 | 0.071 | 0.223 | 3.15× |
| 100 | 5 | 0.312 | 1.359 | 4.35× |
| 100 | 10 | 0.617 | 2.366 | 3.83× |
| 1,000 | 1 | 0.106 | 0.290 | 2.74× |
| 1,000 | 5 | 0.317 | 1.629 | 5.14× |
| 1,000 | 10 | 0.646 | 3.149 | 4.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
