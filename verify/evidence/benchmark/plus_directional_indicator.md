# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.31M | 0.011 | 89.66M | 0.040 | 3.02× | 3.60× |
| 10,000 | 0.071 | 140.75M | 0.066 | 150.50M | 0.106 | 1.49× | 1.60× |
| 100,000 | 0.662 | 151.07M | 0.811 | 123.35M | 0.981 | 1.48× | 1.21× |
| 1,000,000 | 7.338 | 136.27M | 6.340 | 157.74M | 7.274 | 0.99× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.139 | 0.150 | 1.08× |
| 1 | 5 | 0.298 | 0.520 | 1.74× |
| 1 | 10 | 0.590 | 1.019 | 1.73× |
| 10 | 1 | 0.054 | 0.104 | 1.92× |
| 10 | 5 | 0.289 | 0.528 | 1.83× |
| 10 | 10 | 0.617 | 1.195 | 1.94× |
| 100 | 1 | 0.088 | 0.125 | 1.43× |
| 100 | 5 | 0.321 | 0.525 | 1.63× |
| 100 | 10 | 0.815 | 1.461 | 1.79× |
| 1,000 | 1 | 0.083 | 0.107 | 1.29× |
| 1,000 | 5 | 0.311 | 0.612 | 1.96× |
| 1,000 | 10 | 0.630 | 1.227 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
