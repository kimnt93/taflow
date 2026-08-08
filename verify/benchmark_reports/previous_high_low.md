# PreviousHighLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.88M | 0.011 | 88.73M | nan | — | — |
| 10,000 | 0.108 | 92.92M | 0.096 | 104.45M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.022 ms**; native kernel **0.016 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.564 | 0.422 | 2.37M | nan | — | — |
| 1,500 | 10 | 2.015 | 1.064 | 9.40M | nan | — | — |
| 1,500 | 100 | 4.301 | 3.220 | 31.06M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
