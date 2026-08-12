# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.46M | 0.010 | 98.33M | 0.036 | 3.08× | 3.50× |
| 10,000 | 0.089 | 111.92M | 0.085 | 118.31M | 0.108 | 1.21× | 1.28× |
| 100,000 | 0.850 | 117.69M | 0.827 | 120.92M | 0.826 | 0.97× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.140 | 1.42× |
| 1 | 5 | 0.290 | 0.426 | 1.47× |
| 1 | 10 | 0.470 | 0.843 | 1.79× |
| 10 | 1 | 0.049 | 0.085 | 1.71× |
| 10 | 5 | 0.233 | 0.452 | 1.94× |
| 10 | 10 | 0.476 | 0.945 | 1.99× |
| 100 | 1 | 0.060 | 0.082 | 1.37× |
| 100 | 5 | 0.249 | 0.416 | 1.67× |
| 100 | 10 | 0.576 | 0.951 | 1.65× |
| 1,000 | 1 | 0.058 | 0.096 | 1.66× |
| 1,000 | 5 | 0.235 | 0.473 | 2.01× |
| 1,000 | 10 | 0.532 | 1.096 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
