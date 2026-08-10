# CumulativeProduct benchmark (`numpy.cumprod` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 200.53M | 0.004 | 255.11M | 0.017 | 3.43× | 4.36× |
| 10,000 | 0.016 | 629.99M | 0.014 | 712.78M | 0.043 | 2.69× | 3.05× |
| 100,000 | 0.131 | 762.84M | 0.107 | 934.01M | 0.247 | 1.89× | 2.31× |
| 1,000,000 | 1.945 | 514.16M | 1.184 | 844.27M | 2.402 | 1.23× | 2.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.103 | 1.33× |
| 1 | 5 | 0.248 | 0.395 | 1.59× |
| 1 | 10 | 0.471 | 3.245 | 6.89× |
| 10 | 1 | 0.092 | 0.120 | 1.31× |
| 10 | 5 | 1.025 | 0.752 | 0.73× |
| 10 | 10 | 1.565 | 1.017 | 0.65× |
| 100 | 1 | 0.069 | 0.091 | 1.33× |
| 100 | 5 | 0.305 | 0.811 | 2.66× |
| 100 | 10 | 1.493 | 0.707 | 0.47× |
| 1,000 | 1 | 0.052 | 0.071 | 1.36× |
| 1,000 | 5 | 0.289 | 0.407 | 1.41× |
| 1,000 | 10 | 0.548 | 0.940 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
