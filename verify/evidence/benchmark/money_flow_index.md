# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.01M | 0.011 | 87.47M | 0.038 | 3.05× | 3.33× |
| 10,000 | 0.066 | 151.02M | 0.061 | 163.66M | 0.129 | 1.95× | 2.12× |
| 100,000 | 0.656 | 152.42M | 0.595 | 167.97M | 0.929 | 1.42× | 1.56× |
| 1,000,000 | 7.756 | 128.94M | 7.502 | 133.30M | 9.340 | 1.20× | 1.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.166 | 0.151 | 0.91× |
| 1 | 5 | 0.280 | 0.544 | 1.94× |
| 1 | 10 | 0.493 | 0.939 | 1.91× |
| 10 | 1 | 0.058 | 0.100 | 1.73× |
| 10 | 5 | 0.226 | 0.452 | 2.00× |
| 10 | 10 | 0.533 | 0.986 | 1.85× |
| 100 | 1 | 0.057 | 0.089 | 1.56× |
| 100 | 5 | 0.226 | 0.457 | 2.02× |
| 100 | 10 | 0.512 | 1.029 | 2.01× |
| 1,000 | 1 | 0.061 | 0.105 | 1.73× |
| 1,000 | 5 | 0.272 | 0.532 | 1.95× |
| 1,000 | 10 | 0.529 | 1.116 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
