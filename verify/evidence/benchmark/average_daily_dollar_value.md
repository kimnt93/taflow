# AverageDailyDollarValue benchmark (`rolling average dollar volume` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.89M | 0.048 | 20.97M | 0.080 | 1.51× | 1.67× |
| 10,000 | 0.402 | 24.86M | 0.390 | 25.64M | 0.268 | 0.67× | 0.69× |
| 100,000 | 3.611 | 27.69M | 3.828 | 26.13M | 2.111 | 0.58× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.111 | 1.27× |
| 1 | 5 | 0.376 | 0.530 | 1.41× |
| 1 | 10 | 0.602 | 1.052 | 1.75× |
| 10 | 1 | 0.064 | 0.099 | 1.54× |
| 10 | 5 | 0.303 | 0.504 | 1.67× |
| 10 | 10 | 0.589 | 1.040 | 1.77× |
| 100 | 1 | 0.075 | 0.145 | 1.94× |
| 100 | 5 | 0.309 | 0.707 | 2.29× |
| 100 | 10 | 0.634 | 1.357 | 2.14× |
| 1,000 | 1 | 0.108 | 0.158 | 1.46× |
| 1,000 | 5 | 0.296 | 0.756 | 2.55× |
| 1,000 | 10 | 0.640 | 1.584 | 2.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
