# ZeroLagExponentialMovingAverage benchmark (`ZLEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.14M | 0.006 | 168.10M | 0.153 | 22.02× | 25.68× |
| 10,000 | 0.055 | 183.05M | 0.045 | 221.85M | 0.488 | 8.94× | 10.83× |
| 100,000 | 0.470 | 212.99M | 0.447 | 223.51M | 3.955 | 8.42× | 8.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.277 | 2.86× |
| 1 | 5 | 0.248 | 0.975 | 3.92× |
| 1 | 10 | 0.460 | 2.177 | 4.74× |
| 10 | 1 | 0.047 | 0.186 | 3.99× |
| 10 | 5 | 0.195 | 1.005 | 5.15× |
| 10 | 10 | 0.400 | 2.221 | 5.55× |
| 100 | 1 | 0.047 | 0.197 | 4.23× |
| 100 | 5 | 0.222 | 1.062 | 4.79× |
| 100 | 10 | 0.428 | 2.266 | 5.29× |
| 1,000 | 1 | 0.058 | 0.253 | 4.34× |
| 1,000 | 5 | 0.240 | 1.162 | 4.85× |
| 1,000 | 10 | 0.468 | 2.668 | 5.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
