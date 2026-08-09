# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.24M | 0.007 | 141.75M | 0.040 | 5.00× | 5.66× |
| 10,000 | 0.044 | 227.28M | 0.042 | 235.96M | 0.079 | 1.80× | 1.87× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.009 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.301 | 0.176 | 5.68M | 42.760 | 242.93× | 190.97× |
| 1,500 | 10 | 1.170 | 0.645 | 15.51M | 43.275 | 67.13× | 66.01× |
| 1,500 | 100 | 3.511 | 2.286 | 43.74M | 42.270 | 18.49× | 15.81× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.54M | 16.58M | 1.00× | 983.62K | 1.10M | 1.00× | 8.91M |
| 2 | 17.95M | 19.39M | 1.17× | 1.43M | 1.66M | 1.51× | 9.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
