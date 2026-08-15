# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.67M | 0.006 | 177.60M | 0.040 | 5.24× | 7.07× |
| 10,000 | 0.048 | 210.00M | 0.044 | 227.22M | 0.110 | 2.31× | 2.50× |
| 100,000 | 0.468 | 213.87M | 0.430 | 232.77M | 0.905 | 1.93× | 2.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.143 | 1.55× |
| 1 | 5 | 0.283 | 0.561 | 1.98× |
| 1 | 10 | 0.407 | 0.958 | 2.36× |
| 10 | 1 | 0.047 | 0.100 | 2.14× |
| 10 | 5 | 0.215 | 0.475 | 2.21× |
| 10 | 10 | 0.402 | 0.979 | 2.43× |
| 100 | 1 | 0.047 | 0.094 | 1.99× |
| 100 | 5 | 0.186 | 0.464 | 2.49× |
| 100 | 10 | 0.436 | 0.955 | 2.19× |
| 1,000 | 1 | 0.044 | 0.101 | 2.28× |
| 1,000 | 5 | 0.210 | 0.488 | 2.33× |
| 1,000 | 10 | 0.407 | 1.091 | 2.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
