# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 60.18M | 0.016 | 63.46M | 0.044 | 2.63× | 2.78× |
| 10,000 | 0.165 | 60.47M | 0.157 | 63.54M | 0.145 | 0.88× | 0.92× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.023 ms**; native kernel **0.022 ms**; TA-Lib 0.046 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.372 | 0.299 | 3.34M | 49.901 | 166.63× | 179.18× |
| 1,500 | 10 | 1.632 | 1.408 | 7.10M | 50.638 | 35.95× | 27.13× |
| 1,500 | 100 | 7.102 | 5.560 | 17.98M | 50.086 | 9.01× | 6.90× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.84M | 13.41M | 1.00× | 1.16M | 1.18M | 1.00× | 5.75M |
| 2 | 18.74M | 12.29M | 0.92× | 1.31M | 1.35M | 1.14× | 7.52M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
