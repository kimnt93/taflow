# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.92M | 0.046 | 21.73M | 0.074 | 1.55× | 1.61× |
| 10,000 | 0.447 | 22.39M | 0.439 | 22.77M | 0.488 | 1.09× | 1.11× |
| 100,000 | 4.497 | 22.24M | 4.340 | 23.04M | 4.219 | 0.94× | 0.97× |
| 1,000,000 | 47.160 | 21.20M | 44.417 | 22.51M | 46.087 | 0.98× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.150 | 1.19× |
| 1 | 5 | 0.280 | 0.532 | 1.90× |
| 1 | 10 | 0.556 | 1.081 | 1.94× |
| 10 | 1 | 0.065 | 0.099 | 1.53× |
| 10 | 5 | 0.259 | 0.500 | 1.93× |
| 10 | 10 | 0.560 | 1.178 | 2.10× |
| 100 | 1 | 0.060 | 0.106 | 1.76× |
| 100 | 5 | 0.271 | 0.512 | 1.89× |
| 100 | 10 | 0.544 | 1.123 | 2.06× |
| 1,000 | 1 | 0.104 | 0.152 | 1.46× |
| 1,000 | 5 | 0.272 | 0.788 | 2.90× |
| 1,000 | 10 | 0.590 | 1.575 | 2.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
