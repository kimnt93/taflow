# RollingArgmax benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 117.60M | 0.008 | 127.40M | 0.038 | 4.42× | 4.78× |
| 10,000 | 0.058 | 171.51M | 0.054 | 185.06M | 0.100 | 1.71× | 1.84× |
| 100,000 | 0.554 | 180.37M | 0.555 | 180.09M | 0.747 | 1.35× | 1.34× |
| 1,000,000 | 5.844 | 171.12M | 5.475 | 182.65M | 6.914 | 1.18× | 1.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.124 | 0.96× |
| 1 | 5 | 0.227 | 0.460 | 2.02× |
| 1 | 10 | 0.502 | 0.984 | 1.96× |
| 10 | 1 | 0.051 | 0.091 | 1.80× |
| 10 | 5 | 0.216 | 0.430 | 2.00× |
| 10 | 10 | 0.474 | 0.983 | 2.07× |
| 100 | 1 | 0.050 | 0.091 | 1.82× |
| 100 | 5 | 0.224 | 0.448 | 2.01× |
| 100 | 10 | 0.486 | 0.957 | 1.97× |
| 1,000 | 1 | 0.059 | 0.100 | 1.70× |
| 1,000 | 5 | 0.255 | 0.507 | 1.99× |
| 1,000 | 10 | 0.512 | 1.076 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
