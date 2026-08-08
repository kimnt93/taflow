# YangZhang benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.43M | 0.051 | 19.50M | nan | — | — |
| 10,000 | 0.501 | 19.96M | 0.504 | 19.82M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.079 ms**; native kernel **0.078 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.465 | 0.335 | 2.99M | nan | — | — |
| 1,500 | 10 | 3.033 | 1.651 | 6.06M | nan | — | — |
| 1,500 | 100 | 8.858 | 7.181 | 13.93M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
