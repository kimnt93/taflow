# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 158.21M | 0.005 | 186.66M | 0.038 | 6.06× | 7.15× |
| 10,000 | 0.044 | 228.74M | 0.039 | 255.11M | 0.080 | 1.82× | 2.03× |
| 100,000 | 0.406 | 246.35M | 0.376 | 266.24M | 0.486 | 1.20× | 1.29× |
| 1,000,000 | 4.281 | 233.62M | 3.938 | 253.95M | 4.972 | 1.16× | 1.26× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.415 ms**; native kernel **0.381 ms**; TA-Lib 0.481 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.220 | 0.166 | 6.02M | 477.246 | 2873.40× | 202.50× |
| 100,000 | 10 | 0.968 | 0.573 | 17.44M | 483.416 | 843.17× | 58.76× |
| 100,000 | 1,000 | 6.541 | 6.343 | 157.64M | 481.951 | 75.98× | 6.26× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 158.01M | 198.56M | 1.00× | 3.58M | 3.58M | 1.00× | 150.76M |
| 2 | 298.56M | 391.16M | 1.97× | 3.26M | 4.10M | 1.14× | 152.95M |
| 4 | 463.53M | 632.56M | 3.19× | 2.84M | 3.21M | 0.90× | 149.55M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
