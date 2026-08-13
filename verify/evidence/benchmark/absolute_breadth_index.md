# AbsoluteBreadthIndex benchmark (`AbsoluteBreadthIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 28.10M | 0.029 | 34.36M | 8.222 | 231.07× | 282.47× |
| 10,000 | 0.227 | 44.09M | 0.210 | 47.66M | 82.314 | 362.89× | 392.32× |
| 100,000 | 1.999 | 50.03M | 1.997 | 50.07M | 828.019 | 414.24× | 414.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.158 | 0.297 | 1.88× |
| 1 | 5 | 0.368 | 1.124 | 3.05× |
| 1 | 10 | 0.611 | 2.165 | 3.55× |
| 10 | 1 | 0.072 | 0.290 | 4.02× |
| 10 | 5 | 0.283 | 1.712 | 6.04× |
| 10 | 10 | 0.612 | 2.894 | 4.73× |
| 100 | 1 | 0.068 | 1.035 | 15.30× |
| 100 | 5 | 0.290 | 5.547 | 19.10× |
| 100 | 10 | 0.610 | 10.831 | 17.77× |
| 1,000 | 1 | 0.102 | 8.563 | 84.09× |
| 1,000 | 5 | 0.399 | 51.241 | 128.53× |
| 1,000 | 10 | 0.753 | 90.974 | 120.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
