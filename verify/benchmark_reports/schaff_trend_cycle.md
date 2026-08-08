# SchaffTrendCycle benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 19.00M | 0.057 | 17.52M | nan | — | — |
| 10,000 | 0.654 | 15.28M | 0.633 | 15.80M | nan | — | — |
| 100,000 | 6.329 | 15.80M | 6.336 | 15.78M | nan | — | — |
| 1,000,000 | 73.458 | 13.61M | 63.249 | 15.81M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **6.310 ms**; native kernel **6.488 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.588 | 0.499 | 2.00M | nan | — | — |
| 100,000 | 10 | 4.076 | 2.941 | 3.40M | nan | — | — |
| 100,000 | 1,000 | 250.964 | 194.555 | 5.14M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 15.05M | 15.62M | 1.00× | 1.46M | 1.53M | 1.00× | — |
| 2 | 26.76M | 29.17M | 1.87× | 1.54M | 1.48M | 0.97× | — |
| 4 | 35.45M | 43.93M | 2.81× | 1.15M | 1.04M | 0.68× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
