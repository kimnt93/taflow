# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.47M | 0.009 | 106.00M | 0.033 | 3.01× | 3.53× |
| 10,000 | 0.080 | 125.50M | 0.085 | 117.56M | 0.101 | 1.26× | 1.18× |
| 100,000 | 0.821 | 121.75M | 0.786 | 127.15M | 0.737 | 0.90× | 0.94× |
| 1,000,000 | 8.852 | 112.97M | 8.504 | 117.59M | 7.270 | 0.82× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.113 | 1.37× |
| 1 | 5 | 0.274 | 0.484 | 1.77× |
| 1 | 10 | 0.520 | 1.027 | 1.97× |
| 10 | 1 | 0.052 | 0.088 | 1.69× |
| 10 | 5 | 0.267 | 0.481 | 1.80× |
| 10 | 10 | 0.528 | 0.981 | 1.86× |
| 100 | 1 | 0.051 | 0.092 | 1.80× |
| 100 | 5 | 0.260 | 0.463 | 1.78× |
| 100 | 10 | 0.512 | 0.991 | 1.94× |
| 1,000 | 1 | 0.062 | 0.098 | 1.58× |
| 1,000 | 5 | 0.261 | 0.522 | 2.00× |
| 1,000 | 10 | 0.530 | 1.085 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
