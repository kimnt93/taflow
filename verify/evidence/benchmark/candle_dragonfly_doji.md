# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.03M | 0.005 | 192.02M | 0.034 | 4.02× | 6.53× |
| 10,000 | 0.072 | 138.97M | 0.068 | 146.48M | 0.104 | 1.44× | 1.52× |
| 100,000 | 0.774 | 129.13M | 0.739 | 135.29M | 0.763 | 0.98× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.123 | 1.08× |
| 1 | 5 | 0.254 | 0.530 | 2.09× |
| 1 | 10 | 0.413 | 0.914 | 2.22× |
| 10 | 1 | 0.040 | 0.093 | 2.31× |
| 10 | 5 | 0.194 | 0.429 | 2.22× |
| 10 | 10 | 0.413 | 0.939 | 2.27× |
| 100 | 1 | 0.042 | 0.092 | 2.20× |
| 100 | 5 | 0.188 | 0.420 | 2.24× |
| 100 | 10 | 0.389 | 0.917 | 2.36× |
| 1,000 | 1 | 0.062 | 0.108 | 1.73× |
| 1,000 | 5 | 0.241 | 0.473 | 1.96× |
| 1,000 | 10 | 0.434 | 0.965 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
