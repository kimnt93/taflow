# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.50M | 0.012 | 80.48M | 0.038 | 2.39× | 3.03× |
| 10,000 | 0.152 | 65.86M | 0.153 | 65.25M | 0.238 | 1.57× | 1.55× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.020 ms**; native kernel **0.018 ms**; TA-Lib 0.044 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.322 | 0.181 | 5.52M | 46.762 | 258.10× | 151.23× |
| 1,500 | 10 | 1.264 | 0.728 | 13.73M | 46.481 | 63.81× | 36.32× |
| 1,500 | 100 | 4.446 | 3.098 | 32.28M | 46.037 | 14.86× | 8.89× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.30M | 6.81M | 1.00× | 1.09M | 1.47M | 1.00× | 8.62M |
| 2 | 14.79M | 21.16M | 3.11× | 1.34M | 1.51M | 1.03× | 9.10M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
