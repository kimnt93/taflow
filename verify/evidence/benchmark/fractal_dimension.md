# FractalDimension benchmark (`two-chunk rescaled-range dimension` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.872 | 1.15M | 0.821 | 1.22M | 0.755 | 0.87× | 0.92× |
| 10,000 | 8.237 | 1.21M | 8.281 | 1.21M | 5.747 | 0.70× | 0.69× |
| 100,000 | 83.844 | 1.19M | 84.287 | 1.19M | 58.651 | 0.70× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.196 | 0.157 | 0.80× |
| 1 | 5 | 0.436 | 0.423 | 0.97× |
| 1 | 10 | 0.600 | 0.816 | 1.36× |
| 10 | 1 | 0.066 | 0.082 | 1.25× |
| 10 | 5 | 0.290 | 0.404 | 1.39× |
| 10 | 10 | 0.590 | 0.828 | 1.40× |
| 100 | 1 | 0.137 | 0.382 | 2.78× |
| 100 | 5 | 0.311 | 2.137 | 6.87× |
| 100 | 10 | 0.683 | 4.123 | 6.04× |
| 1,000 | 1 | 0.920 | 0.910 | 0.99× |
| 1,000 | 5 | 1.113 | 2.968 | 2.67× |
| 1,000 | 10 | 2.054 | 6.434 | 3.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
