# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.05M | 0.006 | 158.70M | 0.039 | 4.98× | 6.18× |
| 10,000 | 0.057 | 176.85M | 0.053 | 188.57M | 0.093 | 1.65× | 1.76× |
| 100,000 | 0.539 | 185.55M | 0.509 | 196.48M | 0.648 | 1.20× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.126 | 2.21× |
| 1 | 5 | 0.227 | 0.472 | 2.08× |
| 1 | 10 | 0.379 | 0.945 | 2.49× |
| 10 | 1 | 0.042 | 0.090 | 2.12× |
| 10 | 5 | 0.205 | 0.478 | 2.32× |
| 10 | 10 | 0.372 | 0.945 | 2.54× |
| 100 | 1 | 0.041 | 0.097 | 2.36× |
| 100 | 5 | 0.191 | 0.448 | 2.35× |
| 100 | 10 | 0.411 | 0.949 | 2.31× |
| 1,000 | 1 | 0.046 | 0.101 | 2.22× |
| 1,000 | 5 | 0.186 | 0.458 | 2.46× |
| 1,000 | 10 | 0.367 | 1.036 | 2.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
