# GarmanKlass benchmark (`GarmanKlassVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.88M | 0.018 | 56.47M | 0.304 | 15.16× | 17.16× |
| 10,000 | 0.173 | 57.72M | 0.161 | 62.21M | 1.556 | 8.98× | 9.68× |
| 100,000 | 1.613 | 61.99M | 1.596 | 62.67M | 13.955 | 8.65× | 8.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.197 | 0.354 | 1.80× |
| 1 | 5 | 0.266 | 1.366 | 5.13× |
| 1 | 10 | 0.403 | 2.569 | 6.38× |
| 10 | 1 | 0.048 | 0.246 | 5.15× |
| 10 | 5 | 0.195 | 1.438 | 7.36× |
| 10 | 10 | 0.416 | 2.929 | 7.04× |
| 100 | 1 | 0.051 | 0.251 | 4.94× |
| 100 | 5 | 0.201 | 1.577 | 7.86× |
| 100 | 10 | 0.426 | 2.680 | 6.29× |
| 1,000 | 1 | 0.073 | 0.384 | 5.22× |
| 1,000 | 5 | 0.241 | 2.243 | 9.30× |
| 1,000 | 10 | 0.437 | 4.263 | 9.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
