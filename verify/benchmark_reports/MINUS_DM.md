# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.07M | 0.006 | 170.01M | 0.036 | 4.89× | 6.15× |
| 10,000 | 0.058 | 173.64M | 0.051 | 195.23M | 0.083 | 1.44× | 1.62× |
| 100,000 | 0.527 | 189.58M | 0.497 | 201.09M | 0.529 | 1.00× | 1.06× |
| 1,000,000 | 5.518 | 181.22M | 5.035 | 198.61M | 5.147 | 0.93× | 1.02× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.528 ms**; native kernel **0.485 ms**; TA-Lib 0.515 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.264 | 0.179 | 5.59M | 515.472 | 2881.74× | 167.68× |
| 100,000 | 10 | 1.384 | 1.187 | 8.43M | 523.235 | 440.90× | 25.84× |
| 100,000 | 1,000 | 8.827 | 7.194 | 139.01M | 548.754 | 76.28× | 5.11× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 133.15M | 172.07M | 1.00× | 3.20M | 3.71M | 1.00× | 145.52M |
| 2 | 279.83M | 301.37M | 1.75× | 2.96M | 3.07M | 0.83× | 156.64M |
| 4 | 427.43M | 557.37M | 3.24× | 2.93M | 2.97M | 0.80× | 149.05M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
