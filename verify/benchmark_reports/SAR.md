# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.12M | 0.009 | 108.73M | 0.037 | 3.71× | 3.99× |
| 10,000 | 0.111 | 90.27M | 0.106 | 94.00M | 0.090 | 0.81× | 0.85× |
| 100,000 | 1.081 | 92.51M | 1.063 | 94.11M | 0.623 | 0.58× | 0.59× |
| 1,000,000 | 11.197 | 89.31M | 10.515 | 95.10M | 6.005 | 0.54× | 0.57× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.072 ms**; native kernel **1.079 ms**; TA-Lib 0.623 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.288 | 0.183 | 5.45M | 624.817 | 3406.66× | 174.09× |
| 100,000 | 10 | 0.904 | 0.767 | 13.04M | 621.655 | 810.44× | 41.08× |
| 100,000 | 1,000 | 13.183 | 12.676 | 78.89M | 647.926 | 51.11× | 2.95× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 79.33M | 84.71M | 1.00× | 2.33M | 2.54M | 1.00× | 121.22M |
| 2 | 151.37M | 158.25M | 1.87× | 2.62M | 3.34M | 1.32× | 126.81M |
| 4 | 255.32M | 308.71M | 3.64× | 2.33M | 2.69M | 1.06× | 128.18M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
