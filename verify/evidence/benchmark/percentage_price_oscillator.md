# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 163.58M | 0.005 | 190.01M | 0.042 | 6.92× | 8.03× |
| 10,000 | 0.040 | 247.22M | 0.039 | 257.53M | 0.087 | 2.15× | 2.23× |
| 100,000 | 0.392 | 254.90M | 0.369 | 270.71M | 0.554 | 1.41× | 1.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.158 | 0.141 | 0.89× |
| 1 | 5 | 0.240 | 0.482 | 2.01× |
| 1 | 10 | 0.399 | 1.049 | 2.63× |
| 10 | 1 | 0.062 | 0.101 | 1.63× |
| 10 | 5 | 0.226 | 0.499 | 2.21× |
| 10 | 10 | 0.420 | 1.015 | 2.42× |
| 100 | 1 | 0.044 | 0.102 | 2.33× |
| 100 | 5 | 0.251 | 0.528 | 2.10× |
| 100 | 10 | 0.416 | 1.006 | 2.42× |
| 1,000 | 1 | 0.052 | 0.103 | 1.97× |
| 1,000 | 5 | 0.205 | 0.515 | 2.51× |
| 1,000 | 10 | 0.514 | 1.153 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
