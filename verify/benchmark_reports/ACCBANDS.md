# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.52M | 0.011 | 93.67M | 0.048 | 4.08× | 4.47× |
| 10,000 | 0.092 | 108.85M | 0.086 | 116.80M | 0.113 | 1.24× | 1.33× |
| 100,000 | 0.918 | 108.93M | 0.842 | 118.71M | 0.802 | 0.87× | 0.95× |
| 1,000,000 | 19.281 | 51.87M | 15.546 | 64.33M | 12.405 | 0.64× | 0.80× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.938 ms**; native kernel **0.845 ms**; TA-Lib 0.785 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.413 | 0.343 | 2.91M | 809.063 | 2355.66× | 115.48× |
| 100,000 | 10 | 1.999 | 1.968 | 5.08M | 826.465 | 420.02× | 20.58× |
| 100,000 | 1,000 | 109.045 | 85.243 | 11.73M | 805.189 | 9.45× | 0.57× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 50.16M | 86.77M | 1.00× | 1.64M | 1.03M | 1.00× | 87.67M |
| 2 | 112.51M | 187.84M | 2.16× | 1.54M | 1.66M | 1.61× | 95.22M |
| 4 | 153.74M | 292.79M | 3.37× | 1.36M | 1.44M | 1.39× | 96.11M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
