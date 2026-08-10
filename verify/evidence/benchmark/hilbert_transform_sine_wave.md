# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.147 | 6.79M | 0.147 | 6.83M | 0.539 | 3.66× | 3.68× |
| 10,000 | 1.550 | 6.45M | 1.487 | 6.73M | 5.125 | 3.31× | 3.45× |
| 100,000 | 15.565 | 6.42M | 14.973 | 6.68M | 53.197 | 3.42× | 3.55× |
| 1,000,000 | 158.013 | 6.33M | 156.896 | 6.37M | 534.576 | 3.38× | 3.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.142 | 0.228 | 1.61× |
| 1 | 5 | 0.330 | 0.503 | 1.53× |
| 1 | 10 | 0.526 | 0.973 | 1.85× |
| 10 | 1 | 0.052 | 0.092 | 1.75× |
| 10 | 5 | 0.272 | 0.534 | 1.96× |
| 10 | 10 | 0.569 | 1.004 | 1.76× |
| 100 | 1 | 0.071 | 0.132 | 1.86× |
| 100 | 5 | 0.272 | 0.696 | 2.56× |
| 100 | 10 | 0.584 | 1.269 | 2.17× |
| 1,000 | 1 | 0.209 | 0.581 | 2.77× |
| 1,000 | 5 | 0.376 | 2.982 | 7.94× |
| 1,000 | 10 | 0.634 | 6.088 | 9.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
