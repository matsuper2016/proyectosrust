extern crate rusty_machine;
extern crate rand;

use rand::{random, Closed01};
use std::vec::Vec;
use rusty_machine::learning::nnet::{NeuralNet, BCECriterion};
use rusty_machine::learning::toolkit::regularization::Regularization;
use rusty_machine::learning::optim::grad_desc::StochasticGD;
use rusty_machine::linalg::Matrix;
use rusty_machine::learning::SupModel;

// Compuerta AND 
fn main() {
    println!("Aprendizaje compuerta AND:");

    const THRESHOLD: f64 = 0.7;

    const SAMPLES: usize = 10000;
    println!("Generando {} datos de entrenamiento y etiquetas...", SAMPLES as u32);

    let mut input_data = Vec::with_capacity(SAMPLES * 2);
    let mut label_data = Vec::with_capacity(SAMPLES);

    for _ in 0..SAMPLES {
        // Las 2 entradas son "señales" entre 0 y 1
        let Closed01(left) = random::<Closed01<f64>>();
        let Closed01(right) = random::<Closed01<f64>>();
        input_data.push(left);
        input_data.push(right);
        if left > THRESHOLD && right > THRESHOLD {
            label_data.push(1.0);
        } else {
            label_data.push(0.0)
        }
    }

    let inputs = Matrix::new(SAMPLES, 2, input_data);
    let targets = Matrix::new(SAMPLES, 1, label_data);

    let layers = &[2, 1];
    let criterion = BCECriterion::new(Regularization::L2(0.));
    //Creando un perceptron multicapa con una capa de entrada de tamaño 2 y capa de salida tamaño 1
    //Usando una funcion de activacion Sigmoide y un gradiente descendente estocastico para entrenamiento
    let mut model = NeuralNet::new(layers, criterion, StochasticGD::default());//activacion por default sigmoide.
    println!("Entrenando...");
    // Nuestra funcion retorna un tipo Result<(), E>
    model.train(&inputs, &targets).unwrap();

    let test_cases = vec![
        0.0, 0.0,
        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        ];
    let expected = vec![
        0.0,
        0.0,
        1.0,
        0.0,
        ];
    let test_inputs = Matrix::new(test_cases.len() / 2, 2, test_cases);
    let res = model.predict(&test_inputs).unwrap();

    println!("Evaluacion o prueba...");
    let mut hits = 0;
    let mut misses = 0;
    // Evaluation
    println!("Obtenido\tEsperado");
    for (idx, prediction) in res.into_vec().iter().enumerate() {
        println!("{:.2}\t\t{}", prediction, expected[idx]);
        if (prediction - 0.5) * (expected[idx] - 0.5) > 0. {
            hits += 1;
        } else {
            misses += 1;
        }
    }

    println!("Intentos: {}, Fallos: {}", hits, misses);
    let hits_f = hits as f64;
    let total = (hits + misses) as f64;
    println!("Exactitud: {}%", (hits_f / total) * 100.);
}
