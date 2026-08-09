# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 256.51M | 0.003 | 336.09M | 0.032 | 8.09× | 10.60× |
| 10,000 | 0.023 | 429.03M | 0.021 | 486.61M | 0.050 | 2.14× | 2.43× |
| 100,000 | 0.208 | 480.75M | 0.185 | 539.57M | 0.208 | 1.00× | 1.12× |
| 1,000,000 | 2.292 | 436.33M | 1.953 | 512.00M | 1.917 | 0.84× | 0.98× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.207 ms**; native kernel **0.186 ms**; TA-Lib 0.209 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.207 | 0.153 | 6.56M | 206.518 | 1353.92× | 196.28× |
| 100,000 | 10 | 0.856 | 0.491 | 20.35M | 206.620 | 420.48× | 61.56× |
| 100,000 | 1,000 | 5.070 | 3.268 | 305.98M | 215.403 | 65.91× | 9.95× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 278.75M | 371.32M | 1.00× | 2.95M | 4.08M | 1.00× | 353.42M |
| 2 | 598.66M | 616.35M | 1.66× | 2.93M | 3.62M | 0.89× | 315.60M |
| 4 | 647.99M | 1.12G | 3.03× | 3.21M | 3.40M | 0.83× | 316.69M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
