# SqueezePro benchmark (`squeeze_pro` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.23M | 0.049 | 20.28M | 8.765 | 159.80× | 177.72× |
| 10,000 | 0.466 | 21.47M | 0.424 | 23.57M | 12.624 | 27.10× | 29.76× |
| 100,000 | 4.737 | 21.11M | 5.105 | 19.59M | 55.262 | 11.66× | 10.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.467 | 3.90× |
| 1 | 5 | 0.379 | 1.940 | 5.12× |
| 1 | 10 | 0.525 | 3.865 | 7.36× |
| 10 | 1 | 0.057 | 0.383 | 6.66× |
| 10 | 5 | 0.270 | 1.970 | 7.30× |
| 10 | 10 | 0.540 | 3.918 | 7.26× |
| 100 | 1 | 0.062 | 8.694 | 141.01× |
| 100 | 5 | 0.286 | 45.732 | 159.74× |
| 100 | 10 | 0.582 | 89.701 | 154.21× |
| 1,000 | 1 | 0.124 | 9.218 | 74.59× |
| 1,000 | 5 | 0.322 | 49.885 | 155.00× |
| 1,000 | 10 | 0.670 | 101.789 | 152.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
