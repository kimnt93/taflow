# Rising benchmark (`period-over-period rising` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 16.95M | 0.055 | 18.07M | 0.028 | 0.48× | 0.51× |
| 10,000 | 0.465 | 21.50M | 0.451 | 22.17M | 0.036 | 0.08× | 0.08× |
| 100,000 | 4.393 | 22.76M | 4.391 | 22.77M | 0.118 | 0.03× | 0.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.139 | 0.130 | 0.93× |
| 1 | 5 | 0.411 | 0.473 | 1.15× |
| 1 | 10 | 0.564 | 0.903 | 1.60× |
| 10 | 1 | 0.063 | 0.096 | 1.52× |
| 10 | 5 | 0.296 | 0.453 | 1.53× |
| 10 | 10 | 0.606 | 0.907 | 1.50× |
| 100 | 1 | 0.074 | 0.090 | 1.22× |
| 100 | 5 | 0.301 | 0.430 | 1.43× |
| 100 | 10 | 0.594 | 0.903 | 1.52× |
| 1,000 | 1 | 0.114 | 0.092 | 0.81× |
| 1,000 | 5 | 0.302 | 0.467 | 1.54× |
| 1,000 | 10 | 0.644 | 1.024 | 1.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
