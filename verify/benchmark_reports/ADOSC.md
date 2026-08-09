# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 95.02M | 0.009 | 116.08M | 0.036 | 3.39× | 4.14× |
| 10,000 | 0.075 | 134.12M | 0.070 | 143.47M | 0.058 | 0.77× | 0.83× |
| 100,000 | 0.694 | 144.04M | 0.680 | 147.00M | 0.275 | 0.40× | 0.40× |
| 1,000,000 | 7.463 | 134.00M | 7.002 | 142.81M | 2.925 | 0.39× | 0.42× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.684 ms**; native kernel **0.683 ms**; TA-Lib 0.275 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.370 | 0.251 | 3.98M | 274.548 | 1093.00× | 125.03× |
| 100,000 | 10 | 2.229 | 1.092 | 9.15M | 264.589 | 242.21× | 28.69× |
| 100,000 | 1,000 | 13.332 | 8.730 | 114.55M | 271.557 | 31.11× | 4.10× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 106.29M | 118.77M | 1.00× | 2.05M | 2.43M | 1.00× | 255.92M |
| 2 | 220.11M | 225.17M | 1.90× | 2.04M | 2.69M | 1.11× | 241.44M |
| 4 | 351.08M | 447.15M | 3.76× | 1.93M | 2.40M | 0.99× | 244.34M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
