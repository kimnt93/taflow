# UpDownVolumeRatio benchmark (`UpDownVolumeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.61M | 0.005 | 186.99M | 4.392 | 648.31× | 821.25× |
| 10,000 | 0.026 | 379.69M | 0.023 | 426.89M | 43.568 | 1654.23× | 1859.89× |
| 100,000 | 0.219 | 456.47M | 0.195 | 512.17M | 425.591 | 1942.68× | 2179.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.255 | 3.21× |
| 1 | 5 | 0.331 | 1.342 | 4.06× |
| 1 | 10 | 0.744 | 2.169 | 2.91× |
| 10 | 1 | 0.059 | 0.277 | 4.69× |
| 10 | 5 | 0.304 | 1.593 | 5.24× |
| 10 | 10 | 0.487 | 2.563 | 5.26× |
| 100 | 1 | 0.054 | 0.629 | 11.76× |
| 100 | 5 | 0.245 | 3.174 | 12.98× |
| 100 | 10 | 0.493 | 6.540 | 13.27× |
| 1,000 | 1 | 0.056 | 4.600 | 82.06× |
| 1,000 | 5 | 0.271 | 32.592 | 120.40× |
| 1,000 | 10 | 1.172 | 49.860 | 42.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
