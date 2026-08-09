# DetrendedPriceOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.00M | 0.008 | 126.43M | nan | — | — |
| 10,000 | 0.058 | 172.56M | 0.054 | 186.09M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.011 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.244 | 0.171 | 5.84M | nan | — | — |
| 1,500 | 10 | 1.007 | 0.598 | 16.71M | nan | — | — |
| 1,500 | 100 | 2.219 | 1.760 | 56.82M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 12.84M | 18.91M | 1.00× | 1.22M | 1.60M | 1.00× | — |
| 2 | 15.31M | 20.29M | 1.07× | 1.54M | 1.64M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
