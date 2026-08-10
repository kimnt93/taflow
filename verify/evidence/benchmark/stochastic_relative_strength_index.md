# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.04M | 0.024 | 42.47M | 0.056 | 2.36× | 2.39× |
| 10,000 | 0.242 | 41.26M | 0.223 | 44.79M | 0.214 | 0.88× | 0.96× |
| 100,000 | 2.361 | 42.36M | 2.258 | 44.29M | 1.670 | 0.71× | 0.74× |
| 1,000,000 | 26.949 | 37.11M | 24.615 | 40.63M | 16.254 | 0.60× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.155 | 1.47× |
| 1 | 5 | 0.389 | 0.592 | 1.52× |
| 1 | 10 | 0.527 | 1.097 | 2.08× |
| 10 | 1 | 0.051 | 0.110 | 2.16× |
| 10 | 5 | 0.225 | 0.501 | 2.22× |
| 10 | 10 | 0.492 | 1.122 | 2.28× |
| 100 | 1 | 0.058 | 0.104 | 1.80× |
| 100 | 5 | 0.250 | 0.529 | 2.12× |
| 100 | 10 | 0.534 | 1.148 | 2.15× |
| 1,000 | 1 | 0.091 | 0.131 | 1.44× |
| 1,000 | 5 | 0.255 | 0.653 | 2.56× |
| 1,000 | 10 | 0.566 | 1.275 | 2.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
