# KeltnerChannels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.91M | 0.010 | 103.65M | nan | — | — |
| 10,000 | 0.091 | 110.32M | 0.084 | 119.41M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.014 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.396 | 0.302 | 3.32M | nan | — | — |
| 1,500 | 10 | 2.101 | 1.009 | 9.91M | nan | — | — |
| 1,500 | 100 | 4.561 | 2.971 | 33.66M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
