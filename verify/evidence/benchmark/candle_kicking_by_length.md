# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 158.10M | 0.003 | 297.85M | 0.039 | 6.10× | 11.50× |
| 10,000 | 0.068 | 147.23M | 0.063 | 159.06M | 0.170 | 2.50× | 2.70× |
| 100,000 | 0.835 | 119.73M | 0.781 | 128.06M | 1.422 | 1.70× | 1.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.110 | 1.18× |
| 1 | 5 | 0.243 | 0.478 | 1.97× |
| 1 | 10 | 0.435 | 0.951 | 2.18× |
| 10 | 1 | 0.044 | 0.087 | 1.99× |
| 10 | 5 | 0.182 | 0.414 | 2.27× |
| 10 | 10 | 0.382 | 0.887 | 2.32× |
| 100 | 1 | 0.043 | 0.084 | 1.97× |
| 100 | 5 | 0.184 | 0.426 | 2.32× |
| 100 | 10 | 0.391 | 0.893 | 2.28× |
| 1,000 | 1 | 0.049 | 0.109 | 2.23× |
| 1,000 | 5 | 0.201 | 0.499 | 2.49× |
| 1,000 | 10 | 0.404 | 1.030 | 2.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
