# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.95M | 0.012 | 82.98M | 0.052 | 4.02× | 4.33× |
| 10,000 | 0.108 | 92.21M | 0.102 | 98.09M | 0.160 | 1.48× | 1.57× |
| 100,000 | 1.012 | 98.83M | 0.959 | 104.29M | 1.197 | 1.18× | 1.25× |
| 1,000,000 | 19.235 | 51.99M | 14.026 | 71.30M | 12.235 | 0.64× | 0.87× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.008 ms**; native kernel **0.950 ms**; TA-Lib 1.185 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.383 | 0.348 | 2.88M | 1171.399 | 3368.16× | 122.21× |
| 100,000 | 10 | 1.985 | 1.813 | 5.52M | 1172.254 | 646.73× | 23.60× |
| 100,000 | 1,000 | 90.480 | 83.022 | 12.05M | 1218.994 | 14.68× | 0.62× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 80.26M | 89.26M | 1.00× | 1.71M | 1.93M | 1.00× | 70.90M |
| 2 | 140.40M | 170.56M | 1.91× | 1.64M | 2.03M | 1.05× | 69.82M |
| 4 | 200.35M | 290.98M | 3.26× | 1.74M | 1.70M | 0.88× | 69.05M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
