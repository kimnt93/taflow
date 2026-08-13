# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.37M | 0.038 | 26.10M | 0.053 | 1.14× | 1.39× |
| 10,000 | 0.343 | 29.15M | 0.312 | 32.06M | 0.133 | 0.39× | 0.43× |
| 100,000 | 4.086 | 24.48M | 2.959 | 33.80M | 1.585 | 0.39× | 0.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.153 | 0.175 | 1.15× |
| 1 | 5 | 0.344 | 0.533 | 1.55× |
| 1 | 10 | 0.615 | 1.074 | 1.74× |
| 10 | 1 | 0.073 | 0.108 | 1.48× |
| 10 | 5 | 0.303 | 0.507 | 1.67× |
| 10 | 10 | 0.631 | 1.028 | 1.63× |
| 100 | 1 | 0.073 | 0.114 | 1.57× |
| 100 | 5 | 0.302 | 0.521 | 1.73× |
| 100 | 10 | 0.645 | 1.371 | 2.13× |
| 1,000 | 1 | 0.114 | 0.149 | 1.31× |
| 1,000 | 5 | 0.331 | 0.657 | 1.98× |
| 1,000 | 10 | 0.697 | 1.241 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
