# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 64.76M | 0.014 | 69.71M | 0.043 | 2.77× | 2.99× |
| 10,000 | 0.132 | 75.51M | 0.129 | 77.72M | 0.158 | 1.19× | 1.23× |
| 100,000 | 1.282 | 77.99M | 1.261 | 79.32M | 1.257 | 0.98× | 1.00× |
| 1,000,000 | 13.138 | 76.12M | 12.596 | 79.39M | 12.545 | 0.95× | 1.00× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.282 ms**; native kernel **1.271 ms**; TA-Lib 1.290 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.248 | 0.171 | 5.86M | 1268.459 | 7431.53× | 172.84× |
| 100,000 | 10 | 1.095 | 0.695 | 14.39M | 1300.422 | 1871.00× | 43.59× |
| 100,000 | 1,000 | 16.021 | 14.515 | 68.89M | 1332.334 | 91.79× | 3.19× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 63.22M | 68.81M | 1.00× | 2.60M | 3.08M | 1.00× | 68.78M |
| 2 | 125.11M | 126.82M | 1.84× | 2.62M | 3.12M | 1.01× | 65.76M |
| 4 | 150.22M | 195.81M | 2.85× | 2.52M | 2.97M | 0.97× | 59.75M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
