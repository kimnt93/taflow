# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.36M | 0.013 | 77.33M | 0.050 | 3.73× | 3.88× |
| 10,000 | 0.097 | 102.71M | 0.087 | 114.32M | 0.113 | 1.16× | 1.29× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.017 ms**; TA-Lib 0.052 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.353 | 0.278 | 3.60M | 62.371 | 224.51× | 148.67× |
| 1,500 | 10 | 1.432 | 1.807 | 5.53M | 52.867 | 29.26× | 23.52× |
| 1,500 | 100 | 4.768 | 3.741 | 26.73M | 53.506 | 14.30× | 11.04× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.59M | 12.49M | 1.00× | 995.75K | 1.01M | 1.00× | 7.22M |
| 2 | 16.15M | 17.64M | 1.41× | 1.22M | 1.21M | 1.19× | 7.59M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
