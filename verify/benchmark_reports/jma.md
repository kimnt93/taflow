# JurikMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 44.89M | 0.021 | 47.86M | nan | — | — |
| 10,000 | 0.207 | 48.42M | 0.194 | 51.57M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.033 ms**; native kernel **0.030 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.302 | 0.193 | 5.17M | nan | — | — |
| 1,500 | 10 | 1.027 | 0.746 | 13.40M | nan | — | — |
| 1,500 | 100 | 3.995 | 3.497 | 28.60M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
