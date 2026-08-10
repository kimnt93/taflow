# RollingMinimum benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.38M | 0.006 | 161.39M | 0.041 | 6.06× | 6.59× |
| 10,000 | 0.039 | 256.51M | 0.036 | 280.02M | 0.082 | 2.11× | 2.30× |
| 100,000 | 0.410 | 244.04M | 0.367 | 272.60M | 0.567 | 1.38× | 1.55× |
| 1,000,000 | 4.854 | 206.01M | 4.206 | 237.75M | 5.283 | 1.09× | 1.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.121 | 1.16× |
| 1 | 5 | 0.299 | 0.574 | 1.92× |
| 1 | 10 | 0.590 | 1.079 | 1.83× |
| 10 | 1 | 0.051 | 0.094 | 1.82× |
| 10 | 5 | 0.246 | 0.510 | 2.07× |
| 10 | 10 | 0.550 | 1.168 | 2.12× |
| 100 | 1 | 0.050 | 0.093 | 1.88× |
| 100 | 5 | 0.262 | 0.471 | 1.80× |
| 100 | 10 | 0.604 | 1.176 | 1.95× |
| 1,000 | 1 | 0.072 | 0.107 | 1.49× |
| 1,000 | 5 | 0.252 | 0.500 | 1.98× |
| 1,000 | 10 | 0.525 | 1.166 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
