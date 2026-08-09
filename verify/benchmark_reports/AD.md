# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.91M | 0.006 | 166.22M | 0.030 | 3.70× | 4.96× |
| 10,000 | 0.022 | 454.90M | 0.018 | 544.38M | 0.040 | 1.82× | 2.18× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**; TA-Lib 0.029 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.477 | 0.264 | 3.79M | 31.002 | 117.46× | 100.83× |
| 1,500 | 10 | 3.795 | 1.137 | 8.80M | 30.116 | 26.49× | 23.60× |
| 1,500 | 100 | 4.830 | 2.634 | 37.96M | 30.128 | 11.44× | 10.31× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.94M | 14.12M | 1.00× | 1.07M | 934.14K | 1.00× | 8.44M |
| 2 | 16.46M | 12.24M | 0.87× | 1.03M | 1.45M | 1.55× | 9.96M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
