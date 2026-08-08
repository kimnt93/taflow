# RollingWinsorize benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.99M | 0.054 | 18.36M | nan | — | — |
| 10,000 | 0.573 | 17.45M | 0.574 | 17.42M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.085 ms**; native kernel **0.082 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.350 | 0.255 | 3.92M | nan | — | — |
| 1,500 | 10 | 1.658 | 1.110 | 9.01M | nan | — | — |
| 1,500 | 100 | 7.817 | 8.598 | 11.63M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
