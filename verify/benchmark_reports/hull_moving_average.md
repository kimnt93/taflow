# HullMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.55M | 0.039 | 25.49M | nan | — | — |
| 10,000 | 0.374 | 26.75M | 0.370 | 26.99M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.097 ms**; native kernel **0.068 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.343 | 0.248 | 4.04M | nan | — | — |
| 1,500 | 10 | 1.526 | 0.969 | 10.32M | nan | — | — |
| 1,500 | 100 | 6.414 | 5.595 | 17.87M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
