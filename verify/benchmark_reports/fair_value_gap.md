# FairValueGap benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 64.43M | 0.013 | 79.46M | nan | — | — |
| 10,000 | 0.120 | 83.58M | 0.108 | 92.24M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.022 ms**; native kernel **0.018 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.597 | 0.350 | 2.86M | nan | — | — |
| 1,500 | 10 | 2.722 | 1.247 | 8.02M | nan | — | — |
| 1,500 | 100 | 5.079 | 3.533 | 28.30M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
