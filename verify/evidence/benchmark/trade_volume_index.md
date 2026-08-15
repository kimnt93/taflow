# TradeVolumeIndex benchmark (`TradeVolumeIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 185.72M | 0.004 | 267.34M | 0.195 | 36.18× | 52.08× |
| 10,000 | 0.069 | 143.95M | 0.066 | 151.93M | 0.767 | 11.04× | 11.66× |
| 100,000 | 0.723 | 138.27M | 0.701 | 142.66M | 6.830 | 9.44× | 9.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.301 | 2.77× |
| 1 | 5 | 0.320 | 1.123 | 3.51× |
| 1 | 10 | 0.393 | 2.218 | 5.64× |
| 10 | 1 | 0.047 | 0.212 | 4.53× |
| 10 | 5 | 0.218 | 1.293 | 5.93× |
| 10 | 10 | 0.398 | 2.186 | 5.49× |
| 100 | 1 | 0.048 | 0.240 | 4.95× |
| 100 | 5 | 0.202 | 1.315 | 6.51× |
| 100 | 10 | 0.404 | 2.296 | 5.69× |
| 1,000 | 1 | 0.055 | 0.275 | 5.01× |
| 1,000 | 5 | 0.202 | 1.537 | 7.59× |
| 1,000 | 10 | 0.413 | 2.964 | 7.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
