# CumulativeCount benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 246.64M | 0.003 | 338.93M | nan | — | — |
| 10,000 | 0.026 | 378.54M | 0.023 | 438.13M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.005 ms**; native kernel **0.004 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.289 | 0.178 | 5.60M | nan | — | — |
| 1,500 | 10 | 1.044 | 0.535 | 18.68M | nan | — | — |
| 1,500 | 100 | 2.454 | 1.781 | 56.15M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
