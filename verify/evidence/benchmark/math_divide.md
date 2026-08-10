# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 211.24M | 0.004 | 281.07M | 0.028 | 5.87× | 7.81× |
| 10,000 | 0.012 | 854.75M | 0.009 | 1.17G | 0.034 | 2.87× | 3.92× |
| 100,000 | 0.073 | 1.38G | 0.050 | 2.01G | 0.081 | 1.12× | 1.63× |
| 1,000,000 | 1.202 | 831.68M | 0.861 | 1.16G | 0.897 | 0.75× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.144 | 2.21× |
| 1 | 5 | 0.319 | 0.477 | 1.50× |
| 1 | 10 | 0.474 | 0.894 | 1.88× |
| 10 | 1 | 0.050 | 0.088 | 1.77× |
| 10 | 5 | 0.247 | 0.432 | 1.75× |
| 10 | 10 | 0.500 | 0.930 | 1.86× |
| 100 | 1 | 0.050 | 0.091 | 1.81× |
| 100 | 5 | 0.216 | 0.408 | 1.89× |
| 100 | 10 | 0.482 | 0.912 | 1.89× |
| 1,000 | 1 | 0.048 | 0.088 | 1.83× |
| 1,000 | 5 | 0.219 | 0.418 | 1.91× |
| 1,000 | 10 | 0.502 | 0.920 | 1.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
