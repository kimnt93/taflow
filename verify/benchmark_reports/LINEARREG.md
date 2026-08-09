# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.20M | 0.014 | 72.75M | 0.042 | 2.84× | 3.03× |
| 10,000 | 0.132 | 75.97M | 0.125 | 80.25M | 0.152 | 1.16× | 1.22× |
| 100,000 | 1.270 | 78.74M | 1.225 | 81.60M | 1.224 | 0.96× | 1.00× |
| 1,000,000 | 13.085 | 76.42M | 12.265 | 81.53M | 11.918 | 0.91× | 0.97× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.266 ms**; native kernel **1.231 ms**; TA-Lib 1.217 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.229 | 0.165 | 6.06M | 1268.841 | 7688.58× | 175.41× |
| 100,000 | 10 | 1.061 | 0.700 | 14.28M | 1211.870 | 1730.36× | 42.46× |
| 100,000 | 1,000 | 15.027 | 16.583 | 60.30M | 1222.910 | 73.75× | 2.55× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 66.83M | 68.54M | 1.00× | 2.88M | 2.95M | 1.00× | 67.01M |
| 2 | 128.96M | 136.04M | 1.98× | 2.68M | 3.54M | 1.20× | 69.16M |
| 4 | 219.07M | 257.77M | 3.76× | 2.61M | 2.78M | 0.94× | 68.73M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
