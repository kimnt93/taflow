# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.31M | 0.005 | 195.06M | 0.047 | 6.63× | 9.21× |
| 10,000 | 0.050 | 201.79M | 0.041 | 244.61M | 0.130 | 2.62× | 3.17× |
| 100,000 | 1.612 | 62.05M | 0.387 | 258.58M | 1.664 | 1.03× | 4.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.135 | 1.82× |
| 1 | 5 | 0.226 | 0.494 | 2.19× |
| 1 | 10 | 0.439 | 1.056 | 2.41× |
| 10 | 1 | 0.040 | 0.114 | 2.85× |
| 10 | 5 | 0.182 | 0.487 | 2.68× |
| 10 | 10 | 0.401 | 1.059 | 2.64× |
| 100 | 1 | 0.042 | 0.103 | 2.43× |
| 100 | 5 | 0.179 | 0.494 | 2.76× |
| 100 | 10 | 0.387 | 1.016 | 2.63× |
| 1,000 | 1 | 0.057 | 0.130 | 2.27× |
| 1,000 | 5 | 0.210 | 0.551 | 2.62× |
| 1,000 | 10 | 0.445 | 1.129 | 2.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
