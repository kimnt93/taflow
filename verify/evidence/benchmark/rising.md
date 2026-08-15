# Rising benchmark (`period-over-period rising` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 482.91M | 0.001 | 1.02G | 0.028 | 13.52× | 28.58× |
| 10,000 | 0.008 | 1.31G | 0.005 | 2.01G | 0.036 | 4.67× | 7.16× |
| 100,000 | 0.072 | 1.38G | 0.050 | 2.00G | 0.116 | 1.60× | 2.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.130 | 1.33× |
| 1 | 5 | 0.274 | 0.443 | 1.62× |
| 1 | 10 | 0.385 | 0.901 | 2.34× |
| 10 | 1 | 0.044 | 0.090 | 2.06× |
| 10 | 5 | 0.172 | 0.442 | 2.58× |
| 10 | 10 | 0.389 | 0.917 | 2.36× |
| 100 | 1 | 0.043 | 0.086 | 2.01× |
| 100 | 5 | 0.178 | 0.430 | 2.42× |
| 100 | 10 | 0.356 | 0.868 | 2.44× |
| 1,000 | 1 | 0.040 | 0.094 | 2.35× |
| 1,000 | 5 | 0.171 | 0.483 | 2.83× |
| 1,000 | 10 | 0.370 | 1.005 | 2.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
