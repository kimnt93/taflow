# AnchoredVolumeWeightedAveragePrice benchmark (`anchored VWAP deviation bands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.22M | 0.008 | 126.43M | 1.332 | 113.49× | 168.36× |
| 10,000 | 0.075 | 133.92M | 0.064 | 155.81M | 13.244 | 177.36× | 206.35× |
| 100,000 | 0.717 | 139.42M | 0.738 | 135.58M | 133.337 | 185.90× | 180.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.116 | 1.18× |
| 1 | 5 | 0.270 | 0.453 | 1.67× |
| 1 | 10 | 0.418 | 0.914 | 2.18× |
| 10 | 1 | 0.045 | 0.105 | 2.36× |
| 10 | 5 | 0.203 | 0.508 | 2.50× |
| 10 | 10 | 0.400 | 1.053 | 2.63× |
| 100 | 1 | 0.045 | 0.231 | 5.14× |
| 100 | 5 | 0.202 | 1.153 | 5.71× |
| 100 | 10 | 0.422 | 2.434 | 5.77× |
| 1,000 | 1 | 0.055 | 1.538 | 27.72× |
| 1,000 | 5 | 0.229 | 8.432 | 36.90× |
| 1,000 | 10 | 0.447 | 14.659 | 32.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
