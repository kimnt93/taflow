# RollingAlpha benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.39M | 0.058 | 17.13M | nan | — | — |
| 10,000 | 0.589 | 16.98M | 0.570 | 17.56M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.087 ms**; native kernel **0.090 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.368 | 0.267 | 3.75M | nan | — | — |
| 1,500 | 10 | 2.086 | 1.217 | 8.22M | nan | — | — |
| 1,500 | 100 | 8.259 | 7.200 | 13.89M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
