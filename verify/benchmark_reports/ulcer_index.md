# UlcerIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.70M | 0.039 | 25.95M | nan | — | — |
| 10,000 | 0.378 | 26.45M | 0.381 | 26.21M | nan | — | — |
| 100,000 | 3.844 | 26.02M | 3.733 | 26.79M | nan | — | — |
| 1,000,000 | 37.891 | 26.39M | 37.649 | 26.56M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.768 ms**; native kernel **4.154 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.299 | 0.205 | 4.89M | nan | — | — |
| 100,000 | 10 | 1.262 | 1.017 | 9.83M | nan | — | — |
| 100,000 | 1,000 | 54.988 | 37.870 | 26.41M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 25.23M | 25.23M | 1.00× | 2.55M | 3.21M | 1.00× | — |
| 2 | 47.34M | 47.15M | 1.87× | 2.71M | 2.53M | 0.79× | — |
| 4 | 77.14M | 77.20M | 3.06× | 2.87M | 2.80M | 0.87× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
