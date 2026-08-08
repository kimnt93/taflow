# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.38M | 0.014 | 70.39M | 0.043 | 2.71× | 3.01× |
| 10,000 | 0.142 | 70.22M | 0.153 | 65.26M | 0.166 | 1.16× | 1.08× |
| 100,000 | 1.360 | 73.55M | 1.340 | 74.63M | 1.340 | 0.99× | 1.00× |
| 1,000,000 | 13.798 | 72.48M | 13.355 | 74.88M | 13.315 | 0.97× | 1.00× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.283 ms**; native kernel **1.314 ms**; TA-Lib 1.327 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.233 | 0.162 | 6.19M | 1331.045 | 8241.33× | 192.39× |
| 100,000 | 10 | 1.058 | 0.694 | 14.42M | 1287.747 | 1856.32× | 42.68× |
| 100,000 | 1,000 | 19.988 | 15.203 | 65.78M | 1319.097 | 86.77× | 3.02× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 70.29M | 70.02M | 1.00× | 3.09M | 3.38M | 1.00× | 66.42M |
| 2 | 126.40M | 134.76M | 1.92× | 3.13M | 3.47M | 1.03× | 68.35M |
| 4 | 215.14M | 247.30M | 3.53× | 2.53M | 2.86M | 0.84× | 66.51M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
