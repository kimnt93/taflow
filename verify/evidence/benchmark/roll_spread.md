# RollSpread benchmark (`rolling Roll spread estimator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.79M | 0.044 | 22.58M | 0.245 | 4.36× | 5.53× |
| 10,000 | 0.441 | 22.70M | 0.421 | 23.76M | 1.225 | 2.78× | 2.91× |
| 100,000 | 4.163 | 24.02M | 4.516 | 22.15M | 12.445 | 2.99× | 2.76× |
| 1,000,000 | 42.316 | 23.63M | 41.839 | 23.90M | 141.496 | 3.34× | 3.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.159 | 1.69× |
| 1 | 5 | 0.297 | 0.556 | 1.87× |
| 1 | 10 | 0.455 | 1.079 | 2.37× |
| 10 | 1 | 0.046 | 0.102 | 2.24× |
| 10 | 5 | 0.220 | 0.519 | 2.35× |
| 10 | 10 | 0.476 | 1.083 | 2.27× |
| 100 | 1 | 0.051 | 0.221 | 4.30× |
| 100 | 5 | 0.246 | 1.256 | 5.10× |
| 100 | 10 | 0.490 | 2.300 | 4.70× |
| 1,000 | 1 | 0.098 | 0.327 | 3.33× |
| 1,000 | 5 | 0.243 | 1.365 | 5.61× |
| 1,000 | 10 | 0.513 | 2.828 | 5.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
