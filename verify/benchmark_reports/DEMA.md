# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 184.17M | 0.005 | 218.29M | 0.042 | 7.68× | 9.10× |
| 10,000 | 0.037 | 272.72M | 0.037 | 270.07M | 0.097 | 2.65× | 2.62× |
| 100,000 | 0.344 | 290.75M | 0.333 | 300.54M | 0.660 | 1.92× | 1.98× |
| 1,000,000 | 4.105 | 243.58M | 3.455 | 289.42M | 6.809 | 1.66× | 1.97× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.342 ms**; native kernel **0.336 ms**; TA-Lib 0.626 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.224 | 0.157 | 6.35M | 617.260 | 3921.12× | 196.30× |
| 100,000 | 10 | 0.857 | 0.575 | 17.41M | 620.778 | 1080.52× | 53.36× |
| 100,000 | 1,000 | 5.801 | 4.598 | 217.50M | 640.032 | 139.21× | 8.47× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 224.70M | 239.62M | 1.00× | 3.50M | 4.17M | 1.00× | 135.13M |
| 2 | 418.58M | 389.59M | 1.63× | 3.21M | 3.64M | 0.87× | 128.53M |
| 4 | 566.68M | 771.34M | 3.22× | 3.04M | 3.47M | 0.83× | 129.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
