# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.33M | 0.009 | 111.78M | 0.043 | 4.21× | 4.78× |
| 10,000 | 0.105 | 95.34M | 0.098 | 101.57M | 0.152 | 1.45× | 1.54× |
| 100,000 | 1.036 | 96.54M | 0.981 | 101.89M | 1.171 | 1.13× | 1.19× |
| 1,000,000 | 16.078 | 62.20M | 10.453 | 95.66M | 11.372 | 0.71× | 1.09× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.021 ms**; native kernel **0.995 ms**; TA-Lib 1.157 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.348 | 0.302 | 3.31M | 1144.823 | 3784.55× | 106.36× |
| 100,000 | 10 | 1.588 | 1.181 | 8.46M | 1179.429 | 998.37× | 27.73× |
| 100,000 | 1,000 | 75.766 | 67.373 | 14.84M | 1170.786 | 17.38× | 0.65× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 76.09M | 88.78M | 1.00× | 2.15M | 2.54M | 1.00× | 74.31M |
| 2 | 137.01M | 172.75M | 1.95× | 1.94M | 2.60M | 1.02× | 73.00M |
| 4 | 186.13M | 291.50M | 3.28× | 1.75M | 1.91M | 0.75× | 73.35M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
