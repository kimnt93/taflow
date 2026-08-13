# PercentAboveMovingAverage benchmark (`PercentAboveMa` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.58M | 0.033 | 30.74M | 11.396 | 314.28× | 350.28× |
| 10,000 | 0.248 | 40.33M | 0.240 | 41.65M | 109.656 | 442.30× | 456.68× |
| 100,000 | 2.223 | 44.98M | 2.107 | 47.47M | 1102.671 | 495.98× | 523.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.224 | 0.287 | 1.28× |
| 1 | 5 | 0.437 | 1.473 | 3.37× |
| 1 | 10 | 0.630 | 2.269 | 3.60× |
| 10 | 1 | 0.072 | 0.326 | 4.55× |
| 10 | 5 | 0.296 | 1.569 | 5.31× |
| 10 | 10 | 0.613 | 3.353 | 5.47× |
| 100 | 1 | 0.075 | 1.369 | 18.15× |
| 100 | 5 | 0.322 | 6.840 | 21.25× |
| 100 | 10 | 0.632 | 14.043 | 22.22× |
| 1,000 | 1 | 0.103 | 11.422 | 111.40× |
| 1,000 | 5 | 0.412 | 60.355 | 146.45× |
| 1,000 | 10 | 0.769 | 123.736 | 161.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
