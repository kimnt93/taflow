# LowerLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 182.11M | 0.004 | 253.68M | nan | — | — |
| 10,000 | 0.034 | 290.06M | 0.031 | 324.31M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.315 | 0.222 | 4.50M | nan | — | — |
| 1,500 | 10 | 1.620 | 0.761 | 13.15M | nan | — | — |
| 1,500 | 100 | 3.430 | 2.185 | 45.77M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
