# MathAtanh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.15M | 0.005 | 183.55M | nan | — | — |
| 10,000 | 0.050 | 200.35M | 0.047 | 211.97M | nan | — | — |
| 100,000 | 0.482 | 207.51M | 0.459 | 217.92M | nan | — | — |
| 1,000,000 | 5.481 | 182.44M | 5.122 | 195.24M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.487 ms**; native kernel **0.460 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.231 | 0.151 | 6.63M | nan | — | — |
| 100,000 | 10 | 0.973 | 0.601 | 16.65M | nan | — | — |
| 100,000 | 1,000 | 7.144 | 6.764 | 147.83M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 146.73M | 171.61M | 1.00× | 2.98M | 2.98M | 1.00× | — |
| 2 | 292.86M | 336.85M | 1.96× | 3.04M | 4.02M | 1.35× | — |
| 4 | 348.35M | 449.10M | 2.62× | 3.00M | 3.22M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
