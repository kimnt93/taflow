# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.81M | 0.006 | 160.88M | 0.029 | 3.55× | 4.61× |
| 10,000 | 0.022 | 449.56M | 0.018 | 548.21M | 0.040 | 1.82× | 2.21× |
| 100,000 | 0.160 | 624.93M | 0.131 | 763.90M | 0.150 | 0.94× | 1.15× |
| 1,000,000 | 2.156 | 463.77M | 1.918 | 521.40M | 1.954 | 0.91× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.108 | 1.20× |
| 1 | 5 | 0.311 | 0.485 | 1.56× |
| 1 | 10 | 0.492 | 0.872 | 1.77× |
| 10 | 1 | 0.052 | 0.086 | 1.65× |
| 10 | 5 | 0.228 | 0.420 | 1.84× |
| 10 | 10 | 0.519 | 0.913 | 1.76× |
| 100 | 1 | 0.052 | 0.091 | 1.75× |
| 100 | 5 | 0.243 | 0.430 | 1.77× |
| 100 | 10 | 0.534 | 0.964 | 1.81× |
| 1,000 | 1 | 0.054 | 0.093 | 1.71× |
| 1,000 | 5 | 0.271 | 0.473 | 1.74× |
| 1,000 | 10 | 0.591 | 1.024 | 1.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
