# GapDown benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 183.89M | 0.004 | 255.72M | nan | — | — |
| 10,000 | 0.033 | 298.79M | 0.029 | 341.72M | nan | — | — |
| 100,000 | 0.298 | 335.28M | 0.266 | 375.91M | nan | — | — |
| 1,000,000 | 3.563 | 280.65M | 3.162 | 316.27M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.285 ms**; native kernel **0.270 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.280 | 0.206 | 4.86M | nan | — | — |
| 100,000 | 10 | 1.463 | 0.709 | 14.10M | nan | — | — |
| 100,000 | 1,000 | 5.235 | 6.175 | 161.95M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 233.70M | 313.67M | 1.00× | 3.01M | 3.29M | 1.00× | — |
| 2 | 413.66M | 574.92M | 1.83× | 3.31M | 3.36M | 1.02× | — |
| 4 | 612.78M | 764.45M | 2.44× | 3.30M | 3.56M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
