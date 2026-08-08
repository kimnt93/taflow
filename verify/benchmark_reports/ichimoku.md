# Ichimoku benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.128 | 7.83M | 0.127 | 7.89M | nan | — | — |
| 10,000 | 1.247 | 8.02M | 1.252 | 7.99M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.190 ms**; native kernel **0.192 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.546 | 0.449 | 2.23M | nan | — | — |
| 1,500 | 10 | 6.933 | 2.239 | 4.47M | nan | — | — |
| 1,500 | 100 | 16.012 | 14.359 | 6.96M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
