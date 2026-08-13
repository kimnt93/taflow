# ForceIndex benchmark (`ForceIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.28M | 0.037 | 27.37M | 0.193 | 4.50× | 5.29× |
| 10,000 | 0.280 | 35.70M | 0.270 | 37.01M | 0.733 | 2.62× | 2.71× |
| 100,000 | 2.586 | 38.68M | 2.581 | 38.74M | 5.913 | 2.29× | 2.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.245 | 2.17× |
| 1 | 5 | 0.414 | 1.144 | 2.76× |
| 1 | 10 | 0.671 | 2.298 | 3.42× |
| 10 | 1 | 0.077 | 0.209 | 2.73× |
| 10 | 5 | 0.308 | 1.346 | 4.37× |
| 10 | 10 | 0.649 | 2.382 | 3.67× |
| 100 | 1 | 0.082 | 0.214 | 2.62× |
| 100 | 5 | 0.316 | 1.394 | 4.42× |
| 100 | 10 | 0.628 | 2.420 | 3.85× |
| 1,000 | 1 | 0.110 | 0.295 | 2.68× |
| 1,000 | 5 | 0.320 | 1.677 | 5.24× |
| 1,000 | 10 | 0.664 | 2.958 | 4.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
