# GartleyPattern benchmark (`Gartley` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.46M | 0.049 | 20.23M | 0.212 | 3.71× | 4.30× |
| 10,000 | 0.395 | 25.31M | 0.381 | 26.23M | 1.315 | 3.33× | 3.45× |
| 100,000 | 3.748 | 26.68M | 3.738 | 26.75M | 12.303 | 3.28× | 3.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.225 | 1.95× |
| 1 | 5 | 0.469 | 0.822 | 1.75× |
| 1 | 10 | 0.651 | 2.332 | 3.58× |
| 10 | 1 | 0.082 | 0.177 | 2.15× |
| 10 | 5 | 0.315 | 1.099 | 3.49× |
| 10 | 10 | 0.651 | 1.667 | 2.56× |
| 100 | 1 | 0.073 | 0.172 | 2.35× |
| 100 | 5 | 0.315 | 1.116 | 3.54× |
| 100 | 10 | 0.693 | 1.786 | 2.58× |
| 1,000 | 1 | 0.109 | 0.296 | 2.71× |
| 1,000 | 5 | 0.303 | 1.686 | 5.55× |
| 1,000 | 10 | 0.672 | 2.967 | 4.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
