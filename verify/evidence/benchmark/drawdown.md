# Drawdown benchmark (`drawdown from cumulative maximum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.21M | 0.027 | 37.60M | 0.021 | 0.72× | 0.79× |
| 10,000 | 0.198 | 50.52M | 0.180 | 55.57M | 0.058 | 0.29× | 0.32× |
| 100,000 | 1.722 | 58.07M | 1.821 | 54.92M | 0.442 | 0.26× | 0.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.142 | 1.66× |
| 1 | 5 | 0.495 | 0.386 | 0.78× |
| 1 | 10 | 0.557 | 0.696 | 1.25× |
| 10 | 1 | 0.063 | 0.069 | 1.08× |
| 10 | 5 | 0.270 | 0.341 | 1.26× |
| 10 | 10 | 0.561 | 0.727 | 1.30× |
| 100 | 1 | 0.068 | 0.073 | 1.08× |
| 100 | 5 | 0.286 | 0.358 | 1.25× |
| 100 | 10 | 0.614 | 0.715 | 1.16× |
| 1,000 | 1 | 0.088 | 0.074 | 0.84× |
| 1,000 | 5 | 0.287 | 0.426 | 1.49× |
| 1,000 | 10 | 0.606 | 0.951 | 1.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
