# KnowSureThing benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 28.15M | 0.035 | 28.71M | nan | — | — |
| 10,000 | 0.330 | 30.35M | 0.333 | 30.04M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.053 ms**; native kernel **0.052 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.439 | 0.256 | 3.91M | nan | — | — |
| 1,500 | 10 | 1.394 | 0.871 | 11.48M | nan | — | — |
| 1,500 | 100 | 5.538 | 7.989 | 12.52M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
