# ChaikinVolatility benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.62M | 0.007 | 142.89M | nan | — | — |
| 10,000 | 0.063 | 158.32M | 0.059 | 170.18M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.329 | 0.237 | 4.21M | nan | — | — |
| 1,500 | 10 | 1.662 | 0.822 | 12.17M | nan | — | — |
| 1,500 | 100 | 3.350 | 2.479 | 40.34M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
