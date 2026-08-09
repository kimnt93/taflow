# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.64M | 0.013 | 79.38M | 0.037 | 2.47× | 2.90× |
| 10,000 | 0.096 | 103.93M | 0.079 | 126.22M | 0.065 | 0.67× | 0.82× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.022 ms**; native kernel **0.017 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.501 | 0.278 | 3.59M | 39.410 | 141.54× | 119.30× |
| 1,500 | 10 | 2.603 | 1.243 | 8.05M | 38.435 | 30.93× | 28.79× |
| 1,500 | 100 | 5.509 | 3.196 | 31.29M | 38.776 | 12.13× | 10.66× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.15M | 15.96M | 1.00× | 937.64K | 1.24M | 1.00× | 7.71M |
| 2 | 16.67M | 20.47M | 1.28× | 939.32K | 1.40M | 1.13× | 8.69M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
