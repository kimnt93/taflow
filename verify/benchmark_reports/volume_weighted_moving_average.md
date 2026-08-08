# VolumeWeightedMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.71M | 0.018 | 55.64M | nan | — | — |
| 10,000 | 0.169 | 59.27M | 0.167 | 59.97M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.028 ms**; native kernel **0.026 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.345 | 0.245 | 4.07M | nan | — | — |
| 1,500 | 10 | 1.733 | 0.948 | 10.55M | nan | — | — |
| 1,500 | 100 | 4.372 | 3.474 | 28.79M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
