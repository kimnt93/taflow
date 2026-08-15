# YangZhang benchmark (`YangZhangVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.49M | 0.042 | 23.87M | 0.332 | 7.13× | 7.92× |
| 10,000 | 0.411 | 24.31M | 0.418 | 23.92M | 1.838 | 4.47× | 4.40× |
| 100,000 | 4.146 | 24.12M | 3.909 | 25.58M | 16.348 | 3.94× | 4.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.331 | 4.71× |
| 1 | 5 | 0.272 | 1.458 | 5.35× |
| 1 | 10 | 0.403 | 2.512 | 6.23× |
| 10 | 1 | 0.049 | 0.242 | 4.92× |
| 10 | 5 | 0.201 | 1.467 | 7.30× |
| 10 | 10 | 0.393 | 2.671 | 6.80× |
| 100 | 1 | 0.052 | 0.246 | 4.75× |
| 100 | 5 | 0.204 | 1.504 | 7.37× |
| 100 | 10 | 0.412 | 2.607 | 6.34× |
| 1,000 | 1 | 0.087 | 0.406 | 4.68× |
| 1,000 | 5 | 0.192 | 2.264 | 11.82× |
| 1,000 | 10 | 0.439 | 4.698 | 10.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
