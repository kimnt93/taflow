# RollingRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 60.42M | 0.015 | 65.07M | nan | — | — |
| 10,000 | 0.144 | 69.44M | 0.138 | 72.23M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.026 ms**; native kernel **0.024 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.544 | 0.381 | 2.63M | nan | — | — |
| 1,500 | 10 | 2.246 | 1.530 | 6.54M | nan | — | — |
| 1,500 | 100 | 6.157 | 5.677 | 17.61M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
