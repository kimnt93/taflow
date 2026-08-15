# GoldenPocket benchmark (`GoldenPocket` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.65M | 0.014 | 72.07M | 0.500 | 31.79× | 36.00× |
| 10,000 | 0.138 | 72.54M | 0.131 | 76.44M | 3.874 | 28.10× | 29.61× |
| 100,000 | 1.278 | 78.25M | 1.277 | 78.34M | 44.301 | 34.67× | 34.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.223 | 2.94× |
| 1 | 5 | 0.233 | 0.820 | 3.51× |
| 1 | 10 | 0.442 | 2.008 | 4.54× |
| 10 | 1 | 0.050 | 0.165 | 3.29× |
| 10 | 5 | 0.191 | 0.870 | 4.55× |
| 10 | 10 | 0.477 | 2.178 | 4.56× |
| 100 | 1 | 0.053 | 0.215 | 4.02× |
| 100 | 5 | 0.197 | 1.096 | 5.57× |
| 100 | 10 | 0.461 | 2.388 | 5.18× |
| 1,000 | 1 | 0.059 | 0.759 | 12.91× |
| 1,000 | 5 | 0.251 | 3.211 | 12.80× |
| 1,000 | 10 | 0.445 | 6.500 | 14.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
