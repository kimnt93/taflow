# RollingDrawdownDuration benchmark (`DrawdownDuration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 253.60M | 0.003 | 330.32M | 0.128 | 32.38× | 42.18× |
| 10,000 | 0.028 | 360.47M | 0.025 | 407.25M | 0.424 | 15.29× | 17.27× |
| 100,000 | 0.230 | 435.06M | 0.208 | 481.02M | 3.397 | 14.78× | 16.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.238 | 3.43× |
| 1 | 5 | 0.248 | 0.773 | 3.12× |
| 1 | 10 | 0.413 | 1.981 | 4.79× |
| 10 | 1 | 0.049 | 0.160 | 3.29× |
| 10 | 5 | 0.181 | 0.800 | 4.42× |
| 10 | 10 | 0.414 | 1.763 | 4.26× |
| 100 | 1 | 0.048 | 0.161 | 3.33× |
| 100 | 5 | 0.191 | 1.077 | 5.63× |
| 100 | 10 | 0.424 | 1.725 | 4.07× |
| 1,000 | 1 | 0.049 | 0.191 | 3.88× |
| 1,000 | 5 | 0.196 | 1.218 | 6.22× |
| 1,000 | 10 | 0.477 | 2.026 | 4.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
