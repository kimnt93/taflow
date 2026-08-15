# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 398.22M | 0.001 | 927.57M | 0.030 | 12.05× | 28.08× |
| 10,000 | 0.008 | 1.29G | 0.004 | 2.49G | 0.036 | 4.67× | 9.03× |
| 100,000 | 0.069 | 1.46G | 0.042 | 2.36G | 0.073 | 1.07× | 1.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.152 | 2.37× |
| 1 | 5 | 0.257 | 0.437 | 1.70× |
| 1 | 10 | 0.370 | 0.884 | 2.39× |
| 10 | 1 | 0.040 | 0.086 | 2.16× |
| 10 | 5 | 0.216 | 0.538 | 2.50× |
| 10 | 10 | 0.416 | 0.965 | 2.32× |
| 100 | 1 | 0.045 | 0.092 | 2.05× |
| 100 | 5 | 0.184 | 0.450 | 2.45× |
| 100 | 10 | 0.459 | 0.912 | 1.99× |
| 1,000 | 1 | 0.044 | 0.087 | 1.98× |
| 1,000 | 5 | 0.190 | 0.441 | 2.32× |
| 1,000 | 10 | 0.409 | 0.993 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
