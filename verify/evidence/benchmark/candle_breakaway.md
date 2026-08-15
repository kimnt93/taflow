# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.55M | 0.004 | 273.57M | 0.029 | 4.25× | 7.98× |
| 10,000 | 0.066 | 150.43M | 0.061 | 163.16M | 0.094 | 1.41× | 1.53× |
| 100,000 | 0.842 | 118.78M | 0.795 | 125.72M | 0.635 | 0.75× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | 0.118 | 0.80× |
| 1 | 5 | 0.372 | 0.442 | 1.19× |
| 1 | 10 | 0.399 | 0.883 | 2.21× |
| 10 | 1 | 0.043 | 0.087 | 2.03× |
| 10 | 5 | 0.189 | 0.418 | 2.21× |
| 10 | 10 | 0.392 | 0.872 | 2.22× |
| 100 | 1 | 0.041 | 0.096 | 2.33× |
| 100 | 5 | 0.187 | 0.421 | 2.25× |
| 100 | 10 | 0.400 | 0.887 | 2.22× |
| 1,000 | 1 | 0.049 | 0.098 | 1.99× |
| 1,000 | 5 | 0.183 | 0.448 | 2.45× |
| 1,000 | 10 | 0.394 | 0.950 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
