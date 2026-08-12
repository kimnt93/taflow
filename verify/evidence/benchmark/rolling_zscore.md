# RollingZScore benchmark (`ZScore` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.36M | 0.032 | 30.92M | 0.194 | 5.68× | 5.98× |
| 10,000 | 0.300 | 33.32M | 0.316 | 31.66M | 0.567 | 1.89× | 1.80× |
| 100,000 | 3.019 | 33.12M | 3.069 | 32.59M | 6.035 | 2.00× | 1.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.272 | 2.49× |
| 1 | 5 | 0.278 | 1.162 | 4.18× |
| 1 | 10 | 0.514 | 2.384 | 4.64× |
| 10 | 1 | 0.062 | 0.244 | 3.94× |
| 10 | 5 | 0.238 | 1.289 | 5.42× |
| 10 | 10 | 0.509 | 2.482 | 4.87× |
| 100 | 1 | 0.054 | 0.216 | 4.02× |
| 100 | 5 | 0.222 | 1.263 | 5.70× |
| 100 | 10 | 0.577 | 2.465 | 4.27× |
| 1,000 | 1 | 0.085 | 0.262 | 3.07× |
| 1,000 | 5 | 0.231 | 1.538 | 6.65× |
| 1,000 | 10 | 0.517 | 2.818 | 5.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
