# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.89M | 0.016 | 60.62M | 0.045 | 2.54× | 2.70× |
| 10,000 | 0.178 | 56.12M | 0.165 | 60.59M | 0.170 | 0.95× | 1.03× |
| 100,000 | 1.576 | 63.44M | 1.587 | 63.00M | 1.400 | 0.89× | 0.88× |
| 1,000,000 | 16.020 | 62.42M | 16.027 | 62.40M | 13.858 | 0.87× | 0.86× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.587 ms**; native kernel **1.541 ms**; TA-Lib 1.410 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.226 | 0.167 | 5.99M | 1389.051 | 8318.87× | 186.94× |
| 100,000 | 10 | 1.094 | 0.720 | 13.89M | 1447.599 | 2011.13× | 43.91× |
| 100,000 | 1,000 | 18.942 | 17.688 | 56.54M | 1414.679 | 79.98× | 2.54× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 54.07M | 56.27M | 1.00× | 2.89M | 3.72M | 1.00× | 60.06M |
| 2 | 101.78M | 108.54M | 1.93× | 2.53M | 3.22M | 0.87× | 60.91M |
| 4 | 188.30M | 203.74M | 3.62× | 2.67M | 2.62M | 0.70× | 60.43M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
