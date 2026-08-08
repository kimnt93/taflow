# Supertrend benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.01M | 0.018 | 56.78M | nan | — | — |
| 10,000 | 0.189 | 53.04M | 0.173 | 57.67M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.030 ms**; native kernel **0.026 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.431 | 0.328 | 3.05M | nan | — | — |
| 1,500 | 10 | 2.313 | 1.141 | 8.76M | nan | — | — |
| 1,500 | 100 | 5.172 | 3.948 | 25.33M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
