# RollingArgmin benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 178.51M | 0.005 | 214.60M | 0.034 | 6.09× | 7.32× |
| 10,000 | 0.051 | 195.43M | 0.049 | 204.91M | 0.097 | 1.90× | 2.00× |
| 100,000 | 0.524 | 190.81M | 0.508 | 196.73M | 0.695 | 1.33× | 1.37× |
| 1,000,000 | 5.356 | 186.70M | 5.086 | 196.60M | 6.530 | 1.22× | 1.28× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.521 ms**; native kernel **0.500 ms**; TA-Lib 0.701 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.222 | 0.158 | 6.33M | 696.103 | 4406.85× | 181.86× |
| 100,000 | 10 | 1.108 | 0.588 | 17.01M | 685.093 | 1165.10× | 49.05× |
| 100,000 | 1,000 | 15.572 | 15.145 | 66.03M | 699.417 | 46.18× | 2.32× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 150.28M | 160.78M | 1.00× | 3.31M | 3.07M | 1.00× | 122.40M |
| 2 | 297.46M | 304.06M | 1.89× | 3.05M | 3.62M | 1.18× | 115.39M |
| 4 | 418.75M | 577.07M | 3.59× | 3.18M | 3.42M | 1.11× | 119.21M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
