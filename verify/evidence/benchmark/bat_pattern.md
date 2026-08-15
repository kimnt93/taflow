# BatPattern benchmark (`Bat` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.33M | 0.008 | 128.06M | 0.233 | 21.71× | 29.79× |
| 10,000 | 0.092 | 108.58M | 0.087 | 115.26M | 1.513 | 16.43× | 17.44× |
| 100,000 | 0.922 | 108.49M | 0.892 | 112.16M | 13.229 | 14.35× | 14.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.201 | 1.78× |
| 1 | 5 | 0.230 | 0.823 | 3.57× |
| 1 | 10 | 0.386 | 1.711 | 4.44× |
| 10 | 1 | 0.049 | 0.169 | 3.44× |
| 10 | 5 | 0.201 | 1.088 | 5.40× |
| 10 | 10 | 0.405 | 1.715 | 4.23× |
| 100 | 1 | 0.049 | 0.182 | 3.73× |
| 100 | 5 | 0.199 | 1.159 | 5.82× |
| 100 | 10 | 0.422 | 1.919 | 4.55× |
| 1,000 | 1 | 0.060 | 0.301 | 4.98× |
| 1,000 | 5 | 0.201 | 1.741 | 8.66× |
| 1,000 | 10 | 0.447 | 3.017 | 6.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
