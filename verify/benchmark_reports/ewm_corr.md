# ExponentiallyWeightedCorrelation benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.68M | 0.008 | 129.26M | nan | — | — |
| 10,000 | 0.062 | 160.34M | 0.058 | 171.48M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.010 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.307 | 0.212 | 4.71M | nan | — | — |
| 1,500 | 10 | 1.575 | 0.784 | 12.76M | nan | — | — |
| 1,500 | 100 | 3.508 | 2.349 | 42.57M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
