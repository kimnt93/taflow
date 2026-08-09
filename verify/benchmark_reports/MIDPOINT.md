# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.13M | 0.008 | 123.23M | 0.041 | 4.36× | 5.07× |
| 10,000 | 0.082 | 122.28M | 0.081 | 123.87M | 0.107 | 1.31× | 1.33× |
| 100,000 | 0.888 | 112.59M | 0.802 | 124.67M | 0.708 | 0.80× | 0.88× |
| 1,000,000 | 9.451 | 105.81M | 9.178 | 108.95M | 7.284 | 0.77× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.167 | 1.56× |
| 1 | 5 | 0.240 | 0.478 | 1.99× |
| 1 | 10 | 0.458 | 0.982 | 2.15× |
| 10 | 1 | 0.057 | 0.103 | 1.80× |
| 10 | 5 | 0.256 | 0.482 | 1.88× |
| 10 | 10 | 0.478 | 1.001 | 2.10× |
| 100 | 1 | 0.051 | 0.091 | 1.77× |
| 100 | 5 | 0.261 | 0.489 | 1.88× |
| 100 | 10 | 0.535 | 1.059 | 1.98× |
| 1,000 | 1 | 0.058 | 0.102 | 1.75× |
| 1,000 | 5 | 0.237 | 0.521 | 2.20× |
| 1,000 | 10 | 0.576 | 1.082 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
