# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.91M | 0.011 | 87.94M | 0.041 | 3.26× | 3.63× |
| 10,000 | 0.120 | 83.08M | 0.110 | 91.20M | 0.096 | 0.80× | 0.88× |
| 100,000 | 1.044 | 95.77M | 1.052 | 95.01M | 0.630 | 0.60× | 0.60× |
| 1,000,000 | 10.749 | 93.04M | 10.589 | 94.44M | 6.252 | 0.58× | 0.59× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.053 ms**; native kernel **0.999 ms**; TA-Lib 0.600 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.343 | 0.216 | 4.63M | 639.059 | 2956.84× | 168.44× |
| 100,000 | 10 | 1.682 | 0.852 | 11.74M | 606.539 | 711.92× | 39.62× |
| 100,000 | 1,000 | 12.824 | 11.536 | 86.69M | 605.400 | 52.48× | 3.45× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 93.54M | 103.32M | 1.00× | 1.88M | 3.37M | 1.00× | 142.99M |
| 2 | 181.70M | 193.70M | 1.87× | 2.46M | 2.94M | 0.87× | 138.49M |
| 4 | 254.87M | 320.02M | 3.10× | 2.18M | 2.69M | 0.80× | 139.40M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
