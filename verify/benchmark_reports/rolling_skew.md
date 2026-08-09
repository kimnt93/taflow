# RollingSkew benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 28.07M | 0.036 | 27.65M | nan | — | — |
| 10,000 | 0.324 | 30.88M | 0.313 | 31.95M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.057 ms**; native kernel **0.055 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.266 | 0.200 | 4.99M | nan | — | — |
| 1,500 | 10 | 1.265 | 1.063 | 9.40M | nan | — | — |
| 1,500 | 100 | 5.082 | 4.424 | 22.60M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.72M | 10.07M | 1.00× | 983.48K | 1.48M | 1.00× | — |
| 2 | 12.85M | 15.48M | 1.54× | 1.30M | 1.32M | 0.89× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
