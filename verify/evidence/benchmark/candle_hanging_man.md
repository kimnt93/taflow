# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.123 | 8.11M | 0.108 | 9.22M | 0.039 | 0.31× | 0.36× |
| 10,000 | 1.019 | 9.82M | 1.013 | 9.87M | 0.167 | 0.16× | 0.16× |
| 100,000 | 10.124 | 9.88M | 9.963 | 10.04M | 1.413 | 0.14× | 0.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.164 | 0.140 | 0.85× |
| 1 | 5 | 0.394 | 0.464 | 1.18× |
| 1 | 10 | 0.634 | 0.911 | 1.44× |
| 10 | 1 | 0.067 | 0.087 | 1.30× |
| 10 | 5 | 0.312 | 0.427 | 1.37× |
| 10 | 10 | 0.659 | 0.879 | 1.33× |
| 100 | 1 | 0.080 | 0.091 | 1.14× |
| 100 | 5 | 0.336 | 0.449 | 1.34× |
| 100 | 10 | 0.673 | 0.907 | 1.35× |
| 1,000 | 1 | 0.182 | 0.098 | 0.54× |
| 1,000 | 5 | 0.342 | 0.506 | 1.48× |
| 1,000 | 10 | 0.736 | 1.115 | 1.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
