# Falling benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.34M | 0.007 | 138.78M | nan | — | — |
| 10,000 | 0.052 | 193.44M | 0.049 | 205.07M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.289 | 0.176 | 5.69M | nan | — | — |
| 1,500 | 10 | 1.000 | 0.582 | 17.19M | nan | — | — |
| 1,500 | 100 | 11.421 | 1.681 | 59.49M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 13.18M | 16.79M | 1.00× | 971.73K | 1.54M | 1.00× | — |
| 2 | 15.90M | 18.47M | 1.10× | 1.23M | 1.47M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
