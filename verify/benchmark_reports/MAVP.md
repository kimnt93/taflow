# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.123 | 8.12M | 0.115 | 8.73M | 0.110 | 0.90× | 0.96× |
| 10,000 | 1.337 | 7.48M | 1.111 | 9.00M | 0.781 | 0.58× | 0.70× |
| 100,000 | 11.039 | 9.06M | 10.792 | 9.27M | 7.304 | 0.66× | 0.68× |
| 1,000,000 | 110.606 | 9.04M | 113.203 | 8.83M | 96.809 | 0.88× | 0.86× |

## Warm-up

Construct + canonical extend over 100,000 bars: **11.306 ms**; native kernel **11.376 ms**; TA-Lib 7.822 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.523 | 0.313 | 3.20M | 7313.307 | 23383.74× | 118.16× |
| 100,000 | 10 | 2.559 | 2.183 | 4.58M | 7944.661 | 3639.07× | 18.91× |
| 100,000 | 1,000 | 116.226 | 118.102 | 8.47M | 7492.407 | 63.44× | 1.05× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.59M | 9.45M | 1.00× | 1.62M | 2.09M | 1.00× | 12.24M |
| 2 | 17.73M | 18.24M | 1.93× | 1.70M | 2.05M | 0.98× | 12.34M |
| 4 | 34.59M | 34.32M | 3.63× | 1.54M | 1.94M | 0.93× | 11.94M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
