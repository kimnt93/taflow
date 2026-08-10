# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.55M | 0.020 | 48.83M | 0.047 | 2.27× | 2.28× |
| 10,000 | 0.180 | 55.54M | 0.176 | 56.77M | 0.147 | 0.82× | 0.83× |
| 100,000 | 1.953 | 51.20M | 1.939 | 51.58M | 1.083 | 0.55× | 0.56× |
| 1,000,000 | 19.456 | 51.40M | 19.215 | 52.04M | 11.844 | 0.61× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.128 | 1.75× |
| 1 | 5 | 0.336 | 0.608 | 1.81× |
| 1 | 10 | 0.544 | 1.233 | 2.26× |
| 10 | 1 | 0.061 | 0.109 | 1.81× |
| 10 | 5 | 0.293 | 0.602 | 2.05× |
| 10 | 10 | 0.518 | 1.133 | 2.19× |
| 100 | 1 | 0.075 | 0.134 | 1.80× |
| 100 | 5 | 0.300 | 0.572 | 1.90× |
| 100 | 10 | 0.575 | 1.135 | 1.98× |
| 1,000 | 1 | 0.081 | 0.131 | 1.62× |
| 1,000 | 5 | 0.301 | 0.597 | 1.98× |
| 1,000 | 10 | 0.580 | 1.228 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
