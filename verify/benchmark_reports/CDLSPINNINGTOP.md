# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 169.05M | 0.004 | 244.08M | 0.031 | 5.18× | 7.48× |
| 10,000 | 0.102 | 97.91M | 0.092 | 108.49M | 0.119 | 1.16× | 1.29× |
| 100,000 | 0.993 | 100.72M | 0.966 | 103.49M | 0.974 | 0.98× | 1.01× |
| 1,000,000 | 10.541 | 94.87M | 10.062 | 99.38M | 9.731 | 0.92× | 0.97× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.992 ms**; native kernel **0.977 ms**; TA-Lib 0.989 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.333 | 0.269 | 3.71M | 986.185 | 3662.38× | 102.42× |
| 100,000 | 10 | 2.476 | 1.312 | 7.62M | 996.595 | 759.60× | 21.21× |
| 100,000 | 1,000 | 22.552 | 20.231 | 49.43M | 989.755 | 48.92× | 1.52× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 83.33M | 90.05M | 1.00× | 2.33M | 2.15M | 1.00× | 89.52M |
| 2 | 168.77M | 180.12M | 2.00× | 2.44M | 2.52M | 1.17× | 88.14M |
| 4 | 288.58M | 323.74M | 3.60× | 2.29M | 2.43M | 1.13× | 85.21M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
