# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.13M | 0.053 | 18.82M | 0.085 | 1.54× | 1.60× |
| 10,000 | 0.548 | 18.23M | 0.530 | 18.86M | 0.542 | 0.99× | 1.02× |
| 100,000 | 5.487 | 18.22M | 5.305 | 18.85M | 5.035 | 0.92× | 0.95× |
| 1,000,000 | 53.838 | 18.57M | 53.613 | 18.65M | 50.185 | 0.93× | 0.94× |

## Warm-up

Construct + canonical extend over 100,000 bars: **5.367 ms**; native kernel **5.323 ms**; TA-Lib 5.257 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.371 | 0.271 | 3.69M | 5021.968 | 18517.10× | 142.21× |
| 100,000 | 10 | 2.351 | 1.471 | 6.80M | 4970.679 | 3378.98× | 25.85× |
| 100,000 | 1,000 | 116.866 | 101.832 | 9.82M | 5111.076 | 50.19× | 0.83× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 17.86M | 17.90M | 1.00× | 1.63M | 1.80M | 1.00× | 18.46M |
| 2 | 34.01M | 35.17M | 1.96× | 1.72M | 2.04M | 1.13× | 18.03M |
| 4 | 53.83M | 51.40M | 2.87× | 1.77M | 1.92M | 1.07× | 17.87M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
