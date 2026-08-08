# Falling benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.67M | 0.006 | 170.03M | nan | — | — |
| 10,000 | 0.052 | 191.39M | 0.048 | 208.17M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.283 | 0.192 | 5.21M | nan | — | — |
| 1,500 | 10 | 1.126 | 0.593 | 16.88M | nan | — | — |
| 1,500 | 100 | 2.780 | 2.141 | 46.71M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
