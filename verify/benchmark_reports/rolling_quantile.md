# RollingQuantile benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.43M | 0.050 | 20.14M | nan | — | — |
| 10,000 | 0.533 | 18.77M | 0.539 | 18.54M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.078 ms**; native kernel **0.078 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.372 | 0.261 | 3.84M | nan | — | — |
| 1,500 | 10 | 1.780 | 1.146 | 8.73M | nan | — | — |
| 1,500 | 100 | 8.081 | 7.809 | 12.81M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
