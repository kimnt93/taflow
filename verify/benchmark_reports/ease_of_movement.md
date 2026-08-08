# EaseOfMovement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 171.36M | 0.004 | 242.66M | nan | — | — |
| 10,000 | 0.037 | 273.25M | 0.033 | 307.25M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.361 | 0.356 | 2.81M | nan | — | — |
| 1,500 | 10 | 2.593 | 0.997 | 10.03M | nan | — | — |
| 1,500 | 100 | 3.756 | 2.507 | 39.89M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
