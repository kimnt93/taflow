# RectangleRange benchmark (`RectangleRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.91M | 0.008 | 124.43M | 0.234 | 22.65× | 29.08× |
| 10,000 | 0.109 | 91.74M | 0.088 | 113.61M | 1.378 | 12.64× | 15.66× |
| 100,000 | 0.907 | 110.23M | 0.874 | 114.38M | 12.887 | 14.20× | 14.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.245 | 3.17× |
| 1 | 5 | 0.356 | 0.846 | 2.37× |
| 1 | 10 | 0.392 | 1.694 | 4.33× |
| 10 | 1 | 0.049 | 0.168 | 3.45× |
| 10 | 5 | 0.200 | 1.083 | 5.42× |
| 10 | 10 | 0.388 | 1.726 | 4.45× |
| 100 | 1 | 0.051 | 0.180 | 3.56× |
| 100 | 5 | 0.211 | 1.145 | 5.42× |
| 100 | 10 | 0.420 | 1.850 | 4.40× |
| 1,000 | 1 | 0.061 | 0.311 | 5.07× |
| 1,000 | 5 | 0.204 | 1.769 | 8.66× |
| 1,000 | 10 | 0.463 | 3.049 | 6.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
