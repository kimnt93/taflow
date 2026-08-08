# OpeningRange benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.64M | 0.008 | 120.67M | nan | — | — |
| 10,000 | 0.076 | 132.04M | 0.067 | 150.12M | nan | — | — |
| 100,000 | 0.703 | 142.28M | 0.637 | 156.91M | nan | — | — |
| 1,000,000 | 16.768 | 59.64M | 7.395 | 135.23M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.717 ms**; native kernel **0.645 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.426 | 0.395 | 2.53M | nan | — | — |
| 100,000 | 10 | 1.725 | 1.167 | 8.57M | nan | — | — |
| 100,000 | 1,000 | 9.320 | 9.415 | 106.21M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 103.95M | 125.58M | 1.00× | 1.74M | 1.77M | 1.00× | — |
| 2 | 104.08M | 117.17M | 0.93× | 1.97M | 1.92M | 1.09× | — |
| 4 | 101.60M | 115.13M | 0.92× | 1.85M | 1.83M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
