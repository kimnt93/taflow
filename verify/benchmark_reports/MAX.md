# RollingMax benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 217.55M | 0.004 | 279.81M | 0.035 | 7.59× | 9.77× |
| 10,000 | 0.036 | 277.66M | 0.032 | 312.63M | 0.079 | 2.20× | 2.47× |
| 100,000 | 0.388 | 257.73M | 0.367 | 272.84M | 0.500 | 1.29× | 1.37× |
| 1,000,000 | 5.289 | 189.05M | 4.809 | 207.94M | 4.876 | 0.92× | 1.01× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.382 ms**; native kernel **0.364 ms**; TA-Lib 0.509 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.252 | 0.210 | 4.77M | 504.996 | 2408.31× | 148.07× |
| 100,000 | 10 | 1.181 | 0.708 | 14.13M | 512.890 | 724.72× | 42.68× |
| 100,000 | 1,000 | 21.591 | 14.937 | 66.95M | 502.715 | 33.66× | 2.40× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 203.58M | 191.58M | 1.00× | 3.02M | 3.61M | 1.00× | 166.07M |
| 2 | 343.90M | 351.91M | 1.84× | 2.89M | 3.51M | 0.97× | 161.34M |
| 4 | 434.06M | 587.98M | 3.07× | 2.87M | 3.61M | 1.00× | 162.29M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
