# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.38M | 0.012 | 82.96M | 0.042 | 2.97× | 3.50× |
| 10,000 | 0.199 | 50.27M | 0.186 | 53.64M | 0.149 | 0.75× | 0.80× |
| 100,000 | 1.886 | 53.03M | 1.823 | 54.85M | 1.140 | 0.60× | 0.63× |
| 1,000,000 | 20.768 | 48.15M | 20.785 | 48.11M | 11.175 | 0.54× | 0.54× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.900 ms**; native kernel **1.807 ms**; TA-Lib 1.168 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.412 | 0.301 | 3.32M | 1133.015 | 3759.52× | 110.97× |
| 100,000 | 10 | 2.211 | 1.742 | 5.74M | 1122.534 | 644.41× | 19.01× |
| 100,000 | 1,000 | 81.837 | 77.584 | 12.89M | 1158.277 | 14.93× | 0.54× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 45.34M | 49.01M | 1.00× | 1.80M | 2.13M | 1.00× | 77.08M |
| 2 | 83.25M | 98.24M | 2.00× | 1.76M | 2.19M | 1.02× | 76.54M |
| 4 | 121.22M | 147.93M | 3.02× | 1.70M | 2.00M | 0.94× | 73.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
