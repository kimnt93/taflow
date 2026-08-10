# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.09M | 0.011 | 91.76M | 0.035 | 2.36× | 3.23× |
| 10,000 | 0.083 | 120.21M | 0.077 | 130.65M | 0.104 | 1.26× | 1.36× |
| 100,000 | 0.840 | 119.00M | 0.809 | 123.55M | 0.796 | 0.95× | 0.98× |
| 1,000,000 | 8.400 | 119.05M | 8.168 | 122.43M | 7.869 | 0.94× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.124 | 1.50× |
| 1 | 5 | 0.279 | 0.454 | 1.63× |
| 1 | 10 | 0.585 | 0.928 | 1.59× |
| 10 | 1 | 0.054 | 0.101 | 1.88× |
| 10 | 5 | 0.246 | 0.426 | 1.73× |
| 10 | 10 | 0.531 | 0.991 | 1.87× |
| 100 | 1 | 0.058 | 0.091 | 1.58× |
| 100 | 5 | 0.272 | 0.465 | 1.71× |
| 100 | 10 | 0.564 | 0.963 | 1.71× |
| 1,000 | 1 | 0.069 | 0.097 | 1.42× |
| 1,000 | 5 | 0.308 | 0.479 | 1.55× |
| 1,000 | 10 | 0.566 | 0.984 | 1.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
