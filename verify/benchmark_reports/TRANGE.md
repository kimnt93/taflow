# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 276.22M | 0.002 | 481.87M | 0.027 | 7.34× | 12.80× |
| 10,000 | 0.017 | 589.35M | 0.012 | 828.38M | 0.033 | 1.96× | 2.75× |
| 100,000 | 0.131 | 764.71M | 0.109 | 919.94M | 0.083 | 0.64× | 0.77× |
| 1,000,000 | 2.027 | 493.42M | 1.561 | 640.44M | 1.255 | 0.62× | 0.80× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.133 ms**; native kernel **0.108 ms**; TA-Lib 0.085 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.324 | 0.233 | 4.30M | 95.448 | 410.20× | 108.17× |
| 100,000 | 10 | 1.788 | 0.873 | 11.45M | 84.608 | 96.88× | 28.82× |
| 100,000 | 1,000 | 5.135 | 3.414 | 292.92M | 85.578 | 25.07× | 7.80× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 269.22M | 361.95M | 1.00× | 2.42M | 3.07M | 1.00× | 433.47M |
| 2 | 676.82M | 798.11M | 2.21× | 2.35M | 3.04M | 0.99× | 554.76M |
| 4 | 713.32M | 980.19M | 2.71× | 2.35M | 2.69M | 0.88× | 516.02M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
