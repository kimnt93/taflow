# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 362.42M | 0.001 | 742.69M | 0.031 | 11.09× | 22.73× |
| 10,000 | 0.010 | 1.02G | 0.006 | 1.68G | 0.036 | 3.63× | 5.98× |
| 100,000 | 0.075 | 1.33G | 0.049 | 2.02G | 0.078 | 1.04× | 1.59× |
| 1,000,000 | 1.356 | 737.72M | 0.943 | 1.06G | 0.931 | 0.69× | 0.99× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.076 ms**; native kernel **0.050 ms**; TA-Lib 0.080 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.283 | 0.195 | 5.13M | 79.890 | 409.88× | 164.28× |
| 100,000 | 10 | 1.554 | 0.754 | 13.26M | 78.058 | 103.53× | 39.15× |
| 100,000 | 1,000 | 3.899 | 2.281 | 438.46M | 80.287 | 35.20× | 13.22× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 320.18M | 632.23M | 1.00× | 2.29M | 2.92M | 1.00× | 521.22M |
| 2 | 693.20M | 1.37G | 2.17× | 2.71M | 3.27M | 1.12× | 673.23M |
| 4 | 907.19M | 1.61G | 2.55× | 2.71M | 3.00M | 1.03× | 561.53M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
