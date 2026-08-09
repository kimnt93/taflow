# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 204.86M | 0.003 | 297.36M | 0.031 | 6.28× | 9.11× |
| 10,000 | 0.010 | 956.50M | 0.007 | 1.45G | 0.034 | 3.27× | 4.95× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.005 ms**; TA-Lib 0.031 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.351 | 0.190 | 5.27M | 30.889 | 162.75× | 156.51× |
| 1,500 | 10 | 1.598 | 0.762 | 13.13M | 29.416 | 38.62× | 37.16× |
| 1,500 | 100 | 3.290 | 1.810 | 55.26M | 31.647 | 17.49× | 16.14× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.69M | 20.05M | 1.00× | 1.28M | 1.17M | 1.00× | 10.02M |
| 2 | 19.87M | 16.79M | 0.84× | 1.40M | 1.75M | 1.49× | 11.67M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
