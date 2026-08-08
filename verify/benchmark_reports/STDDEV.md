# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 186.73M | 0.005 | 193.09M | 0.039 | 7.30× | 7.55× |
| 10,000 | 0.044 | 229.10M | 0.042 | 235.57M | 0.065 | 1.49× | 1.53× |
| 100,000 | 0.335 | 298.25M | 0.319 | 313.04M | 0.322 | 0.96× | 1.01× |
| 1,000,000 | 3.698 | 270.39M | 3.202 | 312.35M | 3.020 | 0.82× | 0.94× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.351 ms**; native kernel **0.325 ms**; TA-Lib 0.334 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.274 | 0.150 | 6.65M | 311.151 | 2068.02× | 237.70× |
| 100,000 | 10 | 0.889 | 0.526 | 19.00M | 300.568 | 570.94× | 68.52× |
| 100,000 | 1,000 | 6.195 | 5.604 | 178.43M | 319.355 | 56.98× | 7.08× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 186.18M | 250.74M | 1.00× | 3.35M | 3.13M | 1.00× | 207.56M |
| 2 | 366.11M | 432.16M | 1.72× | 2.39M | 3.01M | 0.96× | 215.03M |
| 4 | 544.74M | 762.97M | 3.04× | 2.94M | 3.11M | 0.99× | 228.53M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
