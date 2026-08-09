# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.60M | 0.043 | 23.51M | 0.068 | 1.53× | 1.59× |
| 10,000 | 0.425 | 23.54M | 0.424 | 23.57M | 0.457 | 1.08× | 1.08× |
| 100,000 | 4.412 | 22.67M | 4.221 | 23.69M | 4.196 | 0.95× | 0.99× |
| 1,000,000 | 42.796 | 23.37M | 43.677 | 22.90M | 42.118 | 0.98× | 0.96× |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.456 ms**; native kernel **4.213 ms**; TA-Lib 4.163 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.241 | 0.174 | 5.75M | 4404.614 | 25317.64× | 153.63× |
| 100,000 | 10 | 1.356 | 1.029 | 9.72M | 4192.000 | 4073.82× | 26.44× |
| 100,000 | 1,000 | 45.486 | 43.928 | 22.76M | 4216.238 | 95.98× | 1.56× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21.97M | 22.29M | 1.00× | 2.84M | 3.00M | 1.00× | 21.96M |
| 2 | 42.15M | 40.48M | 1.82× | 2.38M | 3.01M | 1.00× | 21.67M |
| 4 | 78.56M | 81.91M | 3.67× | 2.33M | 2.67M | 0.89× | 21.95M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
