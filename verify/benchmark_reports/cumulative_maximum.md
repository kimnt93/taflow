# CumulativeMaximum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 168.48M | 0.005 | 196.12M | nan | — | — |
| 10,000 | 0.047 | 210.95M | 0.044 | 228.20M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.276 | 0.175 | 5.72M | nan | — | — |
| 1,500 | 10 | 1.025 | 0.520 | 19.23M | nan | — | — |
| 1,500 | 100 | 2.680 | 2.001 | 49.98M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
