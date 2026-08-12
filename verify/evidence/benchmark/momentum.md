# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 218.28M | 0.004 | 262.40M | 0.032 | 7.03× | 8.45× |
| 10,000 | 0.021 | 471.50M | 0.018 | 565.13M | 0.035 | 1.66× | 1.99× |
| 100,000 | 0.185 | 539.93M | 0.159 | 629.95M | 0.063 | 0.34× | 0.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.105 | 1.17× |
| 1 | 5 | 0.238 | 0.447 | 1.87× |
| 1 | 10 | 0.507 | 0.957 | 1.89× |
| 10 | 1 | 0.051 | 0.085 | 1.68× |
| 10 | 5 | 0.219 | 0.445 | 2.03× |
| 10 | 10 | 0.462 | 0.993 | 2.15× |
| 100 | 1 | 0.078 | 0.096 | 1.22× |
| 100 | 5 | 0.245 | 0.448 | 1.83× |
| 100 | 10 | 0.457 | 0.901 | 1.97× |
| 1,000 | 1 | 0.048 | 0.090 | 1.86× |
| 1,000 | 5 | 0.235 | 0.482 | 2.05× |
| 1,000 | 10 | 0.462 | 0.906 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
