# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 264.40M | 0.003 | 376.25M | 0.027 | 7.23× | 10.28× |
| 10,000 | 0.023 | 427.92M | 0.020 | 504.52M | 0.041 | 1.76× | 2.07× |
| 100,000 | 0.212 | 470.75M | 0.188 | 532.50M | 0.163 | 0.77× | 0.87× |
| 1,000,000 | 2.948 | 339.17M | 2.504 | 399.40M | 1.415 | 0.48× | 0.57× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.209 ms**; native kernel **0.191 ms**; TA-Lib 0.160 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.207 | 0.146 | 6.83M | 163.253 | 1115.53× | 170.10× |
| 100,000 | 10 | 0.924 | 0.734 | 13.63M | 158.636 | 216.15× | 34.33× |
| 100,000 | 1,000 | 4.676 | 3.304 | 302.64M | 160.503 | 48.57× | 8.13× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 264.77M | 338.47M | 1.00× | 3.16M | 3.64M | 1.00× | 382.10M |
| 2 | 437.32M | 621.50M | 1.84× | 3.09M | 3.52M | 0.97× | 387.52M |
| 4 | 454.41M | 762.11M | 2.25× | 3.27M | 3.55M | 0.97× | 387.53M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
