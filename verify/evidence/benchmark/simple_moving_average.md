# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 189.35M | 0.004 | 240.15M | 0.035 | 6.64× | 8.42× |
| 10,000 | 0.024 | 424.64M | 0.021 | 469.39M | 0.051 | 2.18× | 2.41× |
| 100,000 | 0.209 | 478.69M | 0.186 | 537.62M | 0.209 | 1.00× | 1.13× |
| 1,000,000 | 2.331 | 428.94M | 1.912 | 523.04M | 2.119 | 0.91× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.159 | 0.122 | 0.77× |
| 1 | 5 | 0.389 | 0.509 | 1.31× |
| 1 | 10 | 0.442 | 0.959 | 2.17× |
| 10 | 1 | 0.054 | 0.114 | 2.12× |
| 10 | 5 | 0.300 | 0.483 | 1.61× |
| 10 | 10 | 0.473 | 0.912 | 1.93× |
| 100 | 1 | 0.047 | 0.094 | 1.98× |
| 100 | 5 | 0.209 | 0.442 | 2.12× |
| 100 | 10 | 0.456 | 0.904 | 1.98× |
| 1,000 | 1 | 0.050 | 0.097 | 1.93× |
| 1,000 | 5 | 0.225 | 0.446 | 1.98× |
| 1,000 | 10 | 0.463 | 0.937 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
