# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.08M | 0.006 | 159.91M | 0.036 | 4.64× | 5.79× |
| 10,000 | 0.044 | 229.51M | 0.042 | 236.18M | 0.064 | 1.48× | 1.52× |
| 100,000 | 0.430 | 232.52M | 0.408 | 245.09M | 0.313 | 0.73× | 0.77× |
| 1,000,000 | 4.406 | 226.95M | 4.016 | 249.00M | 3.177 | 0.72× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.167 | 2.00× |
| 1 | 5 | 0.314 | 0.576 | 1.84× |
| 1 | 10 | 0.556 | 1.056 | 1.90× |
| 10 | 1 | 0.054 | 0.106 | 1.96× |
| 10 | 5 | 0.264 | 0.514 | 1.95× |
| 10 | 10 | 0.567 | 1.108 | 1.96× |
| 100 | 1 | 0.049 | 0.097 | 1.98× |
| 100 | 5 | 0.240 | 0.473 | 1.97× |
| 100 | 10 | 0.657 | 1.175 | 1.79× |
| 1,000 | 1 | 0.059 | 0.119 | 2.03× |
| 1,000 | 5 | 0.270 | 0.506 | 1.88× |
| 1,000 | 10 | 0.600 | 1.179 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
