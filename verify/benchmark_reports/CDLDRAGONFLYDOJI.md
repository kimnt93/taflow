# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 180.89M | 0.004 | 284.27M | 0.036 | 6.49× | 10.20× |
| 10,000 | 0.047 | 211.07M | 0.042 | 239.82M | 0.099 | 2.08× | 2.37× |
| 100,000 | 0.560 | 178.43M | 0.542 | 184.48M | 0.726 | 1.30× | 1.34× |
| 1,000,000 | 6.048 | 165.35M | 5.829 | 171.55M | 7.410 | 1.23× | 1.27× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.571 ms**; native kernel **0.547 ms**; TA-Lib 0.730 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.338 | 0.278 | 3.60M | 717.223 | 2579.01× | 99.58× |
| 100,000 | 10 | 2.618 | 1.286 | 7.78M | 710.097 | 552.31× | 21.51× |
| 100,000 | 1,000 | 24.096 | 22.859 | 43.75M | 743.175 | 32.51× | 1.44× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 134.39M | 138.98M | 1.00× | 2.10M | 2.36M | 1.00× | 114.26M |
| 2 | 270.35M | 285.01M | 2.05× | 2.39M | 2.63M | 1.11× | 110.82M |
| 4 | 429.03M | 516.21M | 3.71× | 2.36M | 2.70M | 1.14× | 112.53M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
