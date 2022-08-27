# Aprendendo-Rust

Este conteúdo está sendo construído a partir da leitura do livro [The Rust Programming Language](https://doc.rust-lang.org/book/) e com a contribuição dos membros do grupo no Telegram [Rust Brasil](https://t.me/rustlangbr)

# Construindo uma simples calculadora.

Nesse exemplo inicial será construído uma simples calculadora!

Para criar o projeto vamos usar o comando `cargo new --bin simple_calculator`, em seguida abra o arquivo `main.rs` para da inicio a codificação em ` simple_calculator/src/main.rs `, no escopo da função `main` vamos da inicio a implementação de nosso exemplo.

- Vamos fazer uso do macro `println!` para  exibir uma mensagem solicitando que o usuário entre com uma operação aritmética o `println!`  é usado para exbibir informações dinamicas.

    EXEMPLO DE USO:
    
    ```rust 
        fn main() {
            println!("Enter your operation: ");
        }
    ```

    Rodando o exemplo o comando  ` cargo run ` .
    
    SAÍDA:

    ```bash
        ➜  simple_calculator$ cargo run
        Compiling simple_calculator v0.1.0 (/srv/Learning-Rust/Rust Basic/simple_calculator)
            Finished dev [unoptimized + debuginfo] target(s) in 0.32s
            Running `target/debug/simple_calculator`
        Enter your operation: 
        ➜  simple_calculator$ 
    ```
    Podemos notar que o programa apenas exibiu a mensagem  "Enter your operation: " e logo em seguida encerrou impedindo que o uruario entrasse com a operação, para resolver isso vamos importar para o escopo do programa a biblioteca de entrada/saída `io` que percente a biblioteca padrão conhecida como `std`:

    ```rust 
        use std::io;
    ```
    O próximo passo é implementar `io` para que o programa fique aguardando uma operação do usuário após execução do macro `println!`.

    ```rust 
    use std::io;

    fn main() {
       
        println!("Enter your operation: ");

        let mut operation = String::new();
    }
    ```
    Antes de implementar de fato `io` precisamos alocar um espaço na mémoria para guardar o valor digitado pelo usuário para isso será necessário criar uma variável mutavél com `let mut operation = String::new();`, note que usamos `String::new()`, isso cria uma nova string vazia chamada `operation` na qual podemos carregar dados. 
    
    Agora vamos usar `io` para capturar a operação digitada pelo usuario
    ```rust
            io::stdin()
            .read_line(&mut operation)
            .expect("Failed to get operation");
    ``` 
    como parâmetro de `io::stdin().read_line(&mut operation)` passamos uma referencia da variável criada anteiormente `operation` onde `&mut` antes do nome da variável caracteriza que está fazendo referencia para uma variável mutável, caso `operation` não fosse mutavél não teria necessidade de usar o `&mut`  ficando dessa maneira `&operation`.

    Nosso programa já recebe uma entrada, porém ainda não executa operações, o próximo passao é identificar qual operação aritmética o usuario deseja, analisando a entrada que será armazenada na nossa variável `operation`, as operações devem seguir o padrão de uma calculadora real exemplo: 1+1, 2-1, 4/2, 2*2.

    Vamos então criar uma lista com os sinais aritimeticos permitidos nesse exemplo `let signals_aritimetic = ['+', '-', '/', '*'];`.
    ```rust
    fn main() {

        let signals_aritimetic = ['+', '-', '/', '*'];

        println!("Enter your operation: ");

        let mut operation = String::new();

        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to get operation");

        }
    ``` 
    Agora precisamos compara o sinal que consta em `operation` com os sinais em nossa lista, vamos usar um laço `for` e dentro do `for` verificar se o `sinal` está 
    presente na `String` alocanda em `operation`, passando o `sinal` como parâmetro de find dessa maneira `find(sinal)` .
    ```rust 
        fn main() {
            let signals_aritimetic = ['+', '-', '/', '*'];

            println!("Enter your operation: ");

            let mut operation = String::new();

            io::stdin()
                .read_line(&mut operation)
                .expect("Failed to get operation");

            for signal in signals_aritimetic {
                if let Some(_result) = operation.find(signal) {

                    }
                }
       }
    ```
para melhor o entendimento e reduzir a duplicação de código vamos criar uma função que será chama posteriormente dentro da função `fn main()`

```rust
    fn perform_operation(expression: &str, signal: char, op: impl Fn(f32, f32) -> f32) {
       
    }
 ```
 A função `fn perform_operation(expression: &str, signal: char, op: impl Fn(f32, f32) -> f32) {}`, tem como primeiro parâmetro `expression: &str` que representa a `operation` o proximo parâmetro é o `signal` que representa o tipo de operação e `op` que implementa uma função que recebe dois `f32` e retorna um `f32` que representa o resultado da operação como inidicado com `-> f32`.

Precisamos fazer um `split` em `expression` para dividir os valores a seres calculados usando como delimitador o `signal` e gravando os valores em um vetor de `&str`   dessa meneira `let values: Vec<&str> = expression.split(signal).collect();`  que terá duas posições representando os dois valores da operação aritmética.

```rust
      fn perform_operation(expression: &str, signal: char, op: impl Fn(f32, f32) -> f32) {
        let values: Vec<&str> = expression.split(signal).collect();
    }
```
em seguida vamos atribuir a uma variável `f` a implementação de `op` mencionada anteriormente que tem como parâmetros dois `f32`  como valores do vetor `values` no indice 0 e indice 1, convertidos de `string`  para `f32` usando `values[0].trim().parse::<f32>()`  o `.tim()` serve para remove espaços, possibilitando a realização de operações aritméticas.

```rust
     fn perform_operation(expression: &str, signal: char, op: impl Fn(f32, f32) -> f32) {
        let values: Vec<&str> = expression.split(signal).collect();
        let f = op(
            values[0].trim().parse::<f32>().unwrap(),
            values[1].trim().parse::<f32>().unwrap(),
     );
     println!("The result is ->: {}", f);
    }
```
voltando para a função `main` onde paramos dentro do `if` vamos fazer uso bem básico do `pattern match` para controle de fluxo, que nos permite comparar um valor com uma série de padrões e executar um código com base no padrão que casar vamos usar para compara o `signal`  vamos precisamos então criar varios braços para o `match` com os sinais aritimeticos e em cada braço chamamos a função `perform_operation` mencionada anteriormente.

```rust
match signal {
                '-' => perform_operation(&operation, signal, |x, y| x - y),
                '+' => perform_operation(&operation, signal, |x, y| x + y),
                '*' => perform_operation(&operation, signal, |x, y| x * y),
                '/' => perform_operation(&operation, signal, |x, y| x / y),
                _ => todo!(),
            };
```
 passamos como parâmetro  para função `perform_operation` uma referencia a `&operation`, `signal` e depois vamos usar `closures` para implementar o próximo parâmetro que é a função `op` que recebe dois `f32` e retorna um `f32`, esse `|x, y| x - y` é chamado de closures,  cada closure que você escreve no programa tem seu proprio tipo, então se você tiver dois `|x, y| x + y` no programa, cada um vai ter um tipo diferente, um código que recebe closure como parâmetro tem que ser genérico, pra receber closures de cada tipo.

Podemos roda o programa para ver a saída com `cargo run`.

```
➜  simple_calculator $ cargo run
   Compiling simple_calculator v0.1.0 (/srv/Learning-Rust/Rust Basic/simple_calculator)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/simple_calculator`
============================= Simples calculator Example =====================================
Your operations need to follow the pattern as in the example: '5-2', '5+2', '5*2', '5/2'
Enter your operation: 
23+9
The result is ->: 32
```