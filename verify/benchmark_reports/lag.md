# Lag benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 184.93M | 0.004 | 233.10M | nan | — | — |
| 10,000 | 0.036 | 274.38M | 0.033 | 301.20M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.285 | 0.188 | 5.33M | nan | — | — |
| 1,500 | 10 | 1.108 | 0.555 | 18.03M | nan | — | — |
| 1,500 | 100 | 2.757 | 1.831 | 54.61M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
