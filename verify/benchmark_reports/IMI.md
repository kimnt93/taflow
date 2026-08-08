# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.51M | 0.006 | 172.54M | 0.082 | 12.69× | 14.08× |
| 10,000 | 0.051 | 194.78M | 0.049 | 204.68M | 0.602 | 11.73× | 12.32× |
| 100,000 | 0.500 | 199.86M | 0.462 | 216.54M | 5.896 | 11.78× | 12.77× |
| 1,000,000 | 5.290 | 189.03M | 4.757 | 210.22M | 58.097 | 10.98× | 12.21× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.489 ms**; native kernel **0.469 ms**; TA-Lib 5.856 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.257 | 0.191 | 5.24M | 5791.768 | 30331.54× | 161.79× |
| 100,000 | 10 | 0.869 | 0.761 | 13.14M | 5954.626 | 7824.59× | 41.26× |
| 100,000 | 1,000 | 6.978 | 6.297 | 158.81M | 5945.399 | 944.16× | 13.34× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 151.78M | 152.42M | 1.00× | 2.95M | 2.92M | 1.00× | 16.55M |
| 2 | 278.93M | 323.89M | 2.13× | 2.86M | 3.74M | 1.28× | 16.15M |
| 4 | 380.60M | 531.86M | 3.49× | 2.71M | 2.83M | 0.97× | 15.96M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
