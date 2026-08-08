# KnowSureThing benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.58M | 0.017 | 59.67M | nan | — | — |
| 10,000 | 0.157 | 63.76M | 0.151 | 66.21M | nan | — | — |
| 100,000 | 1.762 | 56.76M | 1.559 | 64.15M | nan | — | — |
| 1,000,000 | 20.222 | 49.45M | 15.335 | 65.21M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.515 ms**; native kernel **1.504 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.303 | 0.244 | 4.09M | nan | — | — |
| 100,000 | 10 | 1.539 | 1.101 | 9.09M | nan | — | — |
| 100,000 | 1,000 | 64.035 | 67.295 | 14.86M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 57.42M | 61.97M | 1.00× | 1.89M | 2.02M | 1.00× | — |
| 2 | 76.99M | 116.69M | 1.88× | 1.91M | 2.22M | 1.10× | — |
| 4 | 170.08M | 213.07M | 3.44× | 1.72M | 1.85M | 0.92× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
