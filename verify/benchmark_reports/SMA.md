# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 234.43M | 0.003 | 321.26M | 0.037 | 8.76× | 12.00× |
| 10,000 | 0.026 | 385.32M | 0.023 | 428.66M | 0.051 | 1.97× | 2.19× |
| 100,000 | 0.238 | 420.70M | 0.195 | 512.90M | 0.231 | 0.97× | 1.19× |
| 1,000,000 | 2.553 | 391.66M | 1.992 | 502.04M | 1.968 | 0.77× | 0.99× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.223 ms**; native kernel **0.194 ms**; TA-Lib 0.217 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.223 | 0.145 | 6.89M | 234.644 | 1616.96× | 223.68× |
| 100,000 | 10 | 0.907 | 0.659 | 15.17M | 227.842 | 345.68× | 49.59× |
| 100,000 | 1,000 | 4.765 | 3.550 | 281.71M | 228.933 | 64.49× | 9.30× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 285.78M | 358.82M | 1.00× | 3.43M | 3.70M | 1.00× | 331.05M |
| 2 | 575.44M | 565.35M | 1.58× | 2.97M | 3.81M | 1.03× | 311.88M |
| 4 | 703.19M | 1.10G | 3.07× | 3.17M | 3.42M | 0.92× | 327.32M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
