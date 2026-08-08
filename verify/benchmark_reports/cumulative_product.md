# CumulativeProduct benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 223.79M | 0.003 | 294.12M | nan | — | — |
| 10,000 | 0.029 | 340.82M | 0.026 | 380.22M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.005 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.279 | 0.183 | 5.47M | nan | — | — |
| 1,500 | 10 | 1.046 | 0.540 | 18.52M | nan | — | — |
| 1,500 | 100 | 4.129 | 3.089 | 32.37M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
