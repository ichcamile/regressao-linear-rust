# Regressão Linear Pura para Séries Temporais em Rust

## Sobre

Este projeto implementa uma análise e previsão de séries temporais utilizando regressão linear simples, feita de forma “pura” em Rust, ou seja, sem usar crates externos para cálculos matemáticos ou estatísticos. O foco está em eficiência, clareza e controle total sobre o código.

Ideal para entender os fundamentos da regressão linear aplicada a séries temporais e para uso em contextos onde dependências externas devem ser evitadas.

---

## Funcionalidades

* **Ajuste de regressão linear simples:** cálculo dos coeficientes `intercept` e `slope` para melhor ajustar uma reta aos dados da série temporal.
* **Cálculo do coeficiente de determinação (R²):** mede a qualidade do ajuste da regressão.
* **Cálculo do Erro Quadrático Médio (MSE):** avalia a precisão do modelo, quantificando o erro médio das previsões.
* **Previsão para períodos futuros:** utiliza os coeficientes calculados para estimar valores futuros da série temporal.

---

## Como usar

No seu código Rust, importe o módulo e use os métodos da struct `RegressaoLinear` para ajustar o modelo, calcular métricas e gerar previsões.

```rust
use regressao_linear::RegressaoLinear;

fn main() {
    let dados = vec![2.0, 4.0, 6.0, 8.0];

    // Ajusta o modelo de regressão linear aos dados
    let modelo = RegressaoLinear::ajustar(&dados).expect("Falha ao ajustar o modelo");

    // Faz previsão para o próximo ponto (índice 4)
    let previsao = modelo.prever(dados.len() as f64);

    println!("Previsão para o próximo ponto: {:.2}", previsao);

    // Calcula e exibe métricas de avaliação
    println!("Coeficiente de determinação (R²): {:.4}", modelo.calcular_r2(&dados).unwrap());
    println!("Erro Quadrático Médio (MSE): {:.4}", modelo.calcular_mse(&dados).unwrap());
}
```

---

## Rodar testes

Para executar os testes unitários e garantir que tudo está funcionando corretamente, rode:

```bash
cargo test
```

Os testes cobrem:

* Ajuste correto dos coeficientes da regressão linear.
* Cálculo correto das métricas R² e MSE.
* Verificação da função de previsão.
* Tratamento de entradas inválidas (ex: vetor vazio).

---

## Estrutura do Código

* `RegressaoLinear`: Struct principal que guarda os coeficientes do modelo (`intercept` e `slope`).
* `ajustar`: Método estático que calcula os coeficientes a partir dos dados da série.
* `calcular_r2`: Método que calcula o coeficiente de determinação para avaliar a qualidade do ajuste.
* `calcular_mse`: Método que calcula o erro quadrático médio para medir o erro das previsões.
* `prever`: Método que usa os coeficientes para estimar o valor da série em um índice futuro.
