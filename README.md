# Regressão Linear Pura para Séries Temporais em Rust

## Sobre

Projeto para análise e previsão de séries temporais usando regressão linear pura. Implementação feita em Rust sem uso de bibliotecas externas para cálculo matemático.

## Funcionalidades

- Ajuste da regressão linear simples (y = intercept + slope * x)
- Cálculo do coeficiente de determinação R²
- Cálculo do Erro Quadrático Médio (MSE)
- Previsão para períodos futuros

## Como usar

```rust
use regressao_linear::RegressaoLinear;

let dados = vec![2.0, 4.0, 6.0, 8.0];

// Ajusta o modelo
let modelo = RegressaoLinear::ajustar(&dados).unwrap();

// Previsão para o próximo índice
let previsao = modelo.prever(dados.len() as f64);

println!("Previsão: {:.2}", previsao);

// Métricas
println!("R²: {:.4}", modelo.calcular_r2(&dados).unwrap());
println!("MSE: {:.4}", modelo.calcular_mse(&dados).unwrap());
```


## Rodar testes
```bash
cargo test
```
