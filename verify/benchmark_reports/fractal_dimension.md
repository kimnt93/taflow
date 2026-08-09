# FractalDimension benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.10M | 0.054 | 18.49M | nan | — | — |
| 10,000 | 0.563 | 17.78M | 0.537 | 18.63M | nan | — | — |
| 100,000 | 5.382 | 18.58M | 5.569 | 17.96M | nan | — | — |
| 1,000,000 | 54.937 | 18.20M | 53.381 | 18.73M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **5.546 ms**; native kernel **5.670 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.265 | 0.206 | 4.86M | nan | — | — |
| 100,000 | 10 | 1.389 | 0.992 | 10.08M | nan | — | — |
| 100,000 | 1,000 | 56.685 | 53.066 | 18.84M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 17.14M | 16.79M | 1.00× | 2.91M | 3.32M | 1.00× | — |
| 2 | 31.74M | 32.74M | 1.95× | 2.73M | 2.84M | 0.86× | — |
| 4 | 54.07M | 48.63M | 2.90× | 2.57M | 2.61M | 0.78× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
