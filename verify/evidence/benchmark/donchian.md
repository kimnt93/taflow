# Donchian benchmark (`Donchian` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.49M | 0.006 | 176.04M | 0.582 | 73.06× | 102.50× |
| 10,000 | 0.055 | 180.21M | 0.049 | 204.98M | 4.433 | 79.89× | 90.87× |
| 100,000 | 0.539 | 185.42M | 0.477 | 209.63M | 47.280 | 87.66× | 99.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.279 | 3.74× |
| 1 | 5 | 0.294 | 1.152 | 3.92× |
| 1 | 10 | 0.426 | 2.474 | 5.81× |
| 10 | 1 | 0.043 | 0.225 | 5.21× |
| 10 | 5 | 0.195 | 1.419 | 7.26× |
| 10 | 10 | 0.437 | 2.466 | 5.65× |
| 100 | 1 | 0.046 | 0.276 | 6.01× |
| 100 | 5 | 0.223 | 1.669 | 7.47× |
| 100 | 10 | 0.439 | 2.892 | 6.58× |
| 1,000 | 1 | 0.061 | 0.943 | 15.45× |
| 1,000 | 5 | 0.218 | 3.775 | 17.31× |
| 1,000 | 10 | 0.432 | 7.423 | 17.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
