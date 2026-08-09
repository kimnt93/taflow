# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.76M | 0.012 | 85.92M | 0.046 | 3.46× | 3.92× |
| 10,000 | 0.127 | 78.60M | 0.127 | 79.03M | 0.146 | 1.15× | 1.15× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.020 ms**; native kernel **0.017 ms**; TA-Lib 0.046 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.374 | 0.270 | 3.71M | 46.310 | 171.59× | 125.44× |
| 1,500 | 10 | 1.867 | 1.087 | 9.20M | 46.303 | 42.59× | 30.62× |
| 1,500 | 100 | 6.612 | 4.758 | 21.02M | 50.322 | 10.58× | 7.57× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.97M | 17.22M | 1.00× | 1.02M | 1.13M | 1.00× | 7.31M |
| 2 | 15.24M | 19.24M | 1.12× | 1.37M | 1.32M | 1.16× | 9.07M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
