# MathDegrees benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 185.26M | 0.006 | 162.60M | 0.003 | 0.52× | 0.46× |
| 10,000 | 0.017 | 584.60M | 0.014 | 713.86M | 0.015 | 0.87× | 1.06× |
| 100,000 | 0.141 | 709.49M | 0.112 | 891.43M | 0.125 | 0.89× | 1.12× |
| 1,000,000 | 3.310 | 302.14M | 2.526 | 395.86M | 1.381 | 0.42× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.065 | 0.63× |
| 1 | 5 | 0.301 | 0.225 | 0.75× |
| 1 | 10 | 0.489 | 0.417 | 0.85× |
| 10 | 1 | 0.051 | 0.052 | 1.03× |
| 10 | 5 | 0.222 | 0.174 | 0.79× |
| 10 | 10 | 0.494 | 0.436 | 0.88× |
| 100 | 1 | 0.048 | 0.047 | 0.97× |
| 100 | 5 | 0.264 | 0.219 | 0.83× |
| 100 | 10 | 0.512 | 0.409 | 0.80× |
| 1,000 | 1 | 0.051 | 0.042 | 0.82× |
| 1,000 | 5 | 0.246 | 0.494 | 2.00× |
| 1,000 | 10 | 0.495 | 0.439 | 0.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
