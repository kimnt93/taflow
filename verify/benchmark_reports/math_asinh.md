# MathAsinh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.32M | 0.015 | 68.13M | nan | — | — |
| 10,000 | 0.124 | 80.49M | 0.117 | 85.33M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.021 ms**; native kernel **0.020 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.310 | 0.176 | 5.67M | nan | — | — |
| 1,500 | 10 | 1.186 | 0.683 | 14.65M | nan | — | — |
| 1,500 | 100 | 3.571 | 2.805 | 35.65M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.63M | 16.06M | 1.00× | 1.23M | 1.59M | 1.00× | — |
| 2 | 20.04M | 21.33M | 1.33× | 1.11M | 1.51M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
