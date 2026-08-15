# GapUp benchmark (`gap up relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 184.92M | 0.004 | 262.07M | 0.024 | 4.39× | 6.22× |
| 10,000 | 0.032 | 308.25M | 0.029 | 344.52M | 0.045 | 1.40× | 1.56× |
| 100,000 | 0.278 | 359.85M | 0.253 | 395.97M | 0.257 | 0.92× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.085 | 1.25× |
| 1 | 5 | 0.235 | 0.439 | 1.87× |
| 1 | 10 | 0.393 | 0.741 | 1.89× |
| 10 | 1 | 0.045 | 0.072 | 1.61× |
| 10 | 5 | 0.184 | 0.356 | 1.93× |
| 10 | 10 | 0.385 | 0.826 | 2.15× |
| 100 | 1 | 0.046 | 0.072 | 1.58× |
| 100 | 5 | 0.182 | 0.357 | 1.96× |
| 100 | 10 | 0.411 | 0.745 | 1.81× |
| 1,000 | 1 | 0.047 | 0.082 | 1.72× |
| 1,000 | 5 | 0.188 | 0.547 | 2.91× |
| 1,000 | 10 | 0.419 | 1.151 | 2.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
