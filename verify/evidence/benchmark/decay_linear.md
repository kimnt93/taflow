# DecayLinear benchmark (`linear decay weighted mean` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 158.10M | 0.005 | 186.67M | 0.082 | 12.97× | 15.31× |
| 10,000 | 0.037 | 273.51M | 0.035 | 287.40M | 0.271 | 7.40× | 7.77× |
| 100,000 | 0.339 | 295.29M | 0.318 | 314.09M | 2.260 | 6.67× | 7.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.145 | 1.64× |
| 1 | 5 | 0.250 | 0.555 | 2.22× |
| 1 | 10 | 0.456 | 1.153 | 2.53× |
| 10 | 1 | 0.047 | 0.103 | 2.17× |
| 10 | 5 | 0.227 | 0.547 | 2.41× |
| 10 | 10 | 0.522 | 1.160 | 2.22× |
| 100 | 1 | 0.050 | 0.150 | 2.98× |
| 100 | 5 | 0.257 | 0.729 | 2.83× |
| 100 | 10 | 0.497 | 2.103 | 4.23× |
| 1,000 | 1 | 0.084 | 0.195 | 2.31× |
| 1,000 | 5 | 0.245 | 0.772 | 3.15× |
| 1,000 | 10 | 0.557 | 1.716 | 3.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
