# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 133.10M | 0.006 | 154.27M | 0.037 | 4.89× | 5.67× |
| 10,000 | 0.064 | 157.28M | 0.059 | 169.10M | 0.084 | 1.31× | 1.41× |
| 100,000 | 0.613 | 163.10M | 0.578 | 173.08M | 0.567 | 0.93× | 0.98× |
| 1,000,000 | 6.129 | 163.15M | 5.756 | 173.72M | 5.351 | 0.87× | 0.93× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.593 ms**; native kernel **0.574 ms**; TA-Lib 0.570 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.235 | 0.168 | 5.94M | 561.004 | 3330.72× | 179.95× |
| 100,000 | 10 | 0.837 | 0.502 | 19.93M | 564.848 | 1125.83× | 59.63× |
| 100,000 | 1,000 | 11.305 | 7.324 | 136.53M | 558.568 | 76.26× | 4.89× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 120.63M | 138.36M | 1.00× | 2.74M | 3.58M | 1.00× | 137.74M |
| 2 | 254.03M | 269.42M | 1.95× | 3.13M | 3.67M | 1.03× | 139.94M |
| 4 | 401.84M | 500.28M | 3.62× | 3.01M | 3.29M | 0.92× | 135.61M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
