# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.37M | 0.005 | 204.51M | 0.033 | 5.02× | 6.79× |
| 10,000 | 0.073 | 136.25M | 0.063 | 158.02M | 0.140 | 1.91× | 2.22× |
| 100,000 | 0.957 | 104.54M | 0.946 | 105.73M | 1.195 | 1.25× | 1.26× |
| 1,000,000 | 10.176 | 98.27M | 9.992 | 100.08M | 11.693 | 1.15× | 1.17× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.956 ms**; native kernel **0.952 ms**; TA-Lib 1.169 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.351 | 0.301 | 3.33M | 1134.303 | 3772.98× | 92.09× |
| 100,000 | 10 | 2.813 | 1.503 | 6.65M | 1136.354 | 756.20× | 18.55× |
| 100,000 | 1,000 | 39.681 | 36.897 | 27.10M | 1149.103 | 31.14× | 0.91× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 88.26M | 92.53M | 1.00× | 2.09M | 2.59M | 1.00× | 79.13M |
| 2 | 178.36M | 182.75M | 1.98× | 2.32M | 2.57M | 0.99× | 76.62M |
| 4 | 294.13M | 312.15M | 3.37× | 2.21M | 2.31M | 0.89× | 77.95M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
