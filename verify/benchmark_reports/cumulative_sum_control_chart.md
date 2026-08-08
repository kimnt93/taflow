# CumulativeSumControlChart benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 172.34M | 0.005 | 220.10M | nan | — | — |
| 10,000 | 0.042 | 237.60M | 0.040 | 253.13M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.594 | 0.174 | 5.76M | nan | — | — |
| 1,500 | 10 | 1.012 | 0.533 | 18.76M | nan | — | — |
| 1,500 | 100 | 2.508 | 1.972 | 50.71M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
