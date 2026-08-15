# LinearRegressionChannel benchmark (`LinRegChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.070 | 14.21M | 0.068 | 14.69M | 0.590 | 8.38× | 8.67× |
| 10,000 | 0.662 | 15.09M | 0.684 | 14.61M | 4.098 | 6.19× | 5.99× |
| 100,000 | 7.180 | 13.93M | 6.692 | 14.94M | 45.222 | 6.30× | 6.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.321 | 3.06× |
| 1 | 5 | 0.270 | 1.347 | 4.99× |
| 1 | 10 | 0.395 | 2.638 | 6.68× |
| 10 | 1 | 0.051 | 0.252 | 4.99× |
| 10 | 5 | 0.192 | 1.409 | 7.32× |
| 10 | 10 | 0.437 | 2.826 | 6.47× |
| 100 | 1 | 0.051 | 0.289 | 5.67× |
| 100 | 5 | 0.196 | 1.732 | 8.82× |
| 100 | 10 | 0.422 | 3.068 | 7.27× |
| 1,000 | 1 | 0.127 | 0.888 | 6.97× |
| 1,000 | 5 | 0.240 | 3.751 | 15.63× |
| 1,000 | 10 | 0.479 | 7.483 | 15.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
