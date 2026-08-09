# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.48M | 0.009 | 107.60M | 0.040 | 4.12× | 4.32× |
| 10,000 | 0.085 | 117.68M | 0.079 | 126.36M | 0.117 | 1.38× | 1.48× |
| 100,000 | 0.807 | 123.92M | 0.770 | 129.87M | 0.896 | 1.11× | 1.16× |
| 1,000,000 | 8.602 | 116.25M | 7.956 | 125.69M | 8.890 | 1.03× | 1.12× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.816 ms**; native kernel **0.786 ms**; TA-Lib 0.884 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.297 | 0.212 | 4.71M | 941.652 | 4437.35× | 148.62× |
| 100,000 | 10 | 1.007 | 0.983 | 10.17M | 926.221 | 942.41× | 32.14× |
| 100,000 | 1,000 | 10.007 | 9.707 | 103.02M | 898.562 | 92.57× | 4.07× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 98.47M | 95.15M | 1.00× | 2.48M | 2.99M | 1.00× | 90.55M |
| 2 | 185.46M | 209.95M | 2.21× | 2.57M | 3.23M | 1.08× | 89.40M |
| 4 | 305.06M | 307.16M | 3.23× | 2.43M | 2.43M | 0.81× | 93.40M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
