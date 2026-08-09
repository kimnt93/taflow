# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.71M | 0.009 | 114.42M | 0.036 | 3.40× | 4.16× |
| 10,000 | 0.077 | 129.55M | 0.078 | 128.08M | 0.122 | 1.58× | 1.57× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.020 ms**; native kernel **0.011 ms**; TA-Lib 0.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.377 | 0.280 | 3.57M | 41.143 | 147.07× | 106.66× |
| 1,500 | 10 | 2.625 | 1.306 | 7.66M | 40.473 | 30.99× | 22.05× |
| 1,500 | 100 | 6.059 | 3.856 | 25.94M | 41.272 | 10.70× | 7.54× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.13M | 9.48M | 1.00× | 1.00M | 1.14M | 1.00× | 8.65M |
| 2 | 16.22M | 19.05M | 2.01× | 1.33M | 1.31M | 1.15× | 9.35M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
