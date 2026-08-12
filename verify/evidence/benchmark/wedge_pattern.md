# WedgePattern benchmark (`Wedge` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.58M | 0.015 | 66.55M | 0.233 | 16.42× | 15.48× |
| 10,000 | 0.103 | 97.50M | 0.090 | 111.19M | 1.344 | 13.10× | 14.94× |
| 100,000 | 1.155 | 86.58M | 0.849 | 117.82M | 13.746 | 11.90× | 16.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.226 | 2.51× |
| 1 | 5 | 0.297 | 0.896 | 3.02× |
| 1 | 10 | 0.580 | 1.933 | 3.33× |
| 10 | 1 | 0.066 | 0.178 | 2.68× |
| 10 | 5 | 0.305 | 1.190 | 3.90× |
| 10 | 10 | 0.649 | 1.806 | 2.78× |
| 100 | 1 | 0.061 | 0.179 | 2.93× |
| 100 | 5 | 0.251 | 1.276 | 5.08× |
| 100 | 10 | 0.565 | 1.873 | 3.31× |
| 1,000 | 1 | 0.064 | 0.307 | 4.83× |
| 1,000 | 5 | 0.280 | 1.877 | 6.71× |
| 1,000 | 10 | 0.559 | 3.095 | 5.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
