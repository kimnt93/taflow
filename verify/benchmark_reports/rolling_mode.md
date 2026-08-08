# RollingMode benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.27M | 0.027 | 36.37M | nan | — | — |
| 10,000 | 0.266 | 37.59M | 0.258 | 38.81M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.042 ms**; native kernel **0.040 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.319 | 0.215 | 4.65M | nan | — | — |
| 1,500 | 10 | 1.344 | 0.780 | 12.81M | nan | — | — |
| 1,500 | 100 | 5.020 | 5.166 | 19.36M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
