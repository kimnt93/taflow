# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 281.95M | 0.003 | 392.37M | 0.030 | 8.35× | 11.62× |
| 10,000 | 0.021 | 476.35M | 0.018 | 562.30M | 0.033 | 1.55× | 1.83× |
| 100,000 | 0.188 | 530.94M | 0.165 | 606.19M | 0.060 | 0.32× | 0.36× |
| 1,000,000 | 2.165 | 461.87M | 1.783 | 560.98M | 0.568 | 0.26× | 0.32× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.188 ms**; native kernel **0.165 ms**; TA-Lib 0.061 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.208 | 0.143 | 6.97M | 59.578 | 415.34× | 202.91× |
| 100,000 | 10 | 0.846 | 0.478 | 20.93M | 65.342 | 136.75× | 61.24× |
| 100,000 | 1,000 | 4.249 | 3.117 | 320.78M | 60.265 | 19.33× | 9.40× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 345.73M | 376.79M | 1.00× | 3.97M | 3.84M | 1.00× | 728.07M |
| 2 | 577.29M | 752.82M | 2.00× | 3.50M | 4.11M | 1.07× | 738.48M |
| 4 | 763.77M | 1.22G | 3.23× | 3.17M | 3.54M | 0.92× | 616.61M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
