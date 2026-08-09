# ExponentiallyWeightedVariance benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 162.99M | 0.005 | 193.17M | nan | — | — |
| 10,000 | 0.044 | 229.36M | 0.041 | 246.47M | nan | — | — |
| 100,000 | 0.426 | 234.80M | 0.416 | 240.32M | nan | — | — |
| 1,000,000 | 4.400 | 227.28M | 4.043 | 247.34M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.424 ms**; native kernel **0.404 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.198 | 0.145 | 6.90M | nan | — | — |
| 100,000 | 10 | 0.845 | 0.502 | 19.93M | nan | — | — |
| 100,000 | 1,000 | 5.873 | 5.145 | 194.35M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 166.69M | 163.55M | 1.00× | 2.73M | 3.50M | 1.00× | — |
| 2 | 181.91M | 173.76M | 1.06× | 3.76M | 3.86M | 1.10× | — |
| 4 | 191.03M | 161.22M | 0.99× | 3.57M | 3.71M | 1.06× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
