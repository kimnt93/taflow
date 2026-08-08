# HigherHigh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 184.02M | 0.004 | 260.56M | nan | — | — |
| 10,000 | 0.034 | 298.00M | 0.029 | 340.75M | nan | — | — |
| 100,000 | 0.293 | 341.51M | 0.268 | 373.82M | nan | — | — |
| 1,000,000 | 3.759 | 266.01M | 3.209 | 311.63M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.303 ms**; native kernel **0.270 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.258 | 0.183 | 5.48M | nan | — | — |
| 100,000 | 10 | 1.462 | 0.744 | 13.45M | nan | — | — |
| 100,000 | 1,000 | 5.224 | 4.080 | 245.08M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 203.78M | 219.32M | 1.00× | 2.96M | 3.25M | 1.00× | — |
| 2 | 406.15M | 546.49M | 2.49× | 3.29M | 3.50M | 1.08× | — |
| 4 | 508.45M | 861.84M | 3.93× | 3.30M | 3.53M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
