# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.99M | 0.009 | 105.35M | 0.032 | 2.96× | 3.36× |
| 10,000 | 0.076 | 130.91M | 0.073 | 137.19M | 0.090 | 1.18× | 1.24× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.013 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.311 | 0.181 | 5.53M | 36.033 | 199.28× | 135.92× |
| 1,500 | 10 | 1.748 | 0.650 | 15.39M | 35.846 | 55.18× | 40.18× |
| 1,500 | 100 | 3.559 | 2.413 | 41.44M | 36.438 | 15.10× | 11.59× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.51M | 13.41M | 1.00× | 906.79K | 1.09M | 1.00× | 8.90M |
| 2 | 17.41M | 19.94M | 1.49× | 1.28M | 1.53M | 1.40× | 9.34M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
