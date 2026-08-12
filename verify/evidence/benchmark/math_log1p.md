# MathLog1p benchmark (`numpy.log1p` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.62M | 0.010 | 104.21M | 0.019 | 1.85× | 1.97× |
| 10,000 | 0.082 | 122.61M | 0.078 | 128.64M | 0.115 | 1.41× | 1.48× |
| 100,000 | 0.768 | 130.22M | 0.727 | 137.54M | 0.759 | 0.99× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.087 | 0.92× |
| 1 | 5 | 0.270 | 0.274 | 1.02× |
| 1 | 10 | 0.441 | 0.550 | 1.25× |
| 10 | 1 | 0.047 | 0.057 | 1.22× |
| 10 | 5 | 0.218 | 0.266 | 1.22× |
| 10 | 10 | 0.445 | 0.553 | 1.24× |
| 100 | 1 | 0.048 | 0.058 | 1.21× |
| 100 | 5 | 0.232 | 0.280 | 1.21× |
| 100 | 10 | 0.468 | 0.602 | 1.29× |
| 1,000 | 1 | 0.055 | 0.069 | 1.26× |
| 1,000 | 5 | 0.222 | 0.333 | 1.50× |
| 1,000 | 10 | 0.491 | 0.730 | 1.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
