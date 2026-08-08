# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.52M | 0.006 | 178.16M | 0.039 | 5.58× | 7.03× |
| 10,000 | 0.056 | 179.40M | 0.044 | 226.26M | 0.114 | 2.04× | 2.57× |
| 100,000 | 0.450 | 222.35M | 0.412 | 242.67M | 0.814 | 1.81× | 1.97× |
| 1,000,000 | 5.940 | 168.36M | 5.171 | 193.38M | 7.851 | 1.32× | 1.52× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.450 ms**; native kernel **0.402 ms**; TA-Lib 0.824 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.366 | 0.271 | 3.68M | 827.692 | 3048.81× | 123.75× |
| 100,000 | 10 | 1.682 | 1.141 | 8.77M | 812.404 | 712.26× | 29.43× |
| 100,000 | 1,000 | 74.398 | 64.485 | 15.51M | 852.212 | 13.22× | 0.69× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 137.08M | 190.62M | 1.00× | 1.78M | 2.06M | 1.00× | 101.38M |
| 2 | 248.20M | 358.67M | 1.88× | 1.97M | 2.60M | 1.26× | 100.64M |
| 4 | 290.27M | 429.72M | 2.25× | 2.13M | 2.25M | 1.09× | 102.46M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
