# LowerLow benchmark (`lower low relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.30M | 0.007 | 145.90M | 0.022 | 3.04× | 3.23× |
| 10,000 | 0.035 | 287.48M | 0.032 | 310.50M | 0.043 | 1.23× | 1.33× |
| 100,000 | 0.277 | 361.31M | 0.301 | 332.11M | 0.234 | 0.85× | 0.78× |
| 1,000,000 | 3.001 | 333.21M | 2.621 | 381.57M | 4.275 | 1.42× | 1.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.084 | 0.66× |
| 1 | 5 | 0.388 | 0.357 | 0.92× |
| 1 | 10 | 0.475 | 0.755 | 1.59× |
| 10 | 1 | 0.053 | 0.083 | 1.56× |
| 10 | 5 | 0.256 | 0.376 | 1.47× |
| 10 | 10 | 0.493 | 0.759 | 1.54× |
| 100 | 1 | 0.049 | 0.073 | 1.51× |
| 100 | 5 | 0.220 | 0.330 | 1.50× |
| 100 | 10 | 0.463 | 0.757 | 1.63× |
| 1,000 | 1 | 0.055 | 0.075 | 1.36× |
| 1,000 | 5 | 0.225 | 0.546 | 2.42× |
| 1,000 | 10 | 0.491 | 1.111 | 2.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
