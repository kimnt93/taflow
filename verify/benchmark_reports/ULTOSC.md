# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.44M | 0.018 | 56.97M | 0.051 | 2.73× | 2.91× |
| 10,000 | 0.141 | 70.78M | 0.133 | 75.31M | 0.177 | 1.25× | 1.33× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.025 ms**; native kernel **0.024 ms**; TA-Lib 0.056 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.323 | 0.255 | 3.92M | 57.912 | 227.26× | 136.05× |
| 1,500 | 10 | 2.163 | 1.152 | 8.68M | 57.089 | 49.56× | 32.26× |
| 1,500 | 100 | 5.662 | 3.796 | 26.34M | 58.820 | 15.49× | 10.00× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.71M | 13.81M | 1.00× | 1.14M | 1.02M | 1.00× | 6.31M |
| 2 | 12.19M | 20.15M | 1.46× | 1.46M | 1.34M | 1.32× | 8.85M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
