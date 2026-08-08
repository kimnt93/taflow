# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 194.47M | 0.003 | 302.06M | 0.029 | 5.72× | 8.88× |
| 10,000 | 0.053 | 187.92M | 0.050 | 200.51M | 0.073 | 1.36× | 1.46× |
| 100,000 | 0.573 | 174.46M | 0.540 | 185.30M | 0.472 | 0.82× | 0.87× |
| 1,000,000 | 6.105 | 163.80M | 6.021 | 166.07M | 4.650 | 0.76× | 0.77× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.557 ms**; native kernel **0.550 ms**; TA-Lib 0.474 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.326 | 0.255 | 3.93M | 482.871 | 1897.29× | 103.36× |
| 100,000 | 10 | 2.617 | 1.253 | 7.98M | 489.627 | 390.77× | 21.67× |
| 100,000 | 1,000 | 10.706 | 7.876 | 126.97M | 482.564 | 61.27× | 3.69× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 132.53M | 146.38M | 1.00× | 2.64M | 2.85M | 1.00× | 162.14M |
| 2 | 253.40M | 284.18M | 1.94× | 2.36M | 2.65M | 0.93× | 166.74M |
| 4 | 464.44M | 487.47M | 3.33× | 2.47M | 2.72M | 0.96× | 168.31M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
