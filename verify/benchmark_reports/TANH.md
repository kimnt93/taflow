# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 192.08M | 0.004 | 235.70M | 0.028 | 5.42× | 6.65× |
| 10,000 | 0.038 | 265.84M | 0.035 | 281.84M | 0.053 | 1.41× | 1.49× |
| 100,000 | 0.368 | 271.40M | 0.339 | 295.41M | 0.287 | 0.78× | 0.85× |
| 1,000,000 | 4.549 | 219.85M | 3.984 | 250.99M | 2.696 | 0.59× | 0.68× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.363 ms**; native kernel **0.339 ms**; TA-Lib 0.289 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.217 | 0.150 | 6.66M | 286.651 | 1909.98× | 170.44× |
| 100,000 | 10 | 0.966 | 0.560 | 17.86M | 288.050 | 514.34× | 44.52× |
| 100,000 | 1,000 | 6.086 | 4.661 | 214.56M | 290.015 | 62.23× | 6.09× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 211.41M | 210.45M | 1.00× | 3.61M | 3.33M | 1.00× | 235.09M |
| 2 | 350.70M | 434.46M | 2.06× | 3.27M | 3.66M | 1.10× | 254.70M |
| 4 | 423.00M | 670.82M | 3.19× | 3.13M | 3.42M | 1.03× | 254.35M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
