use regressao_linear::RegressaoLinear;

fn main() {
    let dados = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    let modelo = RegressaoLinear::ajustar(&dados).expect("Dados inválidos");

    println!("Modelo ajustado: intercept = {:.4}, slope = {:.4}", modelo.intercept, modelo.slope);

    let prox_indice = dados.len() as f64;
    let previsao = modelo.prever(prox_indice);
    println!("Previsão para índice {}: {:.4}", prox_indice, previsao);

    let r2 = modelo.calcular_r2(&dados).unwrap();
    let mse = modelo.calcular_mse(&dados).unwrap();

    println!("R² = {:.4}, MSE = {:.4}", r2, mse);
}
