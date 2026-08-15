# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 117.82M | 0.005 | 204.14M | 0.042 | 4.96× | 8.59× |
| 10,000 | 0.082 | 122.02M | 0.072 | 138.85M | 0.157 | 1.91× | 2.18× |
| 100,000 | 0.935 | 106.98M | 0.920 | 108.70M | 1.316 | 1.41× | 1.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.123 | 1.72× |
| 1 | 5 | 0.211 | 0.466 | 2.21× |
| 1 | 10 | 0.425 | 0.929 | 2.18× |
| 10 | 1 | 0.048 | 0.097 | 1.99× |
| 10 | 5 | 0.189 | 0.427 | 2.26× |
| 10 | 10 | 0.389 | 0.928 | 2.39× |
| 100 | 1 | 0.049 | 0.098 | 2.02× |
| 100 | 5 | 0.191 | 0.433 | 2.26× |
| 100 | 10 | 0.429 | 0.960 | 2.23× |
| 1,000 | 1 | 0.058 | 0.106 | 1.84× |
| 1,000 | 5 | 0.195 | 0.522 | 2.68× |
| 1,000 | 10 | 0.430 | 1.104 | 2.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
