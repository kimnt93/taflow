# Crossunder benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 178.42M | 0.004 | 240.89M | nan | — | — |
| 10,000 | 0.037 | 269.95M | 0.036 | 279.13M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.320 | 0.221 | 4.53M | nan | — | — |
| 1,500 | 10 | 2.889 | 0.790 | 12.66M | nan | — | — |
| 1,500 | 100 | 3.428 | 26.520 | 3.77M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
