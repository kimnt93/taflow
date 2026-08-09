# SignedPower benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 195.42M | 0.004 | 226.08M | 0.005 | 1.03× | 1.19× |
| 10,000 | 0.024 | 421.48M | 0.022 | 459.59M | 0.023 | 0.99× | 1.07× |
| 100,000 | 0.289 | 346.16M | 0.195 | 513.35M | 0.189 | 0.65× | 0.97× |
| 1,000,000 | 2.588 | 386.45M | 2.158 | 463.48M | 3.255 | 1.26× | 1.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.076 | 0.75× |
| 1 | 5 | 0.266 | 0.231 | 0.87× |
| 1 | 10 | 0.473 | 0.435 | 0.92× |
| 10 | 1 | 0.050 | 0.049 | 0.96× |
| 10 | 5 | 0.239 | 0.213 | 0.89× |
| 10 | 10 | 0.543 | 0.480 | 0.88× |
| 100 | 1 | 0.049 | 0.048 | 0.98× |
| 100 | 5 | 0.248 | 0.215 | 0.87× |
| 100 | 10 | 0.518 | 0.485 | 0.94× |
| 1,000 | 1 | 0.058 | 0.055 | 0.96× |
| 1,000 | 5 | 0.251 | 0.276 | 1.10× |
| 1,000 | 10 | 0.506 | 0.533 | 1.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
