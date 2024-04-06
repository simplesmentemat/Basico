
//1 - Faça um programa que tenha uma função que recebe um array de inteiros com sinal (aceite números negativos) e devolva a soma dos valores deste array e exiba no console. ✅
//2 - Faça um programa que calcule a média entre quatro notas e informe se foi aprovado ou não e a média, para ser aprovado a média deve ser maior ou igual a 7. ✅
//3 - Faça um programa que percorra um vetor com valores inteiros e verifique quais múltiplos de 2.

fn array_calc(array: &[i16]) -> i16 {
    let mut soma = 0;
    for &i in array {
        soma += i;
    }
    soma 
}

fn mean_calc(n1: i8, n2: i8, n3: i8, n4: i8) -> &'static str {
    let mean = (n1 + n2 + n3 + n4) / 4;
    if mean > 7 {
        "Aprovado"
    } else {
        "Não Aprovado"
    }
}

fn primos_check(numeros: &Vec<i32>) -> Vec<i32> {
    numeros.iter().filter(|&&x| x % 2 == 0).cloned().collect()
}

fn main() {

    //Desafio 1
    let array: [i16; 10] = [100, 2, 3, 4, 5, -1, -2, -3, -4, -5];
    let soma = array_calc(&array);
    println!("Soma: {}", soma);

    //Desafio2
    let media_nota = mean_calc(8,5,3,2);
    println!("Foi aprovado?: {}", media_nota);

    //Deasfio 3
    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Números múltiplos de 2 no vetor: {:?}", primos_check(&numeros));
}