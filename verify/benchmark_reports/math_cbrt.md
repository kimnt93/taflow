# MathCbrt benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.84M | 0.020 | 51.01M | nan | — | — |
| 10,000 | 0.188 | 53.09M | 0.193 | 51.72M | nan | — | — |
| 100,000 | 1.838 | 54.42M | 1.855 | 53.92M | nan | — | — |
| 1,000,000 | 18.821 | 53.13M | 19.319 | 51.76M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.840 ms**; native kernel **1.816 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.268 | 0.164 | 6.11M | nan | — | — |
| 100,000 | 10 | 1.117 | 0.713 | 14.03M | nan | — | — |
| 100,000 | 1,000 | 20.375 | 25.150 | 39.76M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 50.88M | 53.09M | 1.00× | 2.89M | 2.57M | 1.00× | — |
| 2 | 96.58M | 91.78M | 1.73× | 2.79M | 2.85M | 1.11× | — |
| 4 | 155.70M | 188.21M | 3.54× | 2.54M | 2.62M | 1.02× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
