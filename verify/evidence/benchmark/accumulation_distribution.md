# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 224.95M | 0.003 | 383.15M | 0.032 | 7.14× | 12.16× |
| 10,000 | 0.020 | 492.87M | 0.017 | 582.57M | 0.045 | 2.20× | 2.60× |
| 100,000 | 0.189 | 529.44M | 0.157 | 637.37M | 0.149 | 0.79× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.099 | 1.23× |
| 1 | 5 | 0.248 | 0.459 | 1.85× |
| 1 | 10 | 0.365 | 0.897 | 2.46× |
| 10 | 1 | 0.046 | 0.118 | 2.54× |
| 10 | 5 | 0.177 | 0.411 | 2.32× |
| 10 | 10 | 0.379 | 0.860 | 2.27× |
| 100 | 1 | 0.040 | 0.089 | 2.23× |
| 100 | 5 | 0.171 | 0.408 | 2.39× |
| 100 | 10 | 0.413 | 0.888 | 2.15× |
| 1,000 | 1 | 0.044 | 0.088 | 2.01× |
| 1,000 | 5 | 0.202 | 0.446 | 2.21× |
| 1,000 | 10 | 0.405 | 0.955 | 2.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
