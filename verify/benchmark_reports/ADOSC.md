# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 153.38M | 0.004 | 222.59M | 0.036 | 5.45× | 7.91× |
| 10,000 | 0.034 | 291.99M | 0.029 | 340.85M | 0.060 | 1.76× | 2.06× |
| 100,000 | 0.319 | 313.34M | 0.272 | 367.37M | 0.326 | 1.02× | 1.20× |
| 1,000,000 | 4.196 | 238.32M | 3.647 | 274.24M | 3.321 | 0.79× | 0.91× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.310 ms**; native kernel **0.280 ms**; TA-Lib 0.291 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.446 | 0.275 | 3.64M | 292.910 | 1066.72× | 120.23× |
| 100,000 | 10 | 2.527 | 1.173 | 8.53M | 286.366 | 244.15× | 28.71× |
| 100,000 | 1,000 | 7.553 | 4.805 | 208.10M | 299.135 | 62.25× | 7.90× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 222.04M | 259.75M | 1.00× | 1.87M | 2.22M | 1.00× | 210.96M |
| 2 | 330.71M | 484.75M | 1.87× | 1.89M | 2.57M | 1.16× | 230.11M |
| 4 | 505.19M | 663.25M | 2.55× | 1.94M | 2.44M | 1.10× | 227.90M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
