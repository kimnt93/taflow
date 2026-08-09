# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 95.16M | 0.009 | 116.41M | 0.035 | 3.37× | 4.12× |
| 10,000 | 0.118 | 84.79M | 0.111 | 90.47M | 0.177 | 1.50× | 1.61× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.011 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.361 | 0.291 | 3.43M | 38.093 | 130.68× | 103.63× |
| 1,500 | 10 | 2.615 | 1.310 | 7.63M | 38.117 | 29.09× | 21.70× |
| 1,500 | 100 | 5.718 | 3.665 | 27.28M | 38.779 | 10.58× | 8.03× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.68M | 13.61M | 1.00× | 1.27M | 1.19M | 1.00× | 7.49M |
| 2 | 14.04M | 19.18M | 1.41× | 1.17M | 1.35M | 1.14× | 8.71M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
