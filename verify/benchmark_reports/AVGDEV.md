# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.54M | 0.019 | 53.73M | 0.047 | 2.28× | 2.53× |
| 10,000 | 0.163 | 61.52M | 0.160 | 62.42M | 0.171 | 1.05× | 1.07× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.028 ms**; native kernel **0.027 ms**; TA-Lib 0.053 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.315 | 0.190 | 5.26M | 52.053 | 273.94× | 163.28× |
| 1,500 | 10 | 1.373 | 0.820 | 12.19M | 54.899 | 66.95× | 38.33× |
| 1,500 | 100 | 4.626 | 3.532 | 28.32M | 54.020 | 15.30× | 8.99× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.13M | 8.78M | 1.00× | 1.23M | 1.45M | 1.00× | 8.55M |
| 2 | 14.49M | 19.54M | 2.23× | 1.27M | 1.64M | 1.13× | 8.72M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
