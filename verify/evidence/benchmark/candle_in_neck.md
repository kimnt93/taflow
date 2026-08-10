# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.40M | 0.023 | 42.87M | 0.041 | 1.99× | 1.76× |
| 10,000 | 0.175 | 57.04M | 0.227 | 43.98M | 0.188 | 1.07× | 0.83× |
| 100,000 | 1.756 | 56.95M | 1.847 | 54.14M | 1.255 | 0.71× | 0.68× |
| 1,000,000 | 17.303 | 57.79M | 18.241 | 54.82M | 11.270 | 0.65× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.177 | 1.79× |
| 1 | 5 | 0.345 | 0.574 | 1.67× |
| 1 | 10 | 0.566 | 0.953 | 1.68× |
| 10 | 1 | 0.057 | 0.102 | 1.80× |
| 10 | 5 | 0.340 | 0.551 | 1.62× |
| 10 | 10 | 0.629 | 0.983 | 1.56× |
| 100 | 1 | 0.069 | 0.106 | 1.55× |
| 100 | 5 | 0.335 | 0.547 | 1.63× |
| 100 | 10 | 0.696 | 0.948 | 1.36× |
| 1,000 | 1 | 0.083 | 0.100 | 1.20× |
| 1,000 | 5 | 0.308 | 0.623 | 2.03× |
| 1,000 | 10 | 0.670 | 1.095 | 1.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
