# MathLog1p benchmark (`numpy.log1p` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.23M | 0.028 | 35.39M | 0.019 | 0.60× | 0.68× |
| 10,000 | 0.220 | 45.55M | 0.217 | 46.14M | 0.088 | 0.40× | 0.40× |
| 100,000 | 2.071 | 48.28M | 2.106 | 47.49M | 0.759 | 0.37× | 0.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.110 | 1.22× |
| 1 | 5 | 0.383 | 0.276 | 0.72× |
| 1 | 10 | 0.576 | 0.542 | 0.94× |
| 10 | 1 | 0.064 | 0.057 | 0.90× |
| 10 | 5 | 0.274 | 0.278 | 1.01× |
| 10 | 10 | 0.568 | 0.578 | 1.02× |
| 100 | 1 | 0.063 | 0.062 | 0.97× |
| 100 | 5 | 0.275 | 0.272 | 0.99× |
| 100 | 10 | 0.613 | 0.575 | 0.94× |
| 1,000 | 1 | 0.088 | 0.067 | 0.77× |
| 1,000 | 5 | 0.296 | 0.343 | 1.16× |
| 1,000 | 10 | 0.587 | 0.682 | 1.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
