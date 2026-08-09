# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.73M | 0.007 | 136.40M | 0.032 | 3.20× | 4.34× |
| 10,000 | 0.091 | 109.69M | 0.088 | 113.45M | 0.124 | 1.36× | 1.40× |
| 100,000 | 0.979 | 102.18M | 0.949 | 105.39M | 0.975 | 1.00× | 1.03× |
| 1,000,000 | 9.865 | 101.37M | 9.734 | 102.73M | 9.680 | 0.98× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.117 | 1.11× |
| 1 | 5 | 0.345 | 0.535 | 1.55× |
| 1 | 10 | 0.517 | 0.926 | 1.79× |
| 10 | 1 | 0.053 | 0.092 | 1.76× |
| 10 | 5 | 0.236 | 0.442 | 1.87× |
| 10 | 10 | 0.508 | 0.914 | 1.80× |
| 100 | 1 | 0.055 | 0.094 | 1.70× |
| 100 | 5 | 0.256 | 0.449 | 1.76× |
| 100 | 10 | 0.528 | 0.921 | 1.74× |
| 1,000 | 1 | 0.066 | 0.105 | 1.60× |
| 1,000 | 5 | 0.245 | 0.499 | 2.04× |
| 1,000 | 10 | 0.556 | 1.048 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
