# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.17M | 0.009 | 107.50M | 0.043 | 3.78× | 4.61× |
| 10,000 | 0.079 | 126.67M | 0.076 | 131.10M | 0.182 | 2.30× | 2.38× |
| 100,000 | 0.759 | 131.73M | 0.738 | 135.58M | 1.582 | 2.08× | 2.14× |
| 1,000,000 | 7.958 | 125.66M | 8.066 | 123.98M | 15.315 | 1.92× | 1.90× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.745 ms**; native kernel **0.732 ms**; TA-Lib 1.502 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.348 | 0.280 | 3.57M | 1500.390 | 5351.33× | 98.80× |
| 100,000 | 10 | 2.757 | 1.385 | 7.22M | 1518.648 | 1096.51× | 20.99× |
| 100,000 | 1,000 | 41.149 | 27.450 | 36.43M | 1626.966 | 59.27× | 1.60× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 104.94M | 114.29M | 1.00× | 2.00M | 2.40M | 1.00× | 58.15M |
| 2 | 226.65M | 224.81M | 1.97× | 2.45M | 2.51M | 1.05× | 58.10M |
| 4 | 371.98M | 345.33M | 3.02× | 2.05M | 2.28M | 0.95× | 55.99M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
