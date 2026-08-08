# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 153.88M | 0.005 | 185.09M | 0.035 | 5.44× | 6.54× |
| 10,000 | 0.050 | 199.93M | 0.051 | 196.12M | 0.099 | 1.98× | 1.94× |
| 100,000 | 0.443 | 225.54M | 0.419 | 238.56M | 0.693 | 1.56× | 1.65× |
| 1,000,000 | 5.903 | 169.41M | 5.625 | 177.77M | 6.880 | 1.17× | 1.22× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.435 ms**; native kernel **0.425 ms**; TA-Lib 0.702 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.273 | 0.186 | 5.39M | 706.860 | 3809.28× | 159.80× |
| 100,000 | 10 | 1.078 | 0.742 | 13.47M | 729.627 | 982.88× | 40.86× |
| 100,000 | 1,000 | 26.183 | 29.700 | 33.67M | 691.165 | 23.27× | 1.21× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 169.57M | 196.15M | 1.00× | 2.95M | 3.50M | 1.00× | 120.80M |
| 2 | 320.00M | 357.02M | 1.82× | 2.96M | 3.26M | 0.93× | 115.16M |
| 4 | 349.12M | 577.98M | 2.95× | 2.81M | 3.20M | 0.92× | 121.92M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
