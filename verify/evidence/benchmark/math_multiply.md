# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 219.23M | 0.003 | 311.01M | 0.029 | 6.39× | 9.07× |
| 10,000 | 0.011 | 951.56M | 0.009 | 1.06G | 0.034 | 3.23× | 3.59× |
| 100,000 | 0.065 | 1.54G | 0.041 | 2.47G | 0.077 | 1.19× | 1.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.171 | 1.50× |
| 1 | 5 | 0.280 | 0.458 | 1.64× |
| 1 | 10 | 0.475 | 0.937 | 1.97× |
| 10 | 1 | 0.049 | 0.086 | 1.76× |
| 10 | 5 | 0.234 | 0.437 | 1.87× |
| 10 | 10 | 0.477 | 0.887 | 1.86× |
| 100 | 1 | 0.051 | 0.084 | 1.67× |
| 100 | 5 | 0.233 | 0.422 | 1.81× |
| 100 | 10 | 0.484 | 0.874 | 1.81× |
| 1,000 | 1 | 0.056 | 0.085 | 1.51× |
| 1,000 | 5 | 0.233 | 0.439 | 1.89× |
| 1,000 | 10 | 0.471 | 0.983 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
