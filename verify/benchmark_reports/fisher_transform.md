# FisherTransform benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.63M | 0.053 | 18.92M | nan | — | — |
| 10,000 | 0.517 | 19.34M | 0.521 | 19.21M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.078 ms**; native kernel **0.078 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.686 | 0.256 | 3.91M | nan | — | — |
| 1,500 | 10 | 2.003 | 1.181 | 8.47M | nan | — | — |
| 1,500 | 100 | 7.653 | 21.009 | 4.76M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
