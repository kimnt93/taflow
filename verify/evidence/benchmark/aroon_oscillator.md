# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.36M | 0.028 | 35.75M | 0.035 | 1.21× | 1.26× |
| 10,000 | 0.282 | 35.52M | 0.278 | 35.98M | 0.137 | 0.49× | 0.49× |
| 100,000 | 2.754 | 36.31M | 2.752 | 36.33M | 1.069 | 0.39× | 0.39× |
| 1,000,000 | 27.843 | 35.92M | 28.963 | 34.53M | 10.439 | 0.37× | 0.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.149 | 1.20× |
| 1 | 5 | 0.267 | 0.472 | 1.77× |
| 1 | 10 | 0.476 | 0.910 | 1.91× |
| 10 | 1 | 0.050 | 0.092 | 1.83× |
| 10 | 5 | 0.216 | 0.415 | 1.92× |
| 10 | 10 | 0.509 | 0.914 | 1.80× |
| 100 | 1 | 0.059 | 0.085 | 1.44× |
| 100 | 5 | 0.244 | 0.451 | 1.85× |
| 100 | 10 | 0.482 | 0.954 | 1.98× |
| 1,000 | 1 | 0.082 | 0.109 | 1.33× |
| 1,000 | 5 | 0.239 | 0.496 | 2.07× |
| 1,000 | 10 | 0.497 | 1.041 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
