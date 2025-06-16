/// Regressão Linear Pura para séries temporais

#[derive(Debug, PartialEq)]
pub struct RegressaoLinear {
    pub intercept: f64,
    pub slope: f64,
}

impl RegressaoLinear {
    /// Ajusta a regressão linear pura para os dados y (com x = índice 0,1,2,...)
    /// Retorna None se os dados forem inválidos ou divisão por zero.
    pub fn ajustar(y: &[f64]) -> Option<Self> {
        let n = y.len();
        if n == 0 {
            return None;
        }

        let x: Vec<f64> = (0..n).map(|i| i as f64).collect();

        let sum_x: f64 = x.iter().sum();
        let sum_y: f64 = y.iter().sum();

        let mean_x = sum_x / n as f64;
        let mean_y = sum_y / n as f64;

        let mut num = 0.0;
        let mut den = 0.0;
        for i in 0..n {
            num += (x[i] - mean_x) * (y[i] - mean_y);
            den += (x[i] - mean_x).powi(2);
        }

        if den == 0.0 {
            return None;
        }

        let slope = num / den;
        let intercept = mean_y - slope * mean_x;

        Some(Self { intercept, slope })
    }

    /// Previsão para o índice x (pode ser futuro)
    pub fn prever(&self, x: f64) -> f64 {
        self.intercept + self.slope * x
    }

    /// Calcula o coeficiente de determinação R² para os dados y
    pub fn calcular_r2(&self, y: &[f64]) -> Option<f64> {
        let n = y.len();
        if n == 0 {
            return None;
        }
        let mean_y = y.iter().sum::<f64>() / n as f64;

        let mut ss_total = 0.0;
        let mut ss_res = 0.0;

        for i in 0..n {
            let y_pred = self.prever(i as f64);
            ss_total += (y[i] - mean_y).powi(2);
            ss_res += (y[i] - y_pred).powi(2);
        }

        if ss_total == 0.0 {
            return None;
        }

        Some(1.0 - ss_res / ss_total)
    }

    /// Calcula o Erro Quadrático Médio (MSE) para os dados y
    pub fn calcular_mse(&self, y: &[f64]) -> Option<f64> {
        let n = y.len();
        if n == 0 {
            return None;
        }

        let mut soma_erro = 0.0;
        for i in 0..n {
            let y_pred = self.prever(i as f64);
            soma_erro += (y[i] - y_pred).powi(2);
        }

        Some(soma_erro / n as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_ajustar_regressao_linear() {
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // y = 2x + 2 perfeito
        let modelo = RegressaoLinear::ajustar(&y).unwrap();
        assert!((modelo.intercept - 2.0).abs() < 1e-6);
        assert!((modelo.slope - 2.0).abs() < 1e-6);
    }

    #[test]
    fn teste_ajustar_regressao_linear_vazio() {
        let y: Vec<f64> = vec![];
        assert_eq!(RegressaoLinear::ajustar(&y), None);
    }

    #[test]
    fn teste_prever() {
        let modelo = RegressaoLinear { intercept: 1.0, slope: 2.0 };
        assert_eq!(modelo.prever(0.0), 1.0);
        assert_eq!(modelo.prever(3.0), 7.0);
    }

    #[test]
    fn teste_r2_perfeito() {
        let y = vec![3.0, 5.0, 7.0, 9.0];
        let modelo = RegressaoLinear::ajustar(&y).unwrap();
        let r2 = modelo.calcular_r2(&y).unwrap();
        assert!((r2 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn teste_r2_invalido() {
        let y = vec![];
        assert_eq!(RegressaoLinear::ajustar(&y).and_then(|m| m.calcular_r2(&y)), None);
    }

    #[test]
    fn teste_mse() {
        let y = vec![2.0, 4.0, 6.0];
        let modelo = RegressaoLinear::ajustar(&y).unwrap();
        let mse = modelo.calcular_mse(&y).unwrap();
        assert!(mse < 1e-6);
    }
}
