# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.18M | 0.009 | 108.30M | 0.041 | 4.02× | 4.44× |
| 10,000 | 0.066 | 152.35M | 0.063 | 159.92M | 0.100 | 1.52× | 1.59× |
| 100,000 | 0.616 | 162.44M | 0.585 | 171.06M | 0.721 | 1.17× | 1.23× |
| 1,000,000 | 6.500 | 153.85M | 5.984 | 167.11M | 7.869 | 1.21× | 1.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.131 | 2.11× |
| 1 | 5 | 0.236 | 0.471 | 1.99× |
| 1 | 10 | 0.525 | 1.003 | 1.91× |
| 10 | 1 | 0.051 | 0.093 | 1.81× |
| 10 | 5 | 0.234 | 0.460 | 1.96× |
| 10 | 10 | 0.506 | 1.150 | 2.27× |
| 100 | 1 | 0.058 | 0.097 | 1.67× |
| 100 | 5 | 0.223 | 0.452 | 2.02× |
| 100 | 10 | 0.462 | 1.121 | 2.43× |
| 1,000 | 1 | 0.059 | 0.103 | 1.74× |
| 1,000 | 5 | 0.278 | 0.520 | 1.87× |
| 1,000 | 10 | 0.516 | 1.086 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
