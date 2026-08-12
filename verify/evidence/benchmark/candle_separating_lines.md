# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.69M | 0.017 | 58.87M | 0.038 | 1.94× | 2.25× |
| 10,000 | 0.141 | 70.72M | 0.130 | 77.15M | 0.127 | 0.90× | 0.98× |
| 100,000 | 1.317 | 75.91M | 1.331 | 75.13M | 1.046 | 0.79× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.108 | 0.92× |
| 1 | 5 | 0.295 | 0.449 | 1.53× |
| 1 | 10 | 0.578 | 0.983 | 1.70× |
| 10 | 1 | 0.056 | 0.085 | 1.51× |
| 10 | 5 | 0.265 | 0.441 | 1.66× |
| 10 | 10 | 0.557 | 0.982 | 1.76× |
| 100 | 1 | 0.057 | 0.088 | 1.53× |
| 100 | 5 | 0.247 | 0.435 | 1.76× |
| 100 | 10 | 0.528 | 0.911 | 1.73× |
| 1,000 | 1 | 0.076 | 0.100 | 1.31× |
| 1,000 | 5 | 0.296 | 0.499 | 1.68× |
| 1,000 | 10 | 0.560 | 1.001 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
