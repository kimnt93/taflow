# AbsoluteBreadthIndex benchmark (`AbsoluteBreadthIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.84M | 0.005 | 190.29M | 8.425 | 1220.32× | 1603.19× |
| 10,000 | 0.029 | 347.99M | 0.024 | 419.27M | 84.046 | 2924.71× | 3523.84× |
| 100,000 | 0.229 | 435.88M | 0.200 | 500.28M | 846.494 | 3689.67× | 4234.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.256 | 2.92× |
| 1 | 5 | 0.386 | 1.158 | 3.00× |
| 1 | 10 | 0.497 | 2.100 | 4.22× |
| 10 | 1 | 0.052 | 0.299 | 5.71× |
| 10 | 5 | 0.225 | 1.745 | 7.76× |
| 10 | 10 | 0.474 | 2.957 | 6.24× |
| 100 | 1 | 0.055 | 1.098 | 20.06× |
| 100 | 5 | 0.235 | 5.820 | 24.80× |
| 100 | 10 | 0.508 | 11.392 | 22.41× |
| 1,000 | 1 | 0.058 | 9.020 | 155.37× |
| 1,000 | 5 | 0.295 | 45.774 | 155.38× |
| 1,000 | 10 | 0.597 | 95.273 | 159.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
