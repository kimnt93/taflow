# MathCot benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.32M | 0.018 | 54.52M | nan | — | — |
| 10,000 | 0.618 | 16.19M | 0.201 | 49.68M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.091 ms**; native kernel **0.030 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.384 | 0.202 | 4.96M | nan | — | — |
| 1,500 | 10 | 1.898 | 0.882 | 11.34M | nan | — | — |
| 1,500 | 100 | 7.977 | 4.039 | 24.76M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
