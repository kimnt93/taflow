# TomDeMarkSequential benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.56M | 0.006 | 169.76M | nan | — | — |
| 10,000 | 0.061 | 163.30M | 0.058 | 171.95M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.009 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.347 | 0.214 | 4.67M | nan | — | — |
| 1,500 | 10 | 0.968 | 0.618 | 16.19M | nan | — | — |
| 1,500 | 100 | 3.014 | 1.954 | 51.18M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
