# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.77M | 0.016 | 63.64M | 0.040 | 1.93× | 2.52× |
| 10,000 | 0.180 | 55.64M | 0.167 | 60.04M | 0.183 | 1.02× | 1.10× |
| 100,000 | 1.705 | 58.66M | 1.646 | 60.75M | 1.601 | 0.94× | 0.97× |
| 1,000,000 | 16.973 | 58.92M | 17.124 | 58.40M | 15.706 | 0.93× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.138 | 1.57× |
| 1 | 5 | 0.358 | 0.556 | 1.55× |
| 1 | 10 | 0.579 | 0.935 | 1.62× |
| 10 | 1 | 0.072 | 0.118 | 1.64× |
| 10 | 5 | 0.344 | 0.638 | 1.85× |
| 10 | 10 | 0.701 | 0.999 | 1.43× |
| 100 | 1 | 0.060 | 0.110 | 1.84× |
| 100 | 5 | 0.337 | 0.619 | 1.84× |
| 100 | 10 | 0.634 | 1.010 | 1.59× |
| 1,000 | 1 | 0.069 | 0.104 | 1.50× |
| 1,000 | 5 | 0.327 | 0.625 | 1.91× |
| 1,000 | 10 | 0.804 | 1.177 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
