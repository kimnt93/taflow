# IntradayIntensity benchmark (`IntradayIntensity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.79M | 0.057 | 17.44M | 0.228 | 4.28× | 3.97× |
| 10,000 | 0.361 | 27.73M | 0.345 | 28.94M | 1.310 | 3.63× | 3.79× |
| 100,000 | 3.704 | 26.99M | 3.583 | 27.91M | 12.102 | 3.27× | 3.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.201 | 1.80× |
| 1 | 5 | 0.411 | 1.189 | 2.89× |
| 1 | 10 | 0.648 | 1.904 | 2.94× |
| 10 | 1 | 0.077 | 0.162 | 2.11× |
| 10 | 5 | 0.306 | 0.796 | 2.60× |
| 10 | 10 | 0.614 | 1.897 | 3.09× |
| 100 | 1 | 0.082 | 0.179 | 2.17× |
| 100 | 5 | 0.320 | 0.879 | 2.74× |
| 100 | 10 | 0.619 | 2.000 | 3.23× |
| 1,000 | 1 | 0.107 | 0.289 | 2.71× |
| 1,000 | 5 | 0.311 | 1.442 | 4.63× |
| 1,000 | 10 | 0.667 | 2.890 | 4.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
