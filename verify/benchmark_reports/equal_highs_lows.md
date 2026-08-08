# EqualHighsLows benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.55M | 0.047 | 21.08M | nan | — | — |
| 10,000 | 0.464 | 21.55M | 0.457 | 21.87M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.077 ms**; native kernel **0.070 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.442 | 0.337 | 2.96M | nan | — | — |
| 1,500 | 10 | 2.449 | 1.369 | 7.30M | nan | — | — |
| 1,500 | 100 | 7.787 | 6.687 | 14.95M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
