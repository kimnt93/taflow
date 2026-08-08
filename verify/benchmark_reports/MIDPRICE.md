# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.24M | 0.008 | 118.18M | 0.041 | 4.88× | 4.87× |
| 10,000 | 0.053 | 190.18M | 0.045 | 220.56M | 0.110 | 2.09× | 2.42× |
| 100,000 | 0.443 | 225.86M | 0.407 | 245.94M | 0.732 | 1.65× | 1.80× |
| 1,000,000 | 5.900 | 169.49M | 5.426 | 184.29M | 6.943 | 1.18× | 1.28× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.479 ms**; native kernel **0.441 ms**; TA-Lib 0.752 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.308 | 0.253 | 3.95M | 749.680 | 2958.43× | 124.38× |
| 100,000 | 10 | 1.683 | 1.025 | 9.76M | 795.171 | 775.87× | 29.05× |
| 100,000 | 1,000 | 27.307 | 22.780 | 43.90M | 706.280 | 31.00× | 1.68× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 155.80M | 153.24M | 1.00× | 2.48M | 2.54M | 1.00× | 103.94M |
| 2 | 300.56M | 352.01M | 2.30× | 2.39M | 2.97M | 1.17× | 112.87M |
| 4 | 302.41M | 432.25M | 2.82× | 2.30M | 2.57M | 1.01× | 115.24M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
